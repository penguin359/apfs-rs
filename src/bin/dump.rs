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
    let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = apfs.load_btree(omap.body.tree_oid, StorageType::Physical);
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    println!("Superblock Object Map B-Tree: {:#?}", btree);
    for record in btree.records {
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
        for record in btree.records {
            let object = apfs.load_object_addr(record.value.paddr).unwrap();
            // let root_tree_result = apfs.load_btree(Oid(record.value.paddr.0 as u64), StorageType::Physical);
            // assert!(root_tree_result.is_ok(), "Bad b-tree load");
            // let root_tree = root_tree_result.unwrap();
            // println!("Volume Root B-Tree: {:#?}", root_tree);
        }

        // let btree_result = apfs.load_btree(volume.body.root_tree_oid, StorageType::Physical);
    }
}
