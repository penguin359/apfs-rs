use std::fs::File;

use apfs::{APFS, APFSObject, Btree, Oid, Paddr, StorageType, OvFlags, OmapKey, OmapVal, OmapRecord, ApfsValue, AnyRecords, LeafRecord, NonLeafRecord, InoExtType, InodeXdata};

use std::{env, collections::HashMap};

fn scan_children(apfs: &mut APFS<File>, btree: &Btree<OmapVal>, children: Vec<NonLeafRecord<OmapKey>>) -> Vec<LeafRecord<OmapVal>> {
    for child in children {
        let node_result = btree.load_btree_node(apfs, child.value.oid, StorageType::Physical);
        if node_result.is_err() {
            println!("Error: {:?}", node_result.as_ref().err());
        }
        assert!(node_result.is_ok(), "Bad b-tree node load");
        let node = node_result.unwrap();
        println!("Volume Object Map B-Tree: {:#?}", node);
        let _records: Vec<OmapRecord> = match node.records {
            AnyRecords::Leaf(x) => x,
            AnyRecords::NonLeaf(x, _) => scan_children(apfs, btree, x),
        };
    }
    vec![]
}

fn dump_omap_apfs_records(btree: &Btree<OmapVal>, apfs: &mut APFS<File>, records: AnyRecords<OmapVal>) {
    let records = match records {
        AnyRecords::Leaf(x) => x,
        AnyRecords::NonLeaf(children, _) => {
            for child in children {
                let node_result = btree.load_btree_node(apfs, child.value.oid, StorageType::Physical);
                if node_result.is_err() {
                    println!("Error: {:?}", node_result.as_ref().err());
                }
                assert!(node_result.is_ok(), "Bad b-tree node load");
                let node = node_result.unwrap();
                dump_omap_apfs_records(btree, apfs, node.records);
            }
            vec![]
        },
    };
    for record in records {
        if record.value.flags.contains(OvFlags::ENCRYPTED) {
            println!("Encrypted volume found, skipping...");
            continue;
        }
        // let object = apfs.load_object_addr(record.value.paddr).unwrap();
        let root_tree_result = apfs.load_btree::<ApfsValue>(Oid(record.value.paddr.0 as u64), StorageType::Physical);
        if root_tree_result.is_err() {
            println!("Error: {:?}", root_tree_result.as_ref().err());
        }
        assert!(root_tree_result.is_ok(), "Bad b-tree load");
        let mut root_tree = root_tree_result.unwrap();
        println!("Volume Root B-Tree: {:#?}", root_tree);
        let records = std::mem::replace(&mut root_tree.root.records, AnyRecords::Leaf(vec![]));
        dump_apfs_records(&root_tree, apfs, records);
    }
}

fn dump_apfs_records(btree: &Btree<ApfsValue>, apfs: &mut APFS<File>, records: AnyRecords<ApfsValue>) {
    let file_records: Vec<LeafRecord<ApfsValue>> = match records {
        AnyRecords::Leaf(x) => x,
        AnyRecords::NonLeaf(children, _) => {
            for child in children {
                // Need to support virtual object lookup
                //let node_result = btree.load_btree_node(apfs, child.value.oid, StorageType::Physical);
                //if node_result.is_err() {
                //    println!("Error: {:?}", node_result.as_ref().err());
                //}
                //assert!(node_result.is_ok(), "Bad b-tree node load");
                //let node = node_result.unwrap();
                //dump_apfs_records(btree, apfs, node.records);
            }
            vec![]
        },
    };
    let mut sizes = HashMap::<u64, u64>::new();
    for file_record in file_records {
        if let ApfsValue::Inode(y) = file_record.value {
            if let Some(&InodeXdata::Dstream(ref z)) = y.xdata.get(&InoExtType::Dstream) {
                sizes.insert(file_record.key.key.obj_id_and_type.id(), z.size);
            }
        } else if let ApfsValue::FileExtent(y) = file_record.value {
            let length = sizes[&file_record.key.key.obj_id_and_type.id()] as usize;
            println!("Reading block: {} ({} bytes)", y.phys_block_num, length);
            if let Ok(block) = apfs.load_block(Paddr(y.phys_block_num as i64)) {
                println!("Body: '{}'", String::from_utf8((&block[0..length]).to_owned()).unwrap());
            }
        }
    }
}

fn main() {
    println!("Dumping file");
    let mut apfs = APFS::open(env::args().skip(1).next().unwrap()).unwrap();
    let superblock = match apfs.load_object_addr(Paddr(0)).unwrap() {
        APFSObject::Superblock(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    println!("Container Superblock: {:#?}", superblock);
    assert!(superblock.body.xp_desc_blocks & (1 << 31) == 0);
    assert!(superblock.body.xp_data_blocks & (1 << 31) == 0);
    //for idx in 0..superblock.body.xp_desc_blocks {
    //    let object = apfs.load_object_addr(Paddr(superblock.body.xp_desc_base.0+idx as i64)).unwrap();
    //    println!("Checkpoint descriptor object: {:#?}", object);
    //}
    //for idx in 0..superblock.body.xp_data_blocks {
    //    let object = apfs.load_object_addr(Paddr(superblock.body.xp_data_base.0+idx as i64));//.unwrap();
    //    println!("Checkpoint data object: {:#?}", object);
    //}
    if superblock.body.keylocker.start_paddr.0 != 0 &&
       superblock.body.keylocker.block_count != 0 {
        println!("Found keylocker");
        println!("{:?}", apfs.load_block(superblock.body.keylocker.start_paddr));
    }
    let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = apfs.load_btree::<OmapVal>(omap.body.tree_oid, StorageType::Physical);
    if btree_result.is_err() {
        println!("Error: {:?}", btree_result.as_ref().err());
    }
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    println!("Superblock Object Map B-Tree: {:#?}", btree);
    let records: Vec<OmapRecord> = match btree.root.records {
        AnyRecords::Leaf(x) => x,
        _ => { panic!("Wrong b-tree record type!"); },
    };
    for record in records {
        let object = apfs.load_object_addr(record.value.paddr).unwrap();
        let volume = match object {
            APFSObject::ApfsSuperblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        println!("Volume Superblock: {:#?}", volume);
        let object = apfs.load_object_oid(volume.body.omap_oid, StorageType::Physical).unwrap();
        let omap = match object {
            APFSObject::ObjectMap(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        let btree_result = apfs.load_btree::<OmapVal>(omap.body.tree_oid, StorageType::Physical);
        if btree_result.is_err() {
            println!("Error: {:?}", btree_result.as_ref().err());
        }
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let mut btree = btree_result.unwrap();
        println!("Volume Object Map B-Tree: {:#?}", btree);
        let records = std::mem::replace(&mut btree.root.records, AnyRecords::Leaf(vec![]));
        dump_omap_apfs_records(&btree, &mut apfs, records);

        // let btree_result = apfs.load_btree(volume.body.root_tree_oid, StorageType::Physical);
    }
}
