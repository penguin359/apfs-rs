use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};
use std::io::Cursor;
use std::ops::RangeBounds;

use byteorder::{LittleEndian, ReadBytesExt, BigEndian};
use num_traits::FromPrimitive;

use crate::{BtreeNodePhys, KVoff, ObjectType, JObjTypes, JDrecHashedKey, JInodeKey, JInodeVal, JDrecVal, JXattrVal, JXattrKey, JFileExtentKey, JFileExtentVal, JDstreamIdKey, JDstreamIdVal, JSiblingKey, JSiblingMapKey, JSiblingMapVal, XfBlob, XFieldDrec, DrecExtType, XFieldInode, InoExtType, JDstream};
use crate::internal::{KVloc, Nloc};
use crate::internal::Oid;
use crate::internal::Xid;
use crate::internal::{BtnFlags, OmapKey};
use crate::internal::OmapVal;
use crate::internal::OvFlags;
use crate::internal::BtreeInfo;
use crate::internal::JKey;

use crate::{APFS, APFSObject, BtreeNodeObject, Paddr, StorageType};

pub trait Key : PartialOrd + Ord + PartialEq + Eq + Sized {
    fn import(source: &mut dyn Read) -> io::Result<Self>;

    // fn import_value(&self, source: &mut dyn Read) -> io::Result<V> {
}

pub trait Value : Sized {
    fn import(source: &mut dyn Read) -> io::Result<Self>;
}

// #[derive(Debug)]
// struct OmapKey {
//     oid: Oid,
//     xid: Xid,
// }

/* Object map keys match on the object ID equality and
   a transaction ID that is less than or equal */
impl Ord for OmapKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.oid.cmp(&other.oid);
        match order {
            Ordering::Equal => match self.xid.cmp(&other.xid) {
                Ordering::Less => Ordering::Less,
                _ => Ordering::Equal,
            },
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
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self::import(source)?)
    }

    // fn import_value(&self, source: &mut dyn Read) -> io::Result<OmapVal> {
    //     Ok(OmapVal::import(source)?)
    // }
}

impl Value for OmapVal {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self::import(source)?)
    }
}

struct ApfsKey {
    pub key: JKey,
}

/* Object map keys match on the object ID equality and
   a transaction ID that is less than or equal */
impl Ord for ApfsKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // let order = self.oid.cmp(&other.oid);
        // match order {
        //     Ordering::Equal => match self.xid.cmp(&other.xid) {
        //         Ordering::Less => Ordering::Less,
        //         _ => Ordering::Equal,
        //     },
        //     _ => order,
        // }
        Ordering::Equal
    }
}

impl PartialOrd for ApfsKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ApfsKey {
    fn eq(&self, other: &Self) -> bool {
        // self.cmp(other) == Ordering::Equal
        true
    }
}

impl Eq for ApfsKey {
}

