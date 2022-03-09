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
    pub oid: Oid,
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
    pub value: OidValue,
}

#[cfg(test)]
mod test;

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
pub enum AnyRecord<'a, V: LeafValue> {
    Leaf(&'a LeafRecord<V>),
    NonLeaf(&'a NonLeafRecord<V::Key>, PhantomData<V>),
}

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

impl BtreeNode<OmapVal> {
    fn get_record<'a>(&'a self, key: <OmapVal as LeafValue>::Key) -> Option<AnyRecord<'a, OmapVal>> {
        Some(match self.records {
            AnyRecords::Leaf(ref x) => {
                return x.into_iter().filter(|y| y.key.oid == key.oid).nth(0).map(|y| AnyRecord::Leaf(y));
            },
            AnyRecords::NonLeaf(ref x, _) => AnyRecord::NonLeaf(x.get(0).unwrap(), PhantomData),
        })
    }
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

    pub fn load_btree_node<S: Read + Seek>(&self, apfs: &mut APFS<S>, oid: Oid, r#type: StorageType) -> io::Result<BtreeNode<V>> {
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
