use apfs::{APFS, APFSObject, Oid, Paddr, StorageType};

use std::env;

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
        println!("Checkpoint descriptor object: {:#?}", object);
    }
    for idx in 0..superblock.body.xp_data_blocks {
        let object = apfs.load_object_addr(Paddr(superblock.body.xp_data_base.0+idx as i64));//.unwrap();
        println!("Checkpoint data object: {:#?}", object);
    }
    let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = apfs.load_btree(omap.body.tree_oid, StorageType::Physical);
    if btree_result.is_err() {
        println!("Error: {:?}", btree_result.as_ref().err());
    }
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    println!("Superblock Object Map B-Tree: {:#?}", btree);
    for record in btree.root.records {
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
        let btree_result = apfs.load_btree(omap.body.tree_oid, StorageType::Physical);
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let btree = btree_result.unwrap();
        println!("Volume Object Map B-Tree: {:#?}", btree);
        for record in btree.root.records {
            // let object = apfs.load_object_addr(record.value.paddr).unwrap();
            let root_tree_result = apfs.load_btree(Oid(record.value.paddr.0 as u64), StorageType::Physical);
            if root_tree_result.is_err() {
                println!("Error: {:?}", root_tree_result.as_ref().err());
            }
            assert!(root_tree_result.is_ok(), "Bad b-tree load");
            let root_tree = root_tree_result.unwrap();
            println!("Volume Root B-Tree: {:#?}", root_tree);
        }

        // let btree_result = apfs.load_btree(volume.body.root_tree_oid, StorageType::Physical);
    }
}
