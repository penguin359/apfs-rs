use std::fs::File;

use super::*;

struct DummySource {
    position: u64,
    block_size: u64,
    blocks: HashMap<u64, Vec<u8>>,
}

impl Read for DummySource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.blocks.get(&(self.position/self.block_size)) {
            Some(data) => {
                buf.copy_from_slice(data);
                self.position += buf.len() as u64;
                Ok(buf.len())
            },
            None => {
                buf.fill(0);
                self.position += buf.len() as u64;
                Ok(buf.len())
            },
        }
    }
}

impl Seek for DummySource {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.position = match pos {
            io::SeekFrom::Start(offset) => offset,
            io::SeekFrom::End(offset) => 0,
            io::SeekFrom::Current(offset) => offset as u64 + self.position,
        };
        Ok(self.position)
    }
}

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

mod apfs_key {
    use super::*;

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
}

use crate::{tests::{test_dir, load_test_apfs_superblock, TEST_APFS_FILE, TEST_16KB_APFS_FILE}, JObjectIdAndType, ObjectMapObject, NxSuperblockObject, BtreeInfoFixed, BtFlags, ObjPhys, ObjectTypeAndFlags, ObjTypeFlags};

fn load_test_apfs_object_map(file: &str) -> (APFS<File>, NxSuperblockObject, ObjectMapObject) {
    let (mut apfs, superblock) = load_test_apfs_superblock(file);
    let object_result = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical);
    assert!(object_result.is_ok(), "Bad object map load");
    let object = object_result.unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    (apfs, superblock, omap)
}

fn load_test_apfs_object_map_btree(file: &str) -> (APFS<File>, NxSuperblockObject, ObjectMapObject, Btree<OmapVal>) {
    let (mut apfs, superblock, omap) = load_test_apfs_object_map(file);
    let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, omap.body.tree_oid, StorageType::Physical);
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    (apfs, superblock, omap, btree)
}

#[test]
fn test_load_object_map() {
    let (_, _, omap) = load_test_apfs_object_map(TEST_APFS_FILE);
}

