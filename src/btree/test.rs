use std::fs::File;

use super::*;

mod omap_key {
    use super::*;

        const KEY1: OmapKey = OmapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        const KEY2: OmapKey = OmapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        const KEY_OID_LESS: OmapKey = OmapKey {
            oid: Oid(21),
            xid: Xid(17),
        };
        const KEY_OID_GREATER: OmapKey = OmapKey {
            oid: Oid(25),
            xid: Xid(17),
        };
        const KEY_XID_LESS: OmapKey = OmapKey {
            oid: Oid(23),
            xid: Xid(16),
        };
        const KEY_XID_GREATER: OmapKey = OmapKey {
            oid: Oid(23),
            xid: Xid(18),
        };
        const KEY_OID_LESS_XID_LESS: OmapKey = OmapKey {
            oid: Oid(21),
            xid: Xid(16),
        };
        const KEY_OID_GREATER_XID_LESS: OmapKey = OmapKey {
            oid: Oid(25),
            xid: Xid(16),
        };
        const KEY_OID_LESS_XID_GREATER: OmapKey = OmapKey {
            oid: Oid(21),
            xid: Xid(18),
        };
        const KEY_OID_GREATER_XID_GREATER: OmapKey = OmapKey {
            oid: Oid(25),
            xid: Xid(18),
        };

        #[test]
    fn test_object_map_key_ordering() {
        assert_eq!(KEY1.cmp(&KEY2), Ordering::Equal);
        assert_eq!(KEY1.cmp(&KEY_OID_LESS), Ordering::Greater);
        assert_eq!(KEY1.cmp(&KEY_OID_GREATER), Ordering::Less);
        assert_eq!(KEY1.cmp(&KEY_XID_LESS), Ordering::Greater);
        assert_eq!(KEY1.cmp(&KEY_XID_GREATER), Ordering::Less);
        assert_eq!(KEY1.cmp(&KEY_OID_LESS_XID_LESS), Ordering::Greater);
        assert_eq!(KEY1.cmp(&KEY_OID_LESS_XID_GREATER), Ordering::Greater);
        assert_eq!(KEY1.cmp(&KEY_OID_GREATER_XID_LESS), Ordering::Less);
        assert_eq!(KEY1.cmp(&KEY_OID_GREATER_XID_GREATER), Ordering::Less);
    }

