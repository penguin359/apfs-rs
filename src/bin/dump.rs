use std::{fs::File, cmp::min};

use apfs::{APFS, APFSObject, Btree, Oid, Paddr, StorageType, OvFlags, OmapVal, OmapRecord, ApfsValue, AnyRecords, InoExtType, InodeXdata, OmapKey, ObjectType, SpacemanFreeQueueValue, NX_EFI_JUMPSTART_MAGIC, NX_EFI_JUMPSTART_VERSION};

use std::{env, collections::HashMap};

fn dump_omap_apfs_records(btree: &Btree<OmapVal>, apfs: &mut APFS<File>, records: &AnyRecords<OmapVal>) {
    match records {
        AnyRecords::Leaf(_) => {},
        AnyRecords::NonLeaf(children, _) => {
            for child in children {
                let node_result = btree.load_btree_node(apfs, child.value.oid, StorageType::Physical);
                if node_result.is_err() {
                    println!("Error: {:?}", node_result.as_ref().err());
                }
                assert!(node_result.is_ok(), "Bad b-tree node load");
                let node = node_result.unwrap();
                println!("Volume Object Map sub B-Tree: {:#?}", node);
                dump_omap_apfs_records(btree, apfs, &node.records);
            }
        },
    };
}

fn dump_apfs_records(btree: &Btree<ApfsValue>, apfs: &mut APFS<File>, omap_btree: &Btree<OmapVal>, records: &AnyRecords<ApfsValue>) {
    let empty = vec![];
    let file_records = match records {
        AnyRecords::Leaf(ref x) => x,
        AnyRecords::NonLeaf(children, _) => {
            for child in children {
                let root_object = omap_btree.get_record(apfs, &OmapKey::new(child.value.oid.0, u64::MAX))
                    .expect("I/O error")
                    .expect("Failed to find address for Volume root B-tree");
                let node_result = btree.load_btree_node(apfs, Oid(root_object.value.paddr.0 as u64), StorageType::Physical);
                let node = node_result.expect("Bad b-tree node load");
                println!("Volume Root sub B-Tree: {:#?}", &node);
                dump_apfs_records(btree, apfs, omap_btree, &node.records);
            }
            &empty
        },
    };
    let mut sizes = HashMap::<u64, u64>::new();
    for file_record in file_records {
        if let ApfsValue::Inode(ref y) = file_record.value {
            if let Some(&InodeXdata::Dstream(ref z)) = y.xdata.get(&InoExtType::Dstream) {
                sizes.insert(file_record.key.key.obj_id_and_type.id(), z.size);
            }
        } else if let ApfsValue::FileExtent(ref y) = file_record.value {
            if let Some(&length) = sizes.get(&file_record.key.key.obj_id_and_type.id()) {
                println!("Reading block: {} ({} bytes)", y.phys_block_num, length);
                if let Ok(block) = apfs.load_block(Paddr(y.phys_block_num as i64)) {
                    println!("Body: '{}'", String::from_utf8((&block[0..min(length as usize, block.len())]).to_owned()).unwrap_or_else(|_| String::from("(binary)")));
                }
            } else {
                println!("Missing inode for file!");
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
    for idx in 0..superblock.body.xp_desc_blocks {
        let object = apfs.load_object_addr(Paddr(superblock.body.xp_desc_base.0+idx as i64)).unwrap();
        println!("Checkpoint descriptor object: {:#?}", &object);
    }
    for idx in 0..superblock.body.xp_data_blocks {
        let object = apfs.load_object_addr(Paddr(superblock.body.xp_data_base.0+idx as i64));//.unwrap();
        println!("Checkpoint data object: {:#?}", &object);
        if let Ok(APFSObject::Spaceman(body)) = object {
            let subobject_result = apfs.load_object_addr(body.body.ip_base);
            if let Ok(subobject) = subobject_result {
                println!("Internal pool data object: {:#?}", &subobject);
            } else {
                println!("Error reading pool data: {:#?}", subobject_result);
            }
            // for subidx in 0..body.body.ip_block_count {
            //     let subobject = apfs.load_object_addr(Paddr(body.body.ip_base.0 + subidx as i64)).unwrap();
            //     println!("Internal pool data object: {:#?}", &subobject);
            // }
            // let subobject = apfs.load_object_addr(body.body.ip_bm_base).unwrap();
            // println!("Internal pool bitmap data object: {:#?}", &subobject);
        } else if let Ok(APFSObject::Btree(body)) = object {
            if body.header.subtype.r#type() == ObjectType::SpacemanFreeQueue {
                let btree = apfs.load_btree::<SpacemanFreeQueueValue>(Oid(superblock.body.xp_data_base.0 as u64 + idx as u64), StorageType::Physical)
                    .expect("Bad b-tree load");
                println!("Space Manager Free Queue B-Tree: {:#?}", btree);
                match &btree.root.records {
                    AnyRecords::Leaf(ref x) => {
                        for record in x {
                            let subobject_result = apfs.load_object_addr(record.key.paddr);
                            if let Ok(subobject) = subobject_result {
                                println!("SFQ Internal pool data object: {:#?}", &subobject);
                            } else {
                                println!("SFQ Error reading pool data: {:#?}", subobject_result);
                            }
                        }
                    },
                    AnyRecords::NonLeaf(_, _) => {},
                }
            }
        }
    }
    if superblock.body.efi_jumpstart != Paddr(0) {
        println!("Dumping Bootloader");
        let object = apfs.load_object_addr(superblock.body.efi_jumpstart).unwrap();
        let jumpstart = match object {
            APFSObject::EfiJumpstart(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        println!("EFI Jumpstart: {:#?}", &jumpstart);
        assert_eq!(jumpstart.body.magic, NX_EFI_JUMPSTART_MAGIC);
        assert_eq!(jumpstart.body.version, NX_EFI_JUMPSTART_VERSION);
        let mut loader = vec![];
        for range in jumpstart.body.rec_extents {
            for idx in 0..range.block_count {
                loader.push(apfs.load_block(Paddr(range.start_paddr.0 + idx as i64)).expect("Failed to load jumpstart block"));
            }
        }
        let mut loader_bytes = loader.into_iter().flatten().collect::<Vec<u8>>();
        loader_bytes.shrink_to(jumpstart.body.efi_file_len as usize);
        println!("Bootloader: {:?}", loader_bytes);
    }
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
    let records: &Vec<OmapRecord> = match &btree.root.records {
        AnyRecords::Leaf(ref x) => x,
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
        let btree = apfs.load_btree::<OmapVal>(omap.body.tree_oid, StorageType::Physical)
            .expect("Bad b-tree load");
        println!("Volume Object Map B-Tree: {:#?}", &btree);
        dump_omap_apfs_records(&btree, &mut apfs, &btree.root.records);
        let root_object = btree.get_record(&mut apfs, &OmapKey::new(volume.body.root_tree_oid.0, u64::MAX))
            .expect("I/O error")
            .expect("Failed to find address for Volume root B-tree");
        if root_object.value.flags.contains(OvFlags::ENCRYPTED) {
            println!("Encrypted volume found, skipping...");
            continue;
        }
        let root_btree = apfs.load_btree::<ApfsValue>(Oid(root_object.value.paddr.0 as u64), StorageType::Physical)
            .expect("Failed to load volume root B-tree");
        println!("Volume Root B-Tree: {:#?}", &root_btree);
        dump_apfs_records(&root_btree, &mut apfs, &btree, &root_btree.root.records);

        // let btree_result = apfs.load_btree(volume.body.root_tree_oid, StorageType::Physical);
    }
}