#[test]
fn test_load_object_map_btree() {
    let (_, _, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
    let records = match &btree.root.records {
        AnyRecords::Leaf(ref x) => x,
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
    let records = match &btree.root.records {
        AnyRecords::Leaf(ref x) => x,
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

mod object_map {
    use super::*;

    const OBJECT_MAP_ROOT_FILE: &str = "object-map-root-nonleaf.blob";
    const OBJECT_MAP_NONLEAF_FILE: &str = "object-map-nonroot-nonleaf.blob";
    const OBJECT_MAP_LEAF_FILE: &str = "object-map-nonroot-leaf.blob";

    fn load_root_object_map() -> Btree<OmapVal> {
        let mut source = File::open(test_dir().join(OBJECT_MAP_ROOT_FILE)).expect("Unable to load blob");
        let mut apfs = APFS { source, block_size: 4096 };
        let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, Oid(0), StorageType::Physical);
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let btree = btree_result.unwrap();
        btree
    }

    fn load_nonroot_object_map(file: &str) -> BtreeNode<OmapVal> {
        let btree = load_root_object_map();
        let mut source = File::open(test_dir().join(file)).expect("Unable to load blob");
        let mut apfs = APFS { source, block_size: 4096 };
        let node_result = btree.load_btree_node(&mut apfs, Oid(0), StorageType::Physical);
        if node_result.is_err() {
            println!("Error: {:?}", node_result.as_ref().err());
        }
        assert!(node_result.is_ok(), "Bad b-tree node load");
        let node = node_result.unwrap();
        node
     }

    #[test]
    fn can_load_root_nonleaf_object_map_btree() {
        let btree = load_root_object_map();
        let records = match &btree.root.records {
            AnyRecords::NonLeaf(ref x, _) => x,
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
    fn can_load_nonroot_nonleaf_object_map_btree() {
        let node = load_nonroot_object_map(OBJECT_MAP_NONLEAF_FILE);
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
    fn can_get_exact_matching_record_from_nonleaf_node() {
        let node = load_nonroot_object_map(OBJECT_MAP_NONLEAF_FILE);
        check_omap_nonleaf_record_lookup_node(&node, 0x404, 0x95d8c3, Oid(0x404), Xid(0x95d8c3), Oid(0x107cfc));
        check_omap_nonleaf_record_lookup_node(&node, 0x440, 0xb93e, Oid(0x440), Xid(0xb93e), Oid(0x12c32f));
        check_omap_nonleaf_record_lookup_node(&node, 0x4a0, 0xb93e, Oid(0x4a0), Xid(0xb93e), Oid(0x14bff0));
        check_omap_nonleaf_record_lookup_node(&node, 0x2e78, 0x8e3e0c, Oid(0x2e78), Xid(0x8e3e0c), Oid(0x14e09d));
    }

    #[test]
    fn no_record_returned_on_bad_exact_match_from_nonleaf_node() {
        let node = load_nonroot_object_map(OBJECT_MAP_NONLEAF_FILE);
        check_omap_record_lookup_missing_node(&node, 0x403, 0x95d8c3);
        check_omap_record_lookup_missing_node(&node, 0x403, 0xb93e);
        check_omap_record_lookup_missing_node(&node, 0x403, u64::MAX);
        check_omap_record_lookup_missing_node(&node, 0x0, 0x95d8c3);
        check_omap_record_lookup_missing_node(&node, 0x0, u64::MAX);
    }

    #[test]
    fn can_get_inexact_matching_record_from_nonleaf_node() {
        let node = load_nonroot_object_map(OBJECT_MAP_NONLEAF_FILE);
        check_omap_nonleaf_record_lookup_node(&node, 0x404, 0x95d8c4, Oid(0x404), Xid(0x95d8c3), Oid(0x107cfc));
        check_omap_nonleaf_record_lookup_node(&node, 0x404, u64::MAX, Oid(0x404), Xid(0x95d8c3), Oid(0x107cfc));
        check_omap_nonleaf_record_lookup_node(&node, 0x405, 0, Oid(0x404), Xid(0x95d8c3), Oid(0x107cfc));
        check_omap_nonleaf_record_lookup_node(&node, 0x439, u64::MAX, Oid(0x404), Xid(0x95d8c3), Oid(0x107cfc));
        check_omap_nonleaf_record_lookup_node(&node, 0x440, 0xb93d, Oid(0x404), Xid(0x95d8c3), Oid(0x107cfc));

        check_omap_nonleaf_record_lookup_node(&node, 0x440, 0xb93f, Oid(0x440), Xid(0xb93e), Oid(0x12c32f));
        check_omap_nonleaf_record_lookup_node(&node, 0x440, u64::MAX, Oid(0x440), Xid(0xb93e), Oid(0x12c32f));
        check_omap_nonleaf_record_lookup_node(&node, 0x4a0, 0xb93d, Oid(0x440), Xid(0xb93e), Oid(0x12c32f));

        check_omap_nonleaf_record_lookup_node(&node, 0x4a0, 0xb93f, Oid(0x4a0), Xid(0xb93e), Oid(0x14bff0));
        check_omap_nonleaf_record_lookup_node(&node, 0x4ea, 0x95d8c1, Oid(0x4a0), Xid(0xb93e), Oid(0x14bff0));

        check_omap_nonleaf_record_lookup_node(&node, 0x2e78, 0x8e3e0d, Oid(0x2e78), Xid(0x8e3e0c), Oid(0x14e09d));
        check_omap_nonleaf_record_lookup_node(&node, 0x2e78, u64::MAX, Oid(0x2e78), Xid(0x8e3e0c), Oid(0x14e09d));
        check_omap_nonleaf_record_lookup_node(&node, 0x2e79, 0, Oid(0x2e78), Xid(0x8e3e0c), Oid(0x14e09d));
        check_omap_nonleaf_record_lookup_node(&node, u64::MAX, 0, Oid(0x2e78), Xid(0x8e3e0c), Oid(0x14e09d));
        check_omap_nonleaf_record_lookup_node(&node, u64::MAX, u64::MAX, Oid(0x2e78), Xid(0x8e3e0c), Oid(0x14e09d));
    }

    #[test]
    fn no_record_returned_on_bad_inexact_match_from_nonleaf_node() {
        let node = load_nonroot_object_map(OBJECT_MAP_NONLEAF_FILE);
        check_omap_record_lookup_missing_node(&node, 0x404, 0x95d8c2);
        check_omap_record_lookup_missing_node(&node, 0x404, 0x123456);
        check_omap_record_lookup_missing_node(&node, 0x404, 0x1000);
        check_omap_record_lookup_missing_node(&node, 0x404, 0);
    }

    #[test]
    fn can_load_nonroot_leaf_object_map_btree() {
        let node = load_nonroot_object_map(OBJECT_MAP_LEAF_FILE);
        let records = match node.records {
            AnyRecords::Leaf(x) => x,
            _ => { panic!("Wrong b-tree record type!"); },
        };
        assert_eq!(records.len(), 104);
        assert_eq!(records[23].key.oid, Oid(0x404),           "key 23 oid");
        assert_eq!(records[23].key.xid, Xid(9829294),         "key 23 xid");
        assert_eq!(records[23].value.flags, OvFlags::empty(), "value 23 flags");
        assert_eq!(records[23].value.size, 4096,              "value 23 size");
        assert_eq!(records[23].value.paddr, Paddr(1284313),   "value 23 paddr");
        assert_eq!(records[24].key.oid, Oid(0x404),           "key 24 oid");
        assert_eq!(records[24].key.xid, Xid(9829474),         "key 24 xid");
        assert_eq!(records[24].value.flags, OvFlags::empty(), "value 24 flags");
        assert_eq!(records[24].value.size, 4096,              "value 24 size");
        assert_eq!(records[24].value.paddr, Paddr(1077411),   "value 24 paddr");
        assert_eq!(records[25].key.oid, Oid(0x408),           "key 25 oid");
        assert_eq!(records[25].key.xid, Xid(54),              "key 25 xid");
        assert_eq!(records[25].value.flags, OvFlags::empty(), "value 25 flags");
        assert_eq!(records[25].value.size, 4096,              "value 25 size");
        assert_eq!(records[25].value.paddr, Paddr(1454548),   "value 25 paddr");
    }

    #[test]
    fn can_get_exact_matching_record_from_leaf_node() {
        let node = load_nonroot_object_map(OBJECT_MAP_LEAF_FILE);
        check_omap_leaf_record_lookup_node(&node, 0x404, 9829294, Oid(0x404), Xid(9829294), 4096, Paddr(1284313));
        check_omap_leaf_record_lookup_node(&node, 0x404, 9829474, Oid(0x404), Xid(9829474), 4096, Paddr(1077411));
        check_omap_leaf_record_lookup_node(&node, 0x408, 54, Oid(0x408), Xid(54), 4096, Paddr(1454548));
    }

    #[test]
    fn no_record_returned_on_bad_exact_match_from_leaf_node() {
        let node = load_nonroot_object_map(OBJECT_MAP_LEAF_FILE);
        check_omap_record_lookup_missing_node(&node, 0x403, 9829294);
        check_omap_record_lookup_missing_node(&node, 0x405, 9829474);
        check_omap_record_lookup_missing_node(&node, 0x407, 54);
        check_omap_record_lookup_missing_node(&node, 0x409, 54);
    }

    #[test]
    fn can_get_inexact_matching_record_from_leaf_node() {
        let node = load_nonroot_object_map(OBJECT_MAP_LEAF_FILE);
        check_omap_leaf_record_lookup_node(&node, 0x404, 9829295, Oid(0x404), Xid(9829294), 4096, Paddr(1284313));
        check_omap_leaf_record_lookup_node(&node, 0x404, 9829473, Oid(0x404), Xid(9829294), 4096, Paddr(1284313));
        check_omap_leaf_record_lookup_node(&node, 0x404, 9829475, Oid(0x404), Xid(9829474), 4096, Paddr(1077411));
        check_omap_leaf_record_lookup_node(&node, 0x404, 19829474, Oid(0x404), Xid(9829474), 4096, Paddr(1077411));
        check_omap_leaf_record_lookup_node(&node, 0x404, u64::MAX, Oid(0x404), Xid(9829474), 4096, Paddr(1077411));
        check_omap_leaf_record_lookup_node(&node, 0x408, 55, Oid(0x408), Xid(54), 4096, Paddr(1454548));
        check_omap_leaf_record_lookup_node(&node, 0x408, u64::MAX, Oid(0x408), Xid(54), 4096, Paddr(1454548));
    }

    #[test]
    fn no_record_returned_on_bad_inexact_match_from_leaf_node() {
        let node = load_nonroot_object_map(OBJECT_MAP_LEAF_FILE);
        check_omap_record_lookup_missing_node(&node, 0x404, 0);
        check_omap_record_lookup_missing_node(&node, 0x404, 9820354);
        check_omap_record_lookup_missing_node(&node, 0x408, 0);
        check_omap_record_lookup_missing_node(&node, 0x408, 53);
    }

    fn load_object_map_from_dummy_source() -> (APFS<DummySource>, Btree<OmapVal>) {
        const BLOCK_SIZE: usize = 4096;
        const ROOT_BLOCK: u64 = 1066964;
        const NONLEAF_BLOCK: u64 = 1079985;
        const LEAF_BLOCK: u64 = 1080572;

        let mut blob_source = File::open(test_dir().join(OBJECT_MAP_ROOT_FILE)).expect("Unable to load blob");
        let mut root_blob = vec![0u8; BLOCK_SIZE];
        blob_source.read_exact(&mut root_blob).unwrap();
        let mut blob_source = File::open(test_dir().join(OBJECT_MAP_NONLEAF_FILE)).expect("Unable to load blob");
        let mut nonleaf_blob = vec![0u8; BLOCK_SIZE];
        blob_source.read_exact(&mut nonleaf_blob).unwrap();
        let mut blob_source = File::open(test_dir().join(OBJECT_MAP_LEAF_FILE)).expect("Unable to load blob");
        let mut leaf_blob = vec![0u8; BLOCK_SIZE];
        blob_source.read_exact(&mut leaf_blob).unwrap();
        let mut source = DummySource {
            position: 0,
            block_size: BLOCK_SIZE as u64,
            blocks: HashMap::new(),
        };
        source.blocks.insert(ROOT_BLOCK, root_blob);
        source.blocks.insert(NONLEAF_BLOCK, nonleaf_blob);
        source.blocks.insert(LEAF_BLOCK, leaf_blob);
        let mut apfs = APFS { source, block_size: BLOCK_SIZE };
        let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, Oid(ROOT_BLOCK), StorageType::Physical);
        (apfs, btree_result.expect("Bad b-tree load"))
    }

    #[test]
    fn can_load_object_map_from_dummy_source() {
        let (_, btree) = load_object_map_from_dummy_source();
    }

    #[test]
    #[ignore = "not implemented yet, will require some work"]
    fn can_get_exact_matching_record_from_btree() {
        let (mut apfs, btree) = load_object_map_from_dummy_source();
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x404, 9829294, Oid(0x404), Xid(9829294), 4096, Paddr(1284313));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x404, 9829474, Oid(0x404), Xid(9829474), 4096, Paddr(1077411));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x408, 54, Oid(0x408), Xid(54), 4096, Paddr(1454548));
    }

    #[test]
    #[ignore = "not implemented yet, will require some work"]
    fn no_record_returned_on_bad_exact_match_from_btree() {
        let (mut apfs, btree) = load_object_map_from_dummy_source();
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 0x403, 9829294);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 0x405, 9829474);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 0x407, 54);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 0x409, 54);
    }

    #[test]
    #[ignore = "not implemented yet, will require some work"]
    fn can_get_inexact_matching_record_from_btree() {
        let (mut apfs, btree) = load_object_map_from_dummy_source();
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x404, 9829295, Oid(0x404), Xid(9829294), 4096, Paddr(1284313));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x404, 9829473, Oid(0x404), Xid(9829294), 4096, Paddr(1284313));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x404, 9829475, Oid(0x404), Xid(9829474), 4096, Paddr(1077411));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x404, 19829474, Oid(0x404), Xid(9829474), 4096, Paddr(1077411));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x404, u64::MAX, Oid(0x404), Xid(9829474), 4096, Paddr(1077411));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x408, 55, Oid(0x408), Xid(54), 4096, Paddr(1454548));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 0x408, u64::MAX, Oid(0x408), Xid(54), 4096, Paddr(1454548));
    }

    #[test]
    #[ignore = "not implemented yet, will require some work"]
    fn no_record_returned_on_bad_inexact_match_from_btree() {
        let (mut apfs, btree) = load_object_map_from_dummy_source();
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 0x404, 0);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 0x404, 9820354);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 0x408, 0);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 0x408, 53);
    }
}