    #[test]
    fn test_object_map_key_equal() {
        assert_eq!(KEY1, KEY1);
        assert_eq!(KEY1, KEY2);
        assert_ne!(KEY1, KEY_OID_LESS);
        assert_ne!(KEY1, KEY_OID_GREATER);
        assert_ne!(KEY1, KEY_XID_LESS);
        assert_ne!(KEY1, KEY_XID_GREATER);
        assert_ne!(KEY1, KEY_OID_LESS_XID_LESS);
        assert_ne!(KEY1, KEY_OID_LESS_XID_GREATER);
        assert_ne!(KEY1, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY1, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY2, KEY2);
        assert_ne!(KEY2, KEY_OID_LESS);
        assert_ne!(KEY2, KEY_OID_GREATER);
        assert_ne!(KEY2, KEY_XID_LESS);
        assert_ne!(KEY2, KEY_XID_GREATER);
        assert_ne!(KEY2, KEY_OID_LESS_XID_LESS);
        assert_ne!(KEY2, KEY_OID_LESS_XID_GREATER);
        assert_ne!(KEY2, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY2, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY_OID_LESS, KEY_OID_LESS);
        assert_ne!(KEY_OID_LESS, KEY_OID_GREATER);
        assert_ne!(KEY_OID_LESS, KEY_XID_LESS);
        assert_ne!(KEY_OID_LESS, KEY_XID_GREATER);
        assert_ne!(KEY_OID_LESS, KEY_OID_LESS_XID_LESS);
        assert_ne!(KEY_OID_LESS, KEY_OID_LESS_XID_GREATER);
        assert_ne!(KEY_OID_LESS, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY_OID_LESS, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY_OID_GREATER, KEY_OID_GREATER);
        assert_ne!(KEY_OID_GREATER, KEY_XID_LESS);
        assert_ne!(KEY_OID_GREATER, KEY_XID_GREATER);
        assert_ne!(KEY_OID_GREATER, KEY_OID_LESS_XID_LESS);
        assert_ne!(KEY_OID_GREATER, KEY_OID_LESS_XID_GREATER);
        assert_ne!(KEY_OID_GREATER, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY_OID_GREATER, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY_XID_LESS, KEY_XID_LESS);
        assert_ne!(KEY_XID_LESS, KEY_XID_GREATER);
        assert_ne!(KEY_XID_LESS, KEY_OID_LESS_XID_LESS);
        assert_ne!(KEY_XID_LESS, KEY_OID_LESS_XID_GREATER);
        assert_ne!(KEY_XID_LESS, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY_XID_LESS, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY_XID_GREATER, KEY_XID_GREATER);
        assert_ne!(KEY_XID_GREATER, KEY_OID_LESS_XID_LESS);
        assert_ne!(KEY_XID_GREATER, KEY_OID_LESS_XID_GREATER);
        assert_ne!(KEY_XID_GREATER, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY_XID_GREATER, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY_OID_LESS_XID_LESS, KEY_OID_LESS_XID_LESS);
        assert_ne!(KEY_OID_LESS_XID_LESS, KEY_OID_LESS_XID_GREATER);
        assert_ne!(KEY_OID_LESS_XID_LESS, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY_OID_LESS_XID_LESS, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY_OID_LESS_XID_GREATER, KEY_OID_LESS_XID_GREATER);
        assert_ne!(KEY_OID_LESS_XID_GREATER, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY_OID_LESS_XID_GREATER, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY_OID_GREATER_XID_LESS, KEY_OID_GREATER_XID_LESS);
        assert_ne!(KEY_OID_GREATER_XID_LESS, KEY_OID_GREATER_XID_GREATER);
        assert_eq!(KEY_OID_GREATER_XID_GREATER, KEY_OID_GREATER_XID_GREATER);
    }