impl Key for ApfsKey {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        // Ok(Self::import(source)?)
        Err(io::Error::new(io::ErrorKind::Other, "not implemented"))
    }
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
        /* Matching keys have same Oid and and Xid less than or equal */
        assert_eq!(key1.cmp(&key_xid_less), Ordering::Equal);
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

    // #[test]
    // fn test_volume_object_key_ordering() {
    //     let key1 = JInodeKey {
    //         oid: Oid(23),
    //         xid: Xid(17),
    //     };
    //     let key2 = OmapKey {
    //         oid: Oid(23),
    //         xid: Xid(17),
    //     };
    //     let key_oid_less = OmapKey {
    //         oid: Oid(21),
    //         xid: Xid(17),
    //     };
    //     let key_oid_greater = OmapKey {
    //         oid: Oid(25),
    //         xid: Xid(17),
    //     };
    //     let key_xid_less = OmapKey {
    //         oid: Oid(23),
    //         xid: Xid(16),
    //     };
    //     let key_xid_greater = OmapKey {
    //         oid: Oid(23),
    //         xid: Xid(18),
    //     };
    //     let key_oid_less_xid_less = OmapKey {
    //         oid: Oid(21),
    //         xid: Xid(16),
    //     };
    //     let key_oid_greater_xid_less = OmapKey {
    //         oid: Oid(25),
    //         xid: Xid(16),
    //     };
    //     let key_oid_less_xid_greater = OmapKey {
    //         oid: Oid(21),
    //         xid: Xid(18),
    //     };
    //     let key_oid_greater_xid_greater = OmapKey {
    //         oid: Oid(25),
    //         xid: Xid(18),
    //     };
    //     assert_eq!(key1.cmp(&key2), Ordering::Equal);
    //     assert_eq!(key1.cmp(&key_oid_less), Ordering::Greater);
    //     assert_eq!(key1.cmp(&key_oid_greater), Ordering::Less);
    //     /* Matching keys have same Oid and and Xid less than or equal */
    //     assert_eq!(key1.cmp(&key_xid_less), Ordering::Equal);
    //     assert_eq!(key1.cmp(&key_xid_greater), Ordering::Less);
    //     assert_eq!(key1.cmp(&key_oid_less_xid_less), Ordering::Greater);
    //     assert_eq!(key1.cmp(&key_oid_less_xid_greater), Ordering::Greater);
    //     assert_eq!(key1.cmp(&key_oid_greater_xid_less), Ordering::Less);
    //     assert_eq!(key1.cmp(&key_oid_greater_xid_greater), Ordering::Less);
    // }

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
            APFSObject::Btree(x) => x,
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
        let btree_result = Btree::<OmapKey, OmapVal>::load_btree(&mut apfs, omap.body.tree_oid, StorageType::Physical);
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let btree = btree_result.unwrap();
        assert_eq!(btree.root.records.len(), 1);
        assert_eq!(btree.root.records[0].key.oid, Oid(1026));
        assert_eq!(btree.root.records[0].key.xid, Xid(4));
        assert!(btree.root.records[0].value.flags.is_empty());
        assert_eq!(btree.root.records[0].value.size, 4096);
        assert_eq!(btree.root.records[0].value.paddr, Paddr(102));
    }

    #[test]
    fn test_load_object_map_btree_dummy() {
        let mut source = File::open(&test_dir().join("btree.blob")).expect("Unable to load blob");
        let mut apfs = APFS { source, block_size: 4096 };
        let btree_result = Btree::<OmapKey, OmapVal>::load_btree(&mut apfs, Oid(0), StorageType::Physical);
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let btree = btree_result.unwrap();
        assert_eq!(btree.root.records.len(), 6);
        assert_eq!(btree.root.records[0].key.oid, Oid(0x0586), "key 0 oid");
        assert_eq!(btree.root.records[0].key.xid, Xid(0x2000), "key 0 xid");
        assert_eq!(btree.root.records[1].key.oid, Oid(0x0588), "key 1 oid");
        assert_eq!(btree.root.records[1].key.xid, Xid(0x2101), "key 1 xid");
        assert_eq!(btree.root.records[2].key.oid, Oid(0x0588), "key 2 oid");
        assert_eq!(btree.root.records[2].key.xid, Xid(0x2202), "key 2 xid");
        assert_eq!(btree.root.records[3].key.oid, Oid(0x0588), "key 3 oid");
        assert_eq!(btree.root.records[3].key.xid, Xid(0x2300), "key 3 xid");
        assert_eq!(btree.root.records[4].key.oid, Oid(0x0589), "key 4 oid");
        assert_eq!(btree.root.records[4].key.xid, Xid(0x1000), "key 4 xid");
        assert_eq!(btree.root.records[5].key.oid, Oid(0x0589), "key 5 oid");
        assert_eq!(btree.root.records[5].key.xid, Xid(0x2000), "key 5 xid");
        assert_eq!(btree.root.records[0].value.flags, OvFlags::empty(), "value 0 flags");
        assert_eq!(btree.root.records[0].value.size, 4096,              "value 0 size");
        assert_eq!(btree.root.records[0].value.paddr, Paddr(0x400),     "value 0 paddr");
        assert_eq!(btree.root.records[1].value.flags, OvFlags::empty(), "value 1 flags");
        assert_eq!(btree.root.records[1].value.size, 4096,              "value 1 size");
        assert_eq!(btree.root.records[1].value.paddr, Paddr(0x200),     "value 1 paddr");
        assert_eq!(btree.root.records[2].value.flags, OvFlags::empty(), "value 2 flags");
        assert_eq!(btree.root.records[2].value.size, 4096,              "value 2 size");
        assert_eq!(btree.root.records[2].value.paddr, Paddr(0x300),     "value 2 paddr");
        assert_eq!(btree.root.records[3].value.flags, OvFlags::empty(), "value 3 flags");
        assert_eq!(btree.root.records[3].value.size, 4096,              "value 3 size");
        assert_eq!(btree.root.records[3].value.paddr, Paddr(0x100),     "value 3 paddr");
        assert_eq!(btree.root.records[4].value.flags, OvFlags::empty(), "value 4 flags");
        assert_eq!(btree.root.records[4].value.size, 4096,              "value 4 size");
        assert_eq!(btree.root.records[4].value.paddr, Paddr(0x500),     "value 4 paddr");
        assert_eq!(btree.root.records[5].value.flags, OvFlags::empty(), "value 5 flags");
        assert_eq!(btree.root.records[5].value.size, 4096,              "value 5 size");
        assert_eq!(btree.root.records[5].value.paddr, Paddr(0x600),     "value 5 paddr");
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
        let btree_result = Btree::<OmapKey, OmapVal>::load_btree(&mut apfs, omap.body.tree_oid, StorageType::Physical);
        assert!(btree_result.is_ok(), "Bad b-tree load");
        let btree = btree_result.unwrap();
        assert_ne!(superblock.body.fs_oid[0], Oid(0));
        let mut found = -1;
        for idx in 0..btree.root.records.len() {
            if btree.root.records[idx].key.oid == superblock.body.fs_oid[0] {
                found = idx as isize;
                break;
            }
        }
        assert!(found >= 0);
        let object = apfs.load_object_addr(btree.root.records[found as usize].value.paddr).unwrap();
        let volume = match object {
            APFSObject::ApfsSuperblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        assert_eq!(volume.body.volname[0..7], *b"MYAPFS\0");
    }
}