#[test]
fn test_load_volume_superblock() {
    let (mut apfs, superblock, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
    assert_ne!(superblock.body.fs_oid[0], Oid(0));
    let mut found = -1;
    let records = match &btree.root.records {
        AnyRecords::Leaf(ref x) => x,
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

fn check_omap_nonleaf_record_lookup_node(node: &BtreeNode<OmapVal>, key_oid: u64, key_xid: u64, oid: Oid, xid: Xid, value: Oid) {
    let any_record = node.get_record(&OmapKey::new(key_oid, key_xid));
    assert!(any_record.is_some(), "no matching record found");
    let any_record = any_record.unwrap();
    let record = match any_record {
        AnyRecord::NonLeaf(x, _) => x,
        _ => { panic!("Expected a non-leaf node"); },
    };
    assert_eq!(record.key.oid, oid);
    assert_eq!(record.key.xid, xid);
    assert_eq!(record.value.oid, value);
}

fn check_omap_leaf_record_lookup(any_record: Option<AnyRecord<OmapVal>>, oid: Oid, xid: Xid, size: u32, paddr: Paddr) {
    assert!(any_record.is_some(), "no matching record found");
    let any_record = any_record.unwrap();
    let record = match any_record {
        AnyRecord::Leaf(x) => x,
        _ => { panic!("Expected a leaf node"); },
    };
    assert_eq!(record.key.oid, oid);
    assert_eq!(record.key.xid, xid);
    assert!(record.value.flags.is_empty());
    assert_eq!(record.value.size, size);
    assert_eq!(record.value.paddr, paddr);
}

fn check_omap_leaf_record_lookup_node(node: &BtreeNode<OmapVal>, key_oid: u64, key_xid: u64, oid: Oid, xid: Xid, size: u32, paddr: Paddr) {
   check_omap_leaf_record_lookup(node.get_record(&OmapKey::new(key_oid, key_xid)), oid, xid, size, paddr)
}

fn check_omap_leaf_record_lookup_btree<S: Read + Seek>(btree: &Btree<OmapVal>, apfs: &mut APFS<S>, key_oid: u64, key_xid: u64, oid: Oid, xid: Xid, size: u32, paddr: Paddr) {
    let record = btree.get_record(apfs, &OmapKey::new(key_oid, key_xid)).expect("error looking up record");
    check_omap_leaf_record_lookup(record.map(|x| AnyRecord::Leaf(x)), oid, xid, size, paddr);
}

fn check_omap_record_lookup_missing(any_record: Option<AnyRecord<OmapVal>>) {
    assert!(any_record.is_none(), "matching record not expected");
}

fn check_omap_record_lookup_missing_node(node: &BtreeNode<OmapVal>, key_oid: u64, key_xid: u64) {
    check_omap_record_lookup_missing(node.get_record(&OmapKey::new(key_oid, key_xid)))
}

fn check_omap_record_lookup_missing_btree<S: Read + Seek>(btree: &Btree<OmapVal>, apfs: &mut APFS<S>, key_oid: u64, key_xid: u64) {
    let record = btree.get_record(apfs, &OmapKey::new(key_oid, key_xid)).expect("error looking up record");
    check_omap_record_lookup_missing(record.map(|x| AnyRecord::Leaf(x)))
}

mod block_4k {
    use super::*;

    #[test]
    fn can_get_exact_matching_record_from_leaf_node() {
        let (mut apfs, superblock, omap, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
        check_omap_leaf_record_lookup_node(&btree.root, 1026, 4, Oid(1026), Xid(4), 4096, Paddr(102));
    }

    #[test]
    fn no_record_returned_on_bad_exact_match_from_leaf_node() {
        let (_, _, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
        check_omap_record_lookup_missing_node(&btree.root, 500, 999);
        check_omap_record_lookup_missing_node(&btree.root, 2012, 1);
    }

    #[test]
    fn can_get_inexact_matching_record_from_leaf_node() {
        let (_, _, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
        check_omap_leaf_record_lookup_node(&btree.root, 1026, 100, Oid(1026), Xid(4), 4096, Paddr(102))
    }

    #[test]
    fn no_record_returned_on_bad_inexact_match_from_leaf_node() {
        let (_, _, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
        check_omap_record_lookup_missing_node(&btree.root, 1026, 1);
    }

    #[test]
    fn can_get_exact_matching_record_from_btree() {
        let (mut apfs, _, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1026, 4, Oid(1026), Xid(4), 4096, Paddr(102));
    }

    #[test]
    fn no_record_returned_on_bad_exact_match_from_btree() {
        let (mut apfs, _, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 500, 999);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 2012, 1);
    }

    #[test]
    fn can_get_inexact_matching_record_from_btree() {
        let (mut apfs, _, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1026, 100, Oid(1026), Xid(4), 4096, Paddr(102));
    }

    #[test]
    fn no_record_returned_on_bad_inexact_match_from_btree() {
        let (mut apfs, _, _, btree) = load_test_apfs_object_map_btree(TEST_APFS_FILE);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1026, 1);
    }
}

#[cfg_attr(not(feature = "expensive_tests"), ignore)]
mod block_16k {
    use super::*;

    #[test]
    fn can_get_exact_matching_record_from_leaf_node() {
        let (_, _, _, btree) = load_test_apfs_object_map_btree(TEST_16KB_APFS_FILE);
        check_omap_leaf_record_lookup_node(&btree.root, 1026, 2, Oid(1026), Xid(2), 16384, Paddr(978));
        check_omap_leaf_record_lookup_node(&btree.root, 1030, 3, Oid(1030), Xid(3), 16384, Paddr(986));
        check_omap_leaf_record_lookup_node(&btree.root, 1032, 4, Oid(1032), Xid(4), 16384, Paddr(998));
    }

    #[test]
    fn no_record_returned_on_bad_exact_match_from_leaf_node() {
        let (_, _, _, btree) = load_test_apfs_object_map_btree(TEST_16KB_APFS_FILE);
        check_omap_record_lookup_missing_node(&btree.root, 1025, 2);
        check_omap_record_lookup_missing_node(&btree.root, 1027, 2);
        check_omap_record_lookup_missing_node(&btree.root, 1029, 3);
        check_omap_record_lookup_missing_node(&btree.root, 1031, 3);
        check_omap_record_lookup_missing_node(&btree.root, 1031, 4);
        check_omap_record_lookup_missing_node(&btree.root, 1033, 4);
    }

    #[test]
    fn can_get_inexact_matching_record_from_leaf_node() {
        let (_, _, _, btree) = load_test_apfs_object_map_btree(TEST_16KB_APFS_FILE);
        check_omap_leaf_record_lookup_node(&btree.root, 1026, 3, Oid(1026), Xid(2), 16384, Paddr(978));
        check_omap_leaf_record_lookup_node(&btree.root, 1026, 4, Oid(1026), Xid(2), 16384, Paddr(978));
        check_omap_leaf_record_lookup_node(&btree.root, 1030, 4, Oid(1030), Xid(3), 16384, Paddr(986));
        check_omap_leaf_record_lookup_node(&btree.root, 1030, 9, Oid(1030), Xid(3), 16384, Paddr(986));
        check_omap_leaf_record_lookup_node(&btree.root, 1032, 5, Oid(1032), Xid(4), 16384, Paddr(998));
        check_omap_leaf_record_lookup_node(&btree.root, 1032, 30, Oid(1032), Xid(4), 16384, Paddr(998));
    }

    #[test]
    fn no_record_returned_on_bad_inexact_match_from_leaf_node() {
        let (_, _, _, btree) = load_test_apfs_object_map_btree(TEST_16KB_APFS_FILE);
        check_omap_record_lookup_missing_node(&btree.root, 1026, 0);
        check_omap_record_lookup_missing_node(&btree.root, 1026, 1);
        check_omap_record_lookup_missing_node(&btree.root, 1030, 1);
        check_omap_record_lookup_missing_node(&btree.root, 1030, 2);
        check_omap_record_lookup_missing_node(&btree.root, 1032, 2);
        check_omap_record_lookup_missing_node(&btree.root, 1032, 3);
    }

    #[test]
    fn can_get_exact_matching_record_from_btree() {
        let (mut apfs, _, _, btree) = load_test_apfs_object_map_btree(TEST_16KB_APFS_FILE);
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1026, 2, Oid(1026), Xid(2), 16384, Paddr(978));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1030, 3, Oid(1030), Xid(3), 16384, Paddr(986));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1032, 4, Oid(1032), Xid(4), 16384, Paddr(998));
    }

    #[test]
    fn no_record_returned_on_bad_exact_match_from_btree() {
        let (mut apfs, _, _, btree) = load_test_apfs_object_map_btree(TEST_16KB_APFS_FILE);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1025, 2);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1027, 2);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1029, 3);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1031, 3);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1031, 4);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1033, 4);
    }

    #[test]
    fn can_get_inexact_matching_record_from_btree() {
        let (mut apfs, _, _, btree) = load_test_apfs_object_map_btree(TEST_16KB_APFS_FILE);
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1026, 3, Oid(1026), Xid(2), 16384, Paddr(978));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1026, 4, Oid(1026), Xid(2), 16384, Paddr(978));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1030, 4, Oid(1030), Xid(3), 16384, Paddr(986));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1030, 9, Oid(1030), Xid(3), 16384, Paddr(986));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1032, 5, Oid(1032), Xid(4), 16384, Paddr(998));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 1032, 30, Oid(1032), Xid(4), 16384, Paddr(998));
    }

    #[test]
    fn no_record_returned_on_bad_inexact_match_from_btree() {
        let (mut apfs, _, _, btree) = load_test_apfs_object_map_btree(TEST_16KB_APFS_FILE);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1026, 0);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1026, 1);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1030, 1);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1030, 2);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1032, 2);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 1032, 3);
    }
}