    #[test]
    fn test_object_map_key_matching() {
        assert_eq!(KEY1.r#match(&KEY2), Ordering::Equal);
        assert_eq!(KEY1.r#match(&KEY_OID_LESS), Ordering::Greater);
        assert_eq!(KEY1.r#match(&KEY_OID_GREATER), Ordering::Less);
        /* Matching keys have same Oid and and Xid less than or equal */
        assert_eq!(KEY1.r#match(&KEY_XID_LESS), Ordering::Equal);
        assert_eq!(KEY1.r#match(&KEY_XID_GREATER), Ordering::Less);
        assert_eq!(KEY1.r#match(&KEY_OID_LESS_XID_LESS), Ordering::Greater);
        assert_eq!(KEY1.r#match(&KEY_OID_LESS_XID_GREATER), Ordering::Greater);
        assert_eq!(KEY1.r#match(&KEY_OID_GREATER_XID_LESS), Ordering::Less);
        assert_eq!(KEY1.r#match(&KEY_OID_GREATER_XID_GREATER), Ordering::Less);
    }
}

#[test]
fn test_volume_object_key_ordering() {
    let key1 = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 500), },
        subkey: ApfsSubKey::Name("middle".to_string()),
    };
    let key2 = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 500), },
        subkey: ApfsSubKey::Name("middle".to_string()),
    };
    let key_object_id_less = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 300), },
        subkey: ApfsSubKey::Name("middle".to_string()),
    };
    let key_object_id_greater = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 650), },
        subkey: ApfsSubKey::Name("middle".to_string()),
    };
    let key_object_type_less = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::Inode, 500), },
        subkey: ApfsSubKey::None,
    };
    let key_object_type_greater = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::SiblingMap, 500), },
        subkey: ApfsSubKey::None,
    };
    let key_name_less = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 500), },
        subkey: ApfsSubKey::Name("alpha".to_string()),
    };
    let key_name_greater = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 500), },
        subkey: ApfsSubKey::Name("zulu".to_string()),
    };
    let key_object_id_less_name_less = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 271), },
        subkey: ApfsSubKey::Name("alpha".to_string()),
    };
    let key_object_id_greater_name_less = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 749), },
        subkey: ApfsSubKey::Name("alpha".to_string()),
    };
    let key_object_id_less_name_greater = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 271), },
        subkey: ApfsSubKey::Name("zulu".to_string()),
    };
    let key_object_id_greater_name_greater = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::DirRec, 749), },
        subkey: ApfsSubKey::Name("zulu".to_string()),
    };
    let key_object_id_less_type_less = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::Inode, 271), },
        subkey: ApfsSubKey::None,
    };
    let key_object_id_greater_type_less = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::Inode, 749), },
        subkey: ApfsSubKey::None,
    };
    let key_object_id_less_type_greater = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::SiblingMap, 271), },
        subkey: ApfsSubKey::None,
    };
    let key_object_id_greater_type_greater = ApfsKey {
        key: JKey { obj_id_and_type: JObjectIdAndType::new_by_field(JObjTypes::SiblingMap, 749), },
        subkey: ApfsSubKey::None,
    };
    assert_eq!(key1.cmp(&key2), Ordering::Equal);
    assert_eq!(key1.cmp(&key_object_id_less), Ordering::Greater);
    assert_eq!(key1.cmp(&key_object_id_greater), Ordering::Less);
    assert_eq!(key1.cmp(&key_object_type_less), Ordering::Greater);
    assert_eq!(key1.cmp(&key_object_type_greater), Ordering::Less);
    assert_eq!(key1.cmp(&key_name_less), Ordering::Greater);
    assert_eq!(key1.cmp(&key_name_greater), Ordering::Less);
    assert_eq!(key1.cmp(&key_object_id_less_name_less), Ordering::Greater);
    assert_eq!(key1.cmp(&key_object_id_less_name_greater), Ordering::Greater);
    assert_eq!(key1.cmp(&key_object_id_greater_name_less), Ordering::Less);
    assert_eq!(key1.cmp(&key_object_id_greater_name_greater), Ordering::Less);
    assert_eq!(key1.cmp(&key_object_id_less_type_less), Ordering::Greater);
    assert_eq!(key1.cmp(&key_object_id_less_type_greater), Ordering::Greater);
    assert_eq!(key1.cmp(&key_object_id_greater_type_less), Ordering::Less);
    assert_eq!(key1.cmp(&key_object_id_greater_type_greater), Ordering::Less);
}

use crate::{tests::{test_dir, load_test_apfs_superblock}, JObjectIdAndType};

#[test]
fn test_load_object_map() {
    let (mut apfs, superblock) = load_test_apfs_superblock();
    let object_result = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical);
    assert!(object_result.is_ok(), "Bad object map load");
    let object = object_result.unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = apfs.load_object_oid(omap.body.tree_oid, StorageType::Physical);
    //assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    let node = match btree {
        APFSObject::Btree(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
}

#[test]
fn test_load_object_map_btree() {
    let (mut apfs, superblock) = load_test_apfs_superblock();
    let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, omap.body.tree_oid, StorageType::Physical);
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    let records: Vec<OmapRecord> = match btree.root.records {
        AnyRecords::Leaf(x) => x,
        _ => { panic!("Wrong b-tree record type!"); },
    };
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].key.oid, Oid(1026));
    assert_eq!(records[0].key.xid, Xid(4));
    assert!(records[0].value.flags.is_empty());
    assert_eq!(records[0].value.size, 4096);
    assert_eq!(records[0].value.paddr, Paddr(102));
}

