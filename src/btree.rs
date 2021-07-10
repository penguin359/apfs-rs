use std::cmp::Ordering;

use crate::internal::Oid;
use crate::internal::Xid;
use crate::internal::OmapKey;
use crate::internal::OvFlags;

pub trait Key : PartialOrd + Ord + PartialEq + Eq {
}

// #[derive(Debug)]
// struct OmapKey {
//     oid: Oid,
//     xid: Xid,
// }

impl Ord for OmapKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.oid.cmp(&other.oid);
        match order {
            Ordering::Equal => self.xid.cmp(&other.xid),
            _ => order,
        }
    }
}

impl PartialOrd for OmapKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OmapKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for OmapKey {
}

impl Key for OmapKey {
}

#[derive(Debug)]
pub struct Record<K, V> 
    where K: Key {
    pub key: K,
    pub value: V,
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_object_map_key_ordering() {
        let key1 = OmapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        let key2 = OmapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        let key_oid_less = OmapKey {
            oid: Oid(21),
            xid: Xid(17),
        };
        let key_oid_greater = OmapKey {
            oid: Oid(25),
            xid: Xid(17),
        };
        let key_xid_less = OmapKey {
            oid: Oid(23),
            xid: Xid(16),
        };
        let key_xid_greater = OmapKey {
            oid: Oid(23),
            xid: Xid(18),
        };
        let key_oid_less_xid_less = OmapKey {
            oid: Oid(21),
            xid: Xid(16),
        };
        let key_oid_greater_xid_less = OmapKey {
            oid: Oid(25),
            xid: Xid(16),
        };
        let key_oid_less_xid_greater = OmapKey {
            oid: Oid(21),
            xid: Xid(18),
        };
        let key_oid_greater_xid_greater = OmapKey {
            oid: Oid(25),
            xid: Xid(18),
        };
        assert_eq!(key1.cmp(&key2), Ordering::Equal);
        assert_eq!(key1.cmp(&key_oid_less), Ordering::Greater);
        assert_eq!(key1.cmp(&key_oid_greater), Ordering::Less);
        assert_eq!(key1.cmp(&key_xid_less), Ordering::Greater);
        assert_eq!(key1.cmp(&key_xid_greater), Ordering::Less);
        assert_eq!(key1.cmp(&key_oid_less_xid_less), Ordering::Greater);
        assert_eq!(key1.cmp(&key_oid_less_xid_greater), Ordering::Greater);
        assert_eq!(key1.cmp(&key_oid_greater_xid_less), Ordering::Less);
        assert_eq!(key1.cmp(&key_oid_greater_xid_greater), Ordering::Less);
    }

    #[test]
    fn test_object_map_key_equal() {
        let key1 = OmapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        let key2 = OmapKey {
            oid: Oid(23),
            xid: Xid(17),
        };
        let key3 = OmapKey {
            oid: Oid(21),
            xid: Xid(17),
        };
        let key4 = OmapKey {
            oid: Oid(23),
            xid: Xid(18),
        };
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
        assert_ne!(key1, key4);
        assert_ne!(key2, key3);
        assert_ne!(key2, key4);
        assert_ne!(key3, key4);
    }

    use crate::{APFS, APFSObject, Paddr, StorageType};
    use crate::tests::test_dir;

    #[test]
    fn test_load_object_map() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let object = apfs.load_object_addr(Paddr(0)).unwrap();
        let superblock = match object {
            APFSObject::Superblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
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
            APFSObject::BtreeNode(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
    }

    #[test]
    fn test_load_object_map_btree() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let object = apfs.load_object_addr(Paddr(0)).unwrap();
        let superblock = match object {
            APFSObject::Superblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
        let omap = match object {
            APFSObject::ObjectMap(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        let btree_result = apfs.load_btree(omap.body.tree_oid, StorageType::Physical);
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let btree = btree_result.unwrap();
        assert_eq!(btree.records.len(), 1);
        assert_eq!(btree.records[0].key.oid, Oid(1026));
        assert_eq!(btree.records[0].key.xid, Xid(4));
        assert!(btree.records[0].value.flags.is_empty());
        assert_eq!(btree.records[0].value.size, 4096);
        assert_eq!(btree.records[0].value.paddr, Paddr(102));
    }

    #[test]
    fn test_load_object_map_btree_dummy() {
        let mut source = File::open(&test_dir().join("btree.blob")).expect("Unable to load blob");
        let mut apfs = APFS { source, block_size: 4096 };
        let btree_result = apfs.load_btree(Oid(0), StorageType::Physical);
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let btree = btree_result.unwrap();
        assert_eq!(btree.records.len(), 6);
        assert_eq!(btree.records[0].key.oid, Oid(0x0586), "key 0 oid");
        assert_eq!(btree.records[0].key.xid, Xid(0x2000), "key 0 xid");
        assert_eq!(btree.records[1].key.oid, Oid(0x0588), "key 1 oid");
        assert_eq!(btree.records[1].key.xid, Xid(0x2101), "key 1 xid");
        assert_eq!(btree.records[2].key.oid, Oid(0x0588), "key 2 oid");
        assert_eq!(btree.records[2].key.xid, Xid(0x2202), "key 2 xid");
        assert_eq!(btree.records[3].key.oid, Oid(0x0588), "key 3 oid");
        assert_eq!(btree.records[3].key.xid, Xid(0x2300), "key 3 xid");
        assert_eq!(btree.records[4].key.oid, Oid(0x0589), "key 4 oid");
        assert_eq!(btree.records[4].key.xid, Xid(0x1000), "key 4 xid");
        assert_eq!(btree.records[5].key.oid, Oid(0x0589), "key 5 oid");
        assert_eq!(btree.records[5].key.xid, Xid(0x2000), "key 5 xid");
        assert_eq!(btree.records[0].value.flags, OvFlags::empty(), "value 0 flags");
        assert_eq!(btree.records[0].value.size, 4096,              "value 0 size");
        assert_eq!(btree.records[0].value.paddr, Paddr(0x400),     "value 0 paddr");
        assert_eq!(btree.records[1].value.flags, OvFlags::empty(), "value 1 flags");
        assert_eq!(btree.records[1].value.size, 4096,              "value 1 size");
        assert_eq!(btree.records[1].value.paddr, Paddr(0x200),     "value 1 paddr");
        assert_eq!(btree.records[2].value.flags, OvFlags::empty(), "value 2 flags");
        assert_eq!(btree.records[2].value.size, 4096,              "value 2 size");
        assert_eq!(btree.records[2].value.paddr, Paddr(0x300),     "value 2 paddr");
        assert_eq!(btree.records[3].value.flags, OvFlags::empty(), "value 3 flags");
        assert_eq!(btree.records[3].value.size, 4096,              "value 3 size");
        assert_eq!(btree.records[3].value.paddr, Paddr(0x100),     "value 3 paddr");
        assert_eq!(btree.records[4].value.flags, OvFlags::empty(), "value 4 flags");
        assert_eq!(btree.records[4].value.size, 4096,              "value 4 size");
        assert_eq!(btree.records[4].value.paddr, Paddr(0x500),     "value 4 paddr");
        assert_eq!(btree.records[5].value.flags, OvFlags::empty(), "value 5 flags");
        assert_eq!(btree.records[5].value.size, 4096,              "value 5 size");
        assert_eq!(btree.records[5].value.paddr, Paddr(0x600),     "value 5 paddr");
    }

    #[test]
    fn test_load_volume_superblock() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let object = apfs.load_object_addr(Paddr(0)).unwrap();
        let superblock = match object {
            APFSObject::Superblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
        let omap = match object {
            APFSObject::ObjectMap(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        let btree_result = apfs.load_btree(omap.body.tree_oid, StorageType::Physical);
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let btree = btree_result.unwrap();
        assert_ne!(superblock.body.fs_oid[0], Oid(0));
        let mut found = -1;
        for idx in 0..btree.records.len() {
            if btree.records[idx].key.oid == superblock.body.fs_oid[0] {
                found = idx as isize;
                break;
            }
        }
        assert!(found >= 0);
        let object = apfs.load_object_addr(btree.records[found as usize].value.paddr).unwrap();
        let volume = match object {
            APFSObject::ApfsSuperblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        assert_eq!(volume.body.volname[0..7], *b"MYAPFS\0");
    }
}