mod dummy_node {
    use super::*;

    fn create_dummy_single_node() -> (APFS<Cursor<Vec<u8>>>, Btree<OmapVal>) {
        (APFS {
            source: Cursor::new(Vec::<u8>::new()),
            block_size: 4096,
        },
        Btree {
            root: Rc::new(BtreeNode {
                node: BtreeNodeObject {
                    header: ObjPhys {
                        cksum: 0,
                        oid: Oid(0),
                        xid: Xid(0),
                        r#type: ObjectTypeAndFlags::new_by_field(ObjectType::Btree, StorageType::Physical, ObjTypeFlags::empty()),
                        subtype: ObjectTypeAndFlags::new_by_field(ObjectType::Omap, StorageType::Virtual, ObjTypeFlags::empty()),
                    },
                    body: BtreeNodePhys {
                        flags: BtnFlags::ROOT | BtnFlags::LEAF | BtnFlags::FIXED_KV_SIZE,
                        level: 0,
                        nkeys: 8,
                        table_space: Nloc {
                            off: 0,
                            len: 0,
                        },
                        free_space: Nloc {
                            off: 0,
                            len: 0,
                        },
                        key_free_list: Nloc {
                            off: 0,
                            len: 0,
                        },
                        val_free_list: Nloc {
                            off: 0,
                            len: 0,
                        },
                        data: vec![],
                    },
                },
                records: AnyRecords::Leaf(vec![
                    LeafRecord {
                        key: OmapKey { oid: Oid(110), xid: Xid(1000) },
                        value: OmapVal { flags: OvFlags::empty(), size: 4096, paddr: Paddr(30), },
                    },
                    LeafRecord {
                        key: OmapKey { oid: Oid(120), xid: Xid(100) },
                        value: OmapVal { flags: OvFlags::empty(), size: 4096, paddr: Paddr(50), },
                    },
                    LeafRecord {
                        key: OmapKey { oid: Oid(120), xid: Xid(200) },
                        value: OmapVal { flags: OvFlags::empty(), size: 4096, paddr: Paddr(40), },
                    },
                    LeafRecord {
                        key: OmapKey { oid: Oid(120), xid: Xid(300) },
                        value: OmapVal { flags: OvFlags::empty(), size: 4096, paddr: Paddr(60), },
                    },
                    LeafRecord {
                        key: OmapKey { oid: Oid(130), xid: Xid(50) },
                        value: OmapVal { flags: OvFlags::empty(), size: 4096, paddr: Paddr(100), },
                    },
                    LeafRecord {
                        key: OmapKey { oid: Oid(130), xid: Xid(51) },
                        value: OmapVal { flags: OvFlags::empty(), size: 4096, paddr: Paddr(101), },
                    },
                    LeafRecord {
                        key: OmapKey { oid: Oid(131), xid: Xid(10) },
                        value: OmapVal { flags: OvFlags::empty(), size: 4096, paddr: Paddr(90), },
                    },
                    LeafRecord {
                        key: OmapKey { oid: Oid(135), xid: Xid(50) },
                        value: OmapVal { flags: OvFlags::empty(), size: 4096, paddr: Paddr(95), },
                    },
                ]),
                _v: PhantomData,
            }),
            info: BtreeInfo {
                fixed: BtreeInfoFixed {
                    flags: BtFlags::SEQUENTIAL_INSERT | BtFlags::PHYSICAL,
                    node_size: 4096,
                    key_size: 16,
                    val_size: 16,
                },
                longest_key: 16,
                longest_val: 16,
                key_count: 8,
                node_count: 1,
            },
            _v: PhantomData,
        })
    }