#[test]
fn test_load_object_map_btree_dummy() {
    let mut source = File::open(test_dir().join("btree.blob")).expect("Unable to load blob");
    let mut apfs = APFS { source, block_size: 4096 };
    let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, Oid(0), StorageType::Physical);
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    let records: Vec<OmapRecord> = match btree.root.records {
        AnyRecords::Leaf(x) => x,
        _ => { panic!("Wrong b-tree record type!"); },
    };
    assert_eq!(records.len(), 6);
    assert_eq!(records[0].key.oid, Oid(0x0586), "key 0 oid");
    assert_eq!(records[0].key.xid, Xid(0x2000), "key 0 xid");
    assert_eq!(records[1].key.oid, Oid(0x0588), "key 1 oid");
    assert_eq!(records[1].key.xid, Xid(0x2101), "key 1 xid");
    assert_eq!(records[2].key.oid, Oid(0x0588), "key 2 oid");
    assert_eq!(records[2].key.xid, Xid(0x2202), "key 2 xid");
    assert_eq!(records[3].key.oid, Oid(0x0588), "key 3 oid");
    assert_eq!(records[3].key.xid, Xid(0x2300), "key 3 xid");
    assert_eq!(records[4].key.oid, Oid(0x0589), "key 4 oid");
    assert_eq!(records[4].key.xid, Xid(0x1000), "key 4 xid");
    assert_eq!(records[5].key.oid, Oid(0x0589), "key 5 oid");
    assert_eq!(records[5].key.xid, Xid(0x2000), "key 5 xid");
    assert_eq!(records[0].value.flags, OvFlags::empty(), "value 0 flags");
    assert_eq!(records[0].value.size, 4096,              "value 0 size");
    assert_eq!(records[0].value.paddr, Paddr(0x400),     "value 0 paddr");
    assert_eq!(records[1].value.flags, OvFlags::empty(), "value 1 flags");
    assert_eq!(records[1].value.size, 4096,              "value 1 size");
    assert_eq!(records[1].value.paddr, Paddr(0x200),     "value 1 paddr");
    assert_eq!(records[2].value.flags, OvFlags::empty(), "value 2 flags");
    assert_eq!(records[2].value.size, 4096,              "value 2 size");
    assert_eq!(records[2].value.paddr, Paddr(0x300),     "value 2 paddr");
    assert_eq!(records[3].value.flags, OvFlags::empty(), "value 3 flags");
    assert_eq!(records[3].value.size, 4096,              "value 3 size");
    assert_eq!(records[3].value.paddr, Paddr(0x100),     "value 3 paddr");
    assert_eq!(records[4].value.flags, OvFlags::empty(), "value 4 flags");
    assert_eq!(records[4].value.size, 4096,              "value 4 size");
    assert_eq!(records[4].value.paddr, Paddr(0x500),     "value 4 paddr");
    assert_eq!(records[5].value.flags, OvFlags::empty(), "value 5 flags");
    assert_eq!(records[5].value.size, 4096,              "value 5 size");
    assert_eq!(records[5].value.paddr, Paddr(0x600),     "value 5 paddr");
}

#[test]
fn test_load_non_leaf_object_map_btree() {
    let mut source = File::open(test_dir().join("object-map-root-nonleaf.blob")).expect("Unable to load blob");
    let mut apfs = APFS { source, block_size: 4096 };
    let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, Oid(0), StorageType::Physical);
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    let records = match btree.root.records {
        AnyRecords::NonLeaf(x, _) => x,
        _ => { panic!("Wrong b-tree record type!"); },
    };
    assert_eq!(records.len(), 85);
    assert_eq!(records[0].key.oid, Oid(0x404), "key oid");
    assert_eq!(records[0].key.xid, Xid(0x95d8c3), "key xid");
    assert_eq!(records[0].value.oid, Oid(0x107ab1), "value oid");
    assert_eq!(records[1].key.oid, Oid(0x2eda), "key oid");
    assert_eq!(records[1].key.xid, Xid(0x6), "key xid");
    assert_eq!(records[1].value.oid, Oid(0x148050), "value oid");
    assert_eq!(records[2].key.oid, Oid(0x5807), "key oid");
    assert_eq!(records[2].key.xid, Xid(0x8de0ea), "key xid");
    assert_eq!(records[2].value.oid, Oid(0x1447ea), "value oid");
}