//pub enum Node<K, R> {
//    //HeaderNode(HeaderNode),
//    //MapNode(MapNode),
//    IndexNode(IndexNode<K>),
//    //LeafNode(LeafNode<R>),
//}

// trait BTreeType {
//     type key: Key,
//     type value: Value,
// }

#[derive(Debug)]
pub struct Btree<K: Key, V: Value> {
    info: BtreeInfo,
    pub root: BtreeNode<K, V>,
}

#[derive(Debug)]
pub struct BtreeNode<K: Key, V: Value> {
    node: BtreeNodeObject,
    pub records: Vec<Record<K, V>>,
}

enum BtreeRawObject {
    BtreeRoot(BtreeNodeObject, BtreeInfo),
    BtreeNonRoot(BtreeNodeObject),
}

enum BtreeDecodedObject<K: Key, V: Value> {
    BtreeRoot(BtreeNode<K, V>, BtreeInfo),
    BtreeNonRoot(BtreeNode<K, V>),
}

impl<K, V> Btree<K, V> where
    K: Key,
    V: Value {
    fn load_btree_object<S: Read + Seek>(apfs: &mut APFS<S>, oid: Oid, r#type: StorageType) -> io::Result<BtreeRawObject> {
        let object = apfs.load_object_oid(oid, r#type)?;
        let mut body = match object {
            APFSObject::Btree(x) => x,
            _ => { panic!("Invalid type"); },
        };
        let info = BtreeInfo::import(&mut Cursor::new(&body.body.data[body.body.data.len()-40..]))?;
        body.body.data.truncate(body.body.data.len()-40);
        Ok(BtreeRawObject::BtreeRoot(body, info))
    }

    fn load_btree_node<S: Read + Seek>(apfs: &mut APFS<S>, oid: Oid, r#type: StorageType) -> io::Result<BtreeDecodedObject<K, V>> {
        let (body, info) = match Self::load_btree_object(apfs, oid, r#type)? {
            BtreeRawObject::BtreeRoot(body, info) => (body, Some(info)),
            _ => { unreachable!() },
        };
        if body.header.subtype.r#type() != ObjectType::Omap &&
           body.header.subtype.r#type() != ObjectType::Fstree {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported B-tree type"));
        }
        // if !body.body.flags.contains(BtnFlags::FIXED_KV_SIZE) {
        //     return Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported B-tree flag combination"));
        // }
        if !body.body.flags.contains(BtnFlags::LEAF) {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported B-tree node"));
        }
        let info = info.unwrap();
        let toc = &body.body.data[body.body.table_space.off as usize..(body.body.table_space.off+body.body.table_space.len) as usize];
        let mut cursor = Cursor::new(toc);
        let mut items = vec![];
        assert!(body.body.flags.contains(BtnFlags::LEAF));
        let mut records = vec![];
        let mut sizes = HashMap::<u64, u64>::new();
        for _ in 0..body.body.nkeys {
            let kvloc = if(body.body.flags.contains(BtnFlags::FIXED_KV_SIZE)) {
                let kvoff = KVoff::import(&mut cursor)?;
                KVloc {
                    k: Nloc {
                        off: kvoff.k,
                        len: info.fixed.key_size as u16,
                    },
                    v: Nloc {
                        off: kvoff.v,
                        len: info.fixed.key_size as u16,
                    },
                }
            } else {
                KVloc::import(&mut cursor)?
            };
            items.push(kvloc);
            let key_data = &body.body.data[(body.body.table_space.off+body.body.table_space.len+kvloc.k.off) as usize..(body.body.table_space.off+body.body.table_space.len+kvloc.k.off + kvloc.k.len) as usize];
            let val_data = &body.body.data[(body.body.data.len() as u16 - kvloc.v.off) as usize..(body.body.data.len() as u16 -  kvloc.v.off + kvloc.v.len) as usize];
            let mut key_cursor = Cursor::new(key_data);
            let mut value_cursor = Cursor::new(val_data);
            if body.header.subtype.r#type() == ObjectType::Omap  {
                let key = K::import(&mut key_cursor)?;
                let val = V::import(&mut value_cursor)?;
                let record = Record {
                    key,
                    value: val,
                };
                records.push(record);
            } else if(body.header.subtype.r#type() == ObjectType::Fstree) {
                let key = JKey::import(&mut key_cursor)?;
                let key_type = key.obj_id_and_type.r#type();
                println!("Key type: {:?}", key);
                match key_type {
                    JObjTypes::Inode => {
                        let value = JInodeVal::import(&mut value_cursor).unwrap();
                        println!("Inode key: {:?}", JInodeKey::import(&mut key_cursor).unwrap());
                        println!("Inode: {:?}", value);
                        if value.xfields.len() > 0 {
                            let mut xfields_cursor = Cursor::new(value.xfields);
                            let blob = XfBlob::import(&mut xfields_cursor)?;
                            let mut xdata_cursor = Cursor::new(blob.data);
                            let fields = (0..blob.num_exts).map(|_| XFieldInode::import(&mut xdata_cursor).unwrap()).collect::<Vec<XFieldInode>>();
                            for field in &fields {
                                let aligned_size = (field.size + 7) & 0xfff8;
                                assert!(aligned_size & 0x07 == 0, "Unaligned field!");
                                let mut xdata = vec![0u8; aligned_size as usize];
                                xdata_cursor.read_exact(&mut xdata);
                                match field.r#type {
                                    InoExtType::SnapXid => {
                                        assert_eq!(field.size, 8);
                                        let mut xvalue_cursor = Cursor::new(xdata);
                                        let xvalue = Xid::import(&mut xvalue_cursor).unwrap();
                                        println!("Snapshot Txid: {:?}", xvalue);
                                    },
                                    InoExtType::DeltaTreeOid => {
                                        assert_eq!(field.size, 8);
                                        let mut xvalue_cursor = Cursor::new(xdata);
                                        let xvalue = Oid::import(&mut xvalue_cursor).unwrap();
                                        println!("Delta Tree OID: {:?}", xvalue);
                                    },
                                    InoExtType::DocumentId => {
                                        assert_eq!(field.size, 4);
                                        let mut xvalue_cursor = Cursor::new(xdata);
                                        let xvalue = xvalue_cursor.read_u32::<LittleEndian>().unwrap();
                                        println!("Document ID: {}", xvalue);
                                    },
                                    InoExtType::Name => {
                                        // assert_eq!(field.size, 4);
                                        let xvalue = std::str::from_utf8(&xdata[0..field.size as usize]).unwrap();
                                        // let mut xvalue_cursor = Cursor::new(xdata);
                                        // let xvalue = xvalue_cursor.read_u32::<LittleEndian>().unwrap();
                                        println!("File name: {}", xvalue);
                                    },
                                    InoExtType::PrevFsize => {
                                        assert_eq!(field.size, 8);
                                        let mut xvalue_cursor = Cursor::new(xdata);
                                        let xvalue = xvalue_cursor.read_u64::<LittleEndian>().unwrap();
                                        println!("Previous file size: {}", xvalue);
                                    },
                                    InoExtType::FinderInfo => {
                                        assert_eq!(field.size, 32);
                                        // let mut xvalue_cursor = Cursor::new(xdata);
                                        // let xvalue = Oid::import(&mut xvalue_cursor).unwrap();
                                        println!("FinderInfo: {:?}", xdata);
                                    },
                                    InoExtType::Dstream => {
                                        // assert_eq!(field.size, 32);
                                        let mut xvalue_cursor = Cursor::new(xdata);
                                        let xvalue = JDstream::import(&mut xvalue_cursor).unwrap();
                                        println!("Dstream: {:?}", &xvalue);
                                        sizes.insert(key.obj_id_and_type.id(), xvalue.size);
                                    },
                                    _ => {},
                                }
                            }
                            println!("Fields: {:?}", &fields);
                        }
                    },
                    JObjTypes::DirRec => {
                        let value = JDrecVal::import(&mut value_cursor).unwrap();
                        println!("DirRec key: {:?}", JDrecHashedKey::import(&mut key_cursor).unwrap());
                        println!("DirRec: {:?}", &value);
                        if value.xfields.len() > 0 {
                            let mut xfields_cursor = Cursor::new(value.xfields);
                            let blob = XfBlob::import(&mut xfields_cursor)?;
                            let mut xdata_cursor = Cursor::new(blob.data);
                            let fields = (0..blob.num_exts).map(|_| XFieldDrec::import(&mut xdata_cursor).unwrap()).collect::<Vec<XFieldDrec>>();
                            for field in &fields {
                                assert!(field.size & 0x07 == 0, "Unaligned field!");
                                let mut xdata = vec![0u8; field.size as usize];
                                xdata_cursor.read_exact(&mut xdata);
                                match field.r#type {
                                    DrecExtType::DrecExtTypeSiblingId => {
                                        assert_eq!(field.size, 8);
                                        let mut xvalue_cursor = Cursor::new(xdata);
                                        let sibling_id = xvalue_cursor.read_u64::<LittleEndian>().unwrap();
                                        println!("Sibling ID: {}", sibling_id);
                                    }
                                }
                            }
                            println!("Fields: {:?}", &fields);
                        }
                    },
                    JObjTypes::Xattr => {
                        println!("Xattr key: {:?}", JXattrKey::import(&mut key_cursor).unwrap());
                        println!("Xattr: {:?}", JXattrVal::import(&mut value_cursor).unwrap());
                    },
                    JObjTypes::FileExtent => {
                        let value = JFileExtentVal::import(&mut value_cursor).unwrap();
                        println!("FileExtent key: {:?}", JFileExtentKey::import(&mut key_cursor).unwrap());
                        println!("FileExtent: {:?}", value);
                        let length = sizes[&key.obj_id_and_type.id()] as usize;
                        // let length = 12;
                        println!("Reading block: {} ({} bytes)", value.phys_block_num, length);
                        if let Ok(block) = apfs.load_block(Paddr(value.phys_block_num as i64)) {
                            println!("Body: '{}'", String::from_utf8((&block[0..length]).to_owned()).unwrap());
                        }
                    },
                    JObjTypes::DstreamId => {
                        println!("DstreamId key: {:?}", JDstreamIdKey::import(&mut key_cursor).unwrap());
                        println!("DstreamId: {:?}", JDstreamIdVal::import(&mut value_cursor).unwrap());
                    },
                    JObjTypes::SiblingLink => {
                        println!("SiblingLink key: {:?}", JSiblingKey::import(&mut key_cursor).unwrap());
                        println!("SiblingLink: {:?}", JSiblingKey::import(&mut value_cursor).unwrap());
                    },
                    JObjTypes::SiblingMap => {
                        println!("SiblingMap key: {:?}", JSiblingMapKey::import(&mut key_cursor).unwrap());
                        println!("SiblingMap: {:?}", JSiblingMapVal::import(&mut value_cursor).unwrap());
                    },
                    _ => {
                        println!("Unsupported key type: {:?}!", key_type);
                    },
                }
            }
        }
        let node = BtreeNode {
            node: body, records
        };
        Ok(BtreeDecodedObject::BtreeRoot(node, info))
    }

    pub fn load_btree<S: Read + Seek>(apfs: &mut APFS<S>, oid: Oid, r#type: StorageType) -> io::Result<Btree<K, V>> {
        let (root, info) = match Self::load_btree_node(apfs, oid, r#type)? {
            BtreeDecodedObject::BtreeRoot(body, info) => (body, info),
            _ => { unreachable!() },
        };
        Ok(Btree { info, root })
    }

    pub fn get_record(&self, key: OmapKey) -> io::Result<Record<OmapKey, OmapVal>> {
        Err(io::Error::new(io::ErrorKind::Other, ""))
    }
}