    #[test]
    fn can_get_exact_matching_record_from_leaf_node() {
        let (_, btree) = create_dummy_single_node();
        check_omap_leaf_record_lookup_node(&btree.root, 110, 1000, Oid(110), Xid(1000), 4096, Paddr(30));
        check_omap_leaf_record_lookup_node(&btree.root, 120, 100, Oid(120), Xid(100), 4096, Paddr(50));
        check_omap_leaf_record_lookup_node(&btree.root, 120, 200, Oid(120), Xid(200), 4096, Paddr(40));
        check_omap_leaf_record_lookup_node(&btree.root, 120, 300, Oid(120), Xid(300), 4096, Paddr(60));
        check_omap_leaf_record_lookup_node(&btree.root, 130, 50, Oid(130), Xid(50), 4096, Paddr(100));
        check_omap_leaf_record_lookup_node(&btree.root, 130, 51, Oid(130), Xid(51), 4096, Paddr(101));
        check_omap_leaf_record_lookup_node(&btree.root, 131, 10, Oid(131), Xid(10), 4096, Paddr(90));
        check_omap_leaf_record_lookup_node(&btree.root, 135, 50, Oid(135), Xid(50), 4096, Paddr(95));
    }

    #[test]
    fn no_record_returned_on_bad_exact_match_from_leaf_node() {
        let (_, btree) = create_dummy_single_node();
        check_omap_record_lookup_missing_node(&btree.root, 109, 1000);
        check_omap_record_lookup_missing_node(&btree.root, 111, 1000);
        check_omap_record_lookup_missing_node(&btree.root, 119, 100);
        check_omap_record_lookup_missing_node(&btree.root, 121, 200);
        check_omap_record_lookup_missing_node(&btree.root, 129, 50);
        check_omap_record_lookup_missing_node(&btree.root, 134, 10);
        check_omap_record_lookup_missing_node(&btree.root, 136, 50);
    }

