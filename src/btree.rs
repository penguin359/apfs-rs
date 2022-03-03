use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, prelude::*};
use std::io::Cursor;
use std::marker::PhantomData;
use std::ops::RangeBounds;
use std::fmt::Debug;

use byteorder::{LittleEndian, ReadBytesExt, BigEndian};
use num_traits::FromPrimitive;

use crate::{BtreeNodePhys, KVoff, ObjectType, JObjTypes, JDrecHashedKey, JInodeKey, JInodeVal, JDrecVal, JXattrVal, JXattrKey, JFileExtentKey, JFileExtentVal, JDstreamIdKey, JDstreamIdVal, JSiblingKey, JSiblingMapKey, JSiblingMapVal, XfBlob, XFieldDrec, DrecExtType, XFieldInode, InoExtType, JDstream, JDrecKey, JSiblingVal};
use crate::internal::{KVloc, Nloc};
use crate::internal::Oid;
use crate::internal::Xid;
use crate::internal::{BtnFlags, OmapKey};
use crate::internal::OmapVal;
use crate::internal::OvFlags;
use crate::internal::BtreeInfo;
use crate::internal::JKey;

use crate::{APFS, APFSObject, BtreeNodeObject, Paddr, StorageType};

pub trait Key : PartialOrd + Ord + PartialEq + Eq + Debug + Sized {
    fn import(source: &mut dyn Read) -> io::Result<Self>;
}

pub trait Value : Debug + Sized {
}

pub trait LeafValue : Value {
    type Key: Key;

    fn import(source: &mut dyn Read, key: &Self::Key) -> io::Result<Self>;
}

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
}

impl Value for OmapVal {}

impl LeafValue for OmapVal {
    type Key = OmapKey;

    fn import(source: &mut dyn Read, _: &Self::Key) -> io::Result<Self> {
        Ok(Self::import(source)?)
    }
}

#[derive(Debug)]
pub enum ApfsSubKey {
    None,
    Name(String),
    DrecHashed(JDrecHashedKey),
    FileExtent(JFileExtentKey),
    SiblingLink(JSiblingKey),
}

#[derive(Debug)]
pub struct ApfsKey {
    pub key: JKey,
    pub subkey: ApfsSubKey,
}

/* Object map keys match on the object ID equality and
   a transaction ID that is less than or equal */
impl Ord for ApfsKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.key.obj_id_and_type.id().cmp(&other.key.obj_id_and_type.id());
        match order {
            Ordering::Equal => {
                let order = (self.key.obj_id_and_type.r#type() as u64).cmp(&(other.key.obj_id_and_type.r#type() as u64));
                match order {
                    Ordering::Equal => match (&self.subkey, &other.subkey) {
                        (ApfsSubKey::Name(ref left), ApfsSubKey::Name(ref right)) => left.cmp(right),
                        _ => Ordering::Equal,
                    },
                    _ => order,
                }
            },
            _ => order,
        }
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
    fn import(key_cursor: &mut dyn Read) -> io::Result<Self> {
        let key = JKey::import(key_cursor)?;
        let key_type = key.obj_id_and_type.r#type();
        println!("Key type: {:?}", key);
        match key_type {
            JObjTypes::Inode => {
                println!("Inode key: {:?}", JInodeKey::import(key_cursor)?);
                return Ok(ApfsKey {
                    key: key,
                    subkey: ApfsSubKey::None,
                });
            },
            JObjTypes::DirRec => {
                let subkey = JDrecHashedKey::import(key_cursor)?;
                println!("DirRec key: {:?}", &subkey);
                return Ok(ApfsKey {
                    key: key,
                    subkey: ApfsSubKey::DrecHashed(subkey),
                });
            },
            JObjTypes::Xattr => {
                let subkey = JXattrKey::import(key_cursor)?;
                println!("Xattr key: {:?}", &subkey);
                return Ok(ApfsKey {
                    key: key,
                    subkey: ApfsSubKey::Name(subkey.name),
                });
            },
            JObjTypes::FileExtent => {
                let subkey = JFileExtentKey::import(key_cursor)?;
                println!("FileExtent key: {:?}", &subkey);
                return Ok(ApfsKey {
                    key: key,
                    subkey: ApfsSubKey::FileExtent(subkey),
                });
            },
            JObjTypes::DstreamId => {
                let subkey = JDstreamIdKey::import(key_cursor)?;
                println!("DstreamId key: {:?}", &subkey);
                return Ok(ApfsKey {
                    key: key,
                    subkey: ApfsSubKey::None,
                });
            },
            JObjTypes::SiblingLink => {
                let subkey = JSiblingKey::import(key_cursor)?;
                println!("SiblingLink key: {:?}", &subkey);
                return Ok(ApfsKey {
                    key: key,
                    subkey: ApfsSubKey::SiblingLink(subkey),
                });
            },
            JObjTypes::SiblingMap => {
                let subkey = JSiblingMapKey::import(key_cursor)?;
                println!("SiblingMap key: {:?}", &subkey);
                return Ok(ApfsKey {
                    key: key,
                    subkey: ApfsSubKey::None,
                });
            },
            _ => {
                println!("Unsupported key type: {:?}!", key_type);
            },
        }
        return Err(io::Error::new(io::ErrorKind::Unsupported, "Unrecognized node type"));
    }
}