#[test]
fn test_load_non_root_object_map_btree() {
    let mut source = File::open(test_dir().join("object-map-root-nonleaf.blob")).expect("Unable to load blob");
    let mut apfs = APFS { source, block_size: 4096 };
    let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, Oid(0), StorageType::Physical);
    let btree = btree_result.unwrap();
    let mut source = File::open(test_dir().join("object-map-nonroot-nonleaf.blob")).expect("Unable to load blob");
    let mut apfs = APFS { source, block_size: 4096 };
    let node_result = btree.load_btree_node(&mut apfs, Oid(0), StorageType::Physical);
    if node_result.is_err() {
        println!("Error: {:?}", node_result.as_ref().err());
    }
    assert!(node_result.is_ok(), "Bad b-tree node load");
    let node = node_result.unwrap();
    let records = match node.records {
        AnyRecords::NonLeaf(x, _) => x,
        _ => { panic!("Wrong b-tree record type!"); },
    };
    assert_eq!(records.len(), 123);
    assert_eq!(records[0].key.oid, Oid(0x404), "key oid");
    assert_eq!(records[0].key.xid, Xid(0x95d8c3), "key xid");
    assert_eq!(records[0].value.oid, Oid(0x107cfc), "value oid");
    assert_eq!(records[1].key.oid, Oid(0x440), "key oid");
    assert_eq!(records[1].key.xid, Xid(0xb93e), "key xid");
    assert_eq!(records[1].value.oid, Oid(0x12c32f), "value oid");
    assert_eq!(records[2].key.oid, Oid(0x4a0), "key oid");
    assert_eq!(records[2].key.xid, Xid(0xb93e), "key xid");
    assert_eq!(records[2].value.oid, Oid(0x14bff0), "value oid");
}

#[test]
fn test_load_volume_superblock() {
    let (mut apfs, superblock) = load_test_apfs_superblock();
    let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, omap.body.tree_oid, StorageType::Physical);
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    assert_ne!(superblock.body.fs_oid[0], Oid(0));
    let mut found = -1;
    let records: Vec<OmapRecord> = match btree.root.records {
        AnyRecords::Leaf(x) => x,
        _ => { panic!("Wrong b-tree record type!"); },
    };
    for idx in 0..records.len() {
        if records[idx].key.oid == superblock.body.fs_oid[0] {
            found = idx as isize;
            break;
        }
    }
    assert!(found >= 0);
    let object = apfs.load_object_addr(records[found as usize].value.paddr).unwrap();
    let volume = match object {
        APFSObject::ApfsSuperblock(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    assert_eq!(volume.body.volname[0..7], *b"MYAPFS\0");
}

#[test]
fn can_get_matching_record_from_leaf_node() {
    // BtreeNode {
    //     node: BtreeNodeObject { header: Ph, body: () }
    // }
    let (mut apfs, superblock) = load_test_apfs_superblock();
    let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, omap.body.tree_oid, StorageType::Physical);
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    let any_record = btree.root.get_record(OmapKey::new(1026, 4));
    assert!(any_record.is_some());
    let any_record = any_record.unwrap();
    let record = match any_record {
        // AnyRecord::NonLeaf(x, _) => x,
        // _ => { panic!("Expected a non-leaf node"); },
        AnyRecord::Leaf(x) => x,
        _ => { panic!("Expected a leaf node"); },
    };
    assert_eq!(record.key.oid, Oid(1026));
    assert_eq!(record.key.xid, Xid(4));
    assert!(record.value.flags.is_empty());
    assert_eq!(record.value.size, 4096);
    assert_eq!(record.value.paddr, Paddr(102));
}


#[test]
fn no_record_returned_on_bad_match_from_leaf_node() {
    let (mut apfs, superblock) = load_test_apfs_superblock();
    let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, omap.body.tree_oid, StorageType::Physical);
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    let any_record = btree.root.get_record(OmapKey::new(500, 999));
    assert!(any_record.is_none());
    let any_record = btree.root.get_record(OmapKey::new(2012, 1));
    assert!(any_record.is_none());
}