    #[test]
    fn can_get_inexact_matching_record_from_leaf_node() {
        let (_, btree) = create_dummy_single_node();
        check_omap_leaf_record_lookup_node(&btree.root, 110, u64::MAX, Oid(110), Xid(1000), 4096, Paddr(30));
        check_omap_leaf_record_lookup_node(&btree.root, 120, 199, Oid(120), Xid(100), 4096, Paddr(50));
        check_omap_leaf_record_lookup_node(&btree.root, 120, 201, Oid(120), Xid(200), 4096, Paddr(40));
        check_omap_leaf_record_lookup_node(&btree.root, 120, 299, Oid(120), Xid(200), 4096, Paddr(40));
        check_omap_leaf_record_lookup_node(&btree.root, 120, 30000, Oid(120), Xid(300), 4096, Paddr(60));
        check_omap_leaf_record_lookup_node(&btree.root, 130, 51, Oid(130), Xid(51), 4096, Paddr(101));
        check_omap_leaf_record_lookup_node(&btree.root, 130, u64::MAX, Oid(130), Xid(51), 4096, Paddr(101));
        check_omap_leaf_record_lookup_node(&btree.root, 131, 49, Oid(131), Xid(10), 4096, Paddr(90));
        check_omap_leaf_record_lookup_node(&btree.root, 135, 65, Oid(135), Xid(50), 4096, Paddr(95));
    }