pub trait Record: Debug {
    type Key: Key;
    type Value: Value;

    fn key(&self) -> &Self::Key;

    fn value(&self) -> &Self::Value;
}

#[derive(Debug)]
pub struct LeafRecord<V> where
    V: LeafValue {

    pub key: V::Key,
    pub value: V,
}

impl<V> Record for LeafRecord<V> where
    V: LeafValue {
    type Key = V::Key;
    type Value = V;

    fn key(&self) -> &Self::Key {
        &self.key
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

pub type OmapRecord = LeafRecord<OmapVal>;

#[derive(Debug)]
pub enum InodeXdata {
    SnapXid(Xid),
    DeltaTreeOid(Oid),
    DocumentId(u32),
    Name(String),
    PrevFsize(u64),
    FinderInfo([u8; 32]),
    Dstream(JDstream),
}

#[derive(Debug)]
pub struct InodeValue {
    value: JInodeVal,
    pub xdata: HashMap<InoExtType, InodeXdata>,
}

#[derive(Debug)]
pub enum DrecXdata {
    SiblingId(u64),
}

#[derive(Debug)]
pub struct DrecValue {
    value: JDrecVal,
    pub xdata: HashMap<DrecExtType, DrecXdata>,
}

#[derive(Debug)]
pub enum ApfsValue {
    Inode(InodeValue),
    Drec(DrecValue),
    Xattr(JXattrVal),
    FileExtent(JFileExtentVal),
    DstreamId(JDstreamIdVal),
    SiblingLink(JSiblingVal),
    SiblingMap(JSiblingMapVal),
}

impl Value for ApfsValue {}

impl LeafValue for ApfsValue {
    type Key = ApfsKey;

    fn import(value_cursor: &mut dyn Read, key: &Self::Key) -> io::Result<Self> {
        let key_type = key.key.obj_id_and_type.r#type();
        println!("Key type: {:?}", key);
        match key_type {
            JObjTypes::Inode => {
                let value = JInodeVal::import(value_cursor)?;
                println!("Inode: {:?}", &value);
                let mut xdata_map = HashMap::new();
                if value.xfields.len() > 0 {
                    let mut xfields_cursor = Cursor::new(&value.xfields);
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
                                xdata_map.insert(field.r#type, InodeXdata::SnapXid(xvalue));
                            },
                            InoExtType::DeltaTreeOid => {
                                assert_eq!(field.size, 8);
                                let mut xvalue_cursor = Cursor::new(xdata);
                                let xvalue = Oid::import(&mut xvalue_cursor).unwrap();
                                println!("Delta Tree OID: {:?}", xvalue);
                                xdata_map.insert(field.r#type, InodeXdata::DeltaTreeOid(xvalue));
                            },
                            InoExtType::DocumentId => {
                                assert_eq!(field.size, 4);
                                let mut xvalue_cursor = Cursor::new(xdata);
                                let xvalue = xvalue_cursor.read_u32::<LittleEndian>().unwrap();
                                println!("Document ID: {}", xvalue);
                                xdata_map.insert(field.r#type, InodeXdata::DocumentId(xvalue));
                            },
                            InoExtType::Name => {
                                // assert_eq!(field.size, 4);
                                let xvalue = std::str::from_utf8(&xdata[0..field.size as usize]).unwrap();
                                // let mut xvalue_cursor = Cursor::new(xdata);
                                // let xvalue = xvalue_cursor.read_u32::<LittleEndian>().unwrap();
                                println!("File name: {}", xvalue);
                                xdata_map.insert(field.r#type, InodeXdata::Name(xvalue.to_owned()));
                            },
                            InoExtType::PrevFsize => {
                                assert_eq!(field.size, 8);
                                let mut xvalue_cursor = Cursor::new(xdata);
                                let xvalue = xvalue_cursor.read_u64::<LittleEndian>().unwrap();
                                println!("Previous file size: {}", xvalue);
                                xdata_map.insert(field.r#type, InodeXdata::PrevFsize(xvalue));
                            },
                            InoExtType::FinderInfo => {
                                assert_eq!(field.size, 32);
                                // let mut xvalue_cursor = Cursor::new(xdata);
                                // let xvalue = Oid::import(&mut xvalue_cursor).unwrap();
                                println!("FinderInfo: {:?}", xdata);
                                xdata_map.insert(field.r#type, InodeXdata::FinderInfo(xdata[0..32].try_into().unwrap()));
                            },
                            InoExtType::Dstream => {
                                let mut xvalue_cursor = Cursor::new(xdata);
                                let xvalue = JDstream::import(&mut xvalue_cursor).unwrap();
                                println!("Dstream: {:?}", &xvalue);
                                xdata_map.insert(field.r#type, InodeXdata::Dstream(xvalue));
                            },
                            _ => {},
                        }
                    }
                    println!("Fields: {:?}", &fields);
                }
                return Ok(ApfsValue::Inode(InodeValue {
                    value,
                    xdata: xdata_map,
                }));
            },
            JObjTypes::DirRec => {
                let value = JDrecVal::import(value_cursor)?;
                println!("DirRec: {:?}", &value);
                let mut xdata_map = HashMap::new();
                if value.xfields.len() > 0 {
                    let mut xfields_cursor = Cursor::new(&value.xfields);
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
                                xdata_map.insert(field.r#type, DrecXdata::SiblingId(sibling_id));
                            }
                        }
                    }
                    println!("Fields: {:?}", &fields);
                }
                return Ok(ApfsValue::Drec(DrecValue {
                    value,
                    xdata: xdata_map,
                }));
            },
            JObjTypes::Xattr => {
                let value = JXattrVal::import(value_cursor).unwrap();
                println!("Xattr: {:?}", &value);
                return Ok(ApfsValue::Xattr(value));
            },
            JObjTypes::FileExtent => {
                let value = JFileExtentVal::import(value_cursor)?;
                println!("FileExtent: {:?}", &value);
                // let length = sizes[&key.obj_id_and_type.id()] as usize;
                // // let length = 12;
                // println!("Reading block: {} ({} bytes)", value.phys_block_num, length);
                // if let Ok(block) = apfs.load_block(Paddr(value.phys_block_num as i64)) {
                //     println!("Body: '{}'", String::from_utf8((&block[0..length]).to_owned()).unwrap());
                // }
                return Ok(ApfsValue::FileExtent(value));
            },
            JObjTypes::DstreamId => {
                let value = JDstreamIdVal::import(value_cursor)?;
                println!("DstreamId: {:?}", &value);
                return Ok(ApfsValue::DstreamId(value));
            },
            JObjTypes::SiblingLink => {
                let value = JSiblingVal::import(value_cursor)?;
                println!("SiblingLink: {:?}", &value);
                return Ok(ApfsValue::SiblingLink(value));
            },
            JObjTypes::SiblingMap => {
                let value = JSiblingMapVal::import(value_cursor)?;
                println!("SiblingMap: {:?}", &value);
                return Ok(ApfsValue::SiblingMap(value));
            },
            _ => {
                println!("Unsupported key type: {:?}!", key_type);
            },
        }
        return Err(io::Error::new(io::ErrorKind::Unsupported, "Unrecognized node type"));
    }
}

#[derive(Debug)]
pub struct OidValue {
    oid: Oid,
}

impl OidValue {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(OidValue { oid: Oid::import(source)? })
    }
}

impl Value for OidValue {}

#[derive(Debug)]
pub struct NonLeafRecord<K: Key> {
    key: K,
    value: OidValue,
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

    #[test]
    // #[ignore = "test failing, developing smaller, more focused tests first"]
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

    use crate::{tests::test_dir, JObjectIdAndType};

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
        let mut source = File::open(&test_dir().join("btree.blob")).expect("Unable to load blob");
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
        let mut source = File::open(&test_dir().join("object-map-root-nonleaf.blob")).expect("Unable to load blob");
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
        let mut source = File::open(&test_dir().join("object-map-root-nonleaf.blob")).expect("Unable to load blob");
        let mut apfs = APFS { source, block_size: 4096 };
        let btree_result = Btree::<OmapVal>::load_btree(&mut apfs, Oid(0), StorageType::Physical);
        let btree = btree_result.unwrap();
        let mut source = File::open(&test_dir().join("object-map-nonroot-nonleaf.blob")).expect("Unable to load blob");
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
pub enum AnyRecords<V: LeafValue> {
    Leaf(Vec<LeafRecord<V>>),
    NonLeaf(Vec<NonLeafRecord<V::Key>>, PhantomData<V>),
}

#[derive(Debug)]
pub struct Btree<V: LeafValue> {
    info: BtreeInfo,
    pub root: BtreeNode<V>,
    _v: PhantomData<V>,
}

#[derive(Debug)]
pub struct BtreeNode<V: LeafValue> {
    node: BtreeNodeObject,
    pub records: AnyRecords<V>,
    _v: PhantomData<V>,
}

enum BtreeRawObject {
    BtreeRoot(BtreeNodeObject, BtreeInfo),
    BtreeNonRoot(BtreeNodeObject),
}

impl<V> Btree<V> where
    V: LeafValue {
    fn load_btree_object<S: Read + Seek>(apfs: &mut APFS<S>, oid: Oid, r#type: StorageType) -> io::Result<BtreeRawObject> {
        let object = apfs.load_object_oid(oid, r#type)?;
        let body = match object {
            APFSObject::Btree(mut body) => {
                let info = BtreeInfo::import(&mut Cursor::new(&body.body.data[body.body.data.len()-40..]))?;
                body.body.data.truncate(body.body.data.len()-40);
                BtreeRawObject::BtreeRoot(body, info)
            },
            APFSObject::BtreeNode(body) => BtreeRawObject::BtreeNonRoot(body),
            _ => { panic!("Invalid type"); },
        };
        Ok(body)
    }

    fn decode_btree_node(body: BtreeNodeObject, info: &BtreeInfo) -> io::Result<BtreeNode<V>> {
        if body.header.subtype.r#type() != ObjectType::Omap &&
           body.header.subtype.r#type() != ObjectType::Fstree {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported B-tree type"));
        }
        let toc = &body.body.data[body.body.table_space.off as usize..(body.body.table_space.off+body.body.table_space.len) as usize];
        let mut cursor = Cursor::new(toc);
        let mut items = vec![];
        let mut records = vec![];
        let mut nrecords = vec![];
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
                        len: if body.body.flags.contains(BtnFlags::LEAF) {
                            info.fixed.key_size as u16
                        } else {
                            8  // Length of OidValue
                        },
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
            let key = V::Key::import(&mut key_cursor)?;
            if body.body.flags.contains(BtnFlags::LEAF) {
                if body.header.subtype.r#type() == ObjectType::Omap  {
                    let value = V::import(&mut value_cursor, &key)?;
                    let record = LeafRecord {
                        key,
                        value,
                    };
                    records.push(record);
                } else if(body.header.subtype.r#type() == ObjectType::Fstree) {
                    if let Ok(key) = V::Key::import(&mut key_cursor) {
                        let value = V::import(&mut value_cursor, &key)?;
                        let record = LeafRecord {
                            key,
                            value,
                        };
                        records.push(record);
                    }
                }
            } else {
                nrecords.push(NonLeafRecord {
                    key,
                    value: OidValue::import(&mut value_cursor)?,
                });
            }
        }
        let node = if body.body.flags.contains(BtnFlags::LEAF) {
            BtreeNode {
                node: body, records: AnyRecords::Leaf(records), _v: PhantomData
            }
        } else {
            BtreeNode {
                node: body, records: AnyRecords::NonLeaf(nrecords, PhantomData), _v: PhantomData
            }
        };
        Ok(node)
    }

    fn load_btree_node<S: Read + Seek>(&self, apfs: &mut APFS<S>, oid: Oid, r#type: StorageType) -> io::Result<BtreeNode<V>> {
        let body = match Self::load_btree_object(apfs, oid, r#type)? {
            BtreeRawObject::BtreeNonRoot(body) => body,
            _ => { return Err(io::Error::new(io::ErrorKind::InvalidData, "Root node as a descendent in tree")); },
        };
        let node = Self::decode_btree_node(body, &self.info)?;
        Ok(node)
    }

    pub fn load_btree<S: Read + Seek>(apfs: &mut APFS<S>, oid: Oid, r#type: StorageType) -> io::Result<Btree<V>> {
        let (body, info) = match Self::load_btree_object(apfs, oid, r#type)? {
            BtreeRawObject::BtreeRoot(body, info) => (body, info),
            _ => { return Err(io::Error::new(io::ErrorKind::InvalidData, "Non-root node at top of tree")); },
        };
        let root = Self::decode_btree_node(body, &info)?;
        Ok(Btree { info, root, _v: PhantomData })
    }

    pub fn get_record(&self, key: OmapKey) -> io::Result<OmapRecord> {
        Err(io::Error::new(io::ErrorKind::Other, ""))
    }
}