    #[test]
    fn no_record_returned_on_bad_inexact_match_from_leaf_node() {
        let (_, btree) = create_dummy_single_node();
        check_omap_record_lookup_missing_node(&btree.root, 110, 999);
        check_omap_record_lookup_missing_node(&btree.root, 120, 0);
        check_omap_record_lookup_missing_node(&btree.root, 120, 50);
        check_omap_record_lookup_missing_node(&btree.root, 120, 99);
        check_omap_record_lookup_missing_node(&btree.root, 130, 49);
        check_omap_record_lookup_missing_node(&btree.root, 130, 1);
        check_omap_record_lookup_missing_node(&btree.root, 131, 9);
        check_omap_record_lookup_missing_node(&btree.root, 135, 49);
    }

    #[test]
    fn can_get_exact_matching_record_from_btree() {
        let (mut apfs, btree) = create_dummy_single_node();
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 110, 1000, Oid(110), Xid(1000), 4096, Paddr(30));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 120, 100, Oid(120), Xid(100), 4096, Paddr(50));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 120, 200, Oid(120), Xid(200), 4096, Paddr(40));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 120, 300, Oid(120), Xid(300), 4096, Paddr(60));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 130, 50, Oid(130), Xid(50), 4096, Paddr(100));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 130, 51, Oid(130), Xid(51), 4096, Paddr(101));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 131, 10, Oid(131), Xid(10), 4096, Paddr(90));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 135, 50, Oid(135), Xid(50), 4096, Paddr(95));
    }

    #[test]
    fn no_record_returned_on_bad_exact_match_from_btree() {
        let (mut apfs, btree) = create_dummy_single_node();
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 109, 1000);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 111, 1000);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 119, 100);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 121, 200);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 129, 50);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 134, 10);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 136, 50);
    }

    #[test]
    fn can_get_inexact_matching_record_from_btree() {
        let (mut apfs, btree) = create_dummy_single_node();
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 110, u64::MAX, Oid(110), Xid(1000), 4096, Paddr(30));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 120, 199, Oid(120), Xid(100), 4096, Paddr(50));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 120, 201, Oid(120), Xid(200), 4096, Paddr(40));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 120, 299, Oid(120), Xid(200), 4096, Paddr(40));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 120, 30000, Oid(120), Xid(300), 4096, Paddr(60));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 130, 51, Oid(130), Xid(51), 4096, Paddr(101));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 130, u64::MAX, Oid(130), Xid(51), 4096, Paddr(101));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 131, 49, Oid(131), Xid(10), 4096, Paddr(90));
        check_omap_leaf_record_lookup_btree(&btree, &mut apfs, 135, 65, Oid(135), Xid(50), 4096, Paddr(95));
    }

    #[test]
    fn no_record_returned_on_bad_inexact_match_from_btree() {
        let (mut apfs, btree) = create_dummy_single_node();
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 110, 999);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 120, 0);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 120, 50);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 120, 99);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 130, 49);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 130, 1);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 131, 9);
        check_omap_record_lookup_missing_btree(&btree, &mut apfs, 135, 49);
    }
}
