#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_derive;


#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    pub fn test_dir() -> PathBuf {
        let root = ::std::env::var_os("CARGO_MANIFEST_DIR").map(|x| PathBuf::from(x))
            .unwrap_or_else(|| ::std::env::current_dir().unwrap());
        root.join("testdata")
    }

    #[test]
    fn test_open_image() {
        let file_result = APFS::open(&test_dir().join("test-apfs.img"));
        assert!(file_result.is_ok());
    }

    #[test]
    fn test_open_nonexistent_image() {
        let file_result = APFS::open(&test_dir().join("nonexistent.img"));
        assert!(file_result.is_err());
    }

    #[test]
    fn test_load_block0() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let mut block_result = apfs.load_block(Paddr(0));
        assert!(block_result.is_ok());
        let block = block_result.unwrap();
        assert_eq!(block.len(), 4096);
        let mut cursor = Cursor::new(&block[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        assert_eq!(header.cksum, fletcher64(&block[8..]), "bad checksum");
        assert_eq!(header.r#type.r#type(), ObjectType::NxSuperblock, "type");
        assert_eq!(header.r#type.storage(), StorageType::Ephemeral, "type");
    }

    
    #[test]
    #[cfg_attr(not(feature = "expensive_tests"), ignore)]
    fn test_load_block0_16k() {
        let mut apfs = APFS::open(&test_dir().join("apfs-16k-cs.img")).unwrap();
        let mut block_result = apfs.load_block(Paddr(0));
        assert!(block_result.is_ok());
        let block = block_result.unwrap();
        assert_eq!(block.len(), 16384, "bad block size");
        let mut cursor = Cursor::new(&block[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        assert_eq!(header.cksum, fletcher64(&block[8..]), "bad checksum");
        assert_eq!(header.r#type.r#type(), ObjectType::NxSuperblock, "type");
        assert_eq!(header.r#type.storage(), StorageType::Ephemeral, "type");
    }
    

    #[test]
    fn test_load_nonexistent_block() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let block_result = apfs.load_block(Paddr(10000000));
        assert!(block_result.is_err());
    }

    #[test]
    fn test_load_block0_object() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let object_result = apfs.load_object_addr(Paddr(0));
        assert!(object_result.is_ok());
        let object = object_result.unwrap();
        let superblock = match object {
            APFSObject::Superblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        assert_eq!(superblock.body.block_size, 4096);
        //let mut cursor = Cursor::new(&block[..]);
        //let header = ObjPhys::import(&mut cursor).unwrap();
        //assert_eq!(header.o_cksum, fletcher64(&block[8..]), "bad checksum");
        //assert_eq!(header.o_type.r#type(), ObjectType::NxSuperblock, "type");
        //assert_eq!(header.o_type.storage(), OBJ_EPHEMERAL, "type");
    }

    #[test]
    fn test_load_block0_bad_checksum() {
        let block = [0u8; NX_DEFAULT_BLOCK_SIZE];
        let mut source = Cursor::new(&block[..]);
        let mut apfs = APFS { source, block_size: NX_DEFAULT_BLOCK_SIZE };
        let object_result = apfs.load_object_addr(Paddr(0));
        assert!(object_result.is_err(), "failed to detect bad checksum");
    }

    #[test]
    fn test_load_checkpoint_descriptors() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let object = apfs.load_object_addr(Paddr(0)).unwrap();
        let superblock = match object {
            APFSObject::Superblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        let object_result = apfs.load_object_addr(superblock.body.xp_desc_base);
        assert!(object_result.is_ok(), "Bad checkpoint object load");
        let object = object_result.unwrap();
        let mapping = match object {
            APFSObject::CheckpointMapping(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        for idx in 0..superblock.body.xp_desc_blocks {
            let addr = superblock.body.xp_desc_base.0 + idx as i64;
            let object_result = apfs.load_object_addr(Paddr(addr));
            assert!(object_result.is_ok(), "Bad checkpoint object load");
        }
        //let mut cursor = Cursor::new(&block[..]);
        //let header = ObjPhys::import(&mut cursor).unwrap();
        //assert_eq!(header.o_cksum, fletcher64(&block[8..]), "bad checksum");
        //assert_eq!(header.o_type.r#type(), ObjectType::NxSuperblock, "type");
        //assert_eq!(header.o_type.storage(), OBJ_EPHEMERAL, "type");
    }
}

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, prelude::*, Cursor, SeekFrom};
use std::path::Path;

use num_traits::FromPrimitive;

#[macro_use]
mod int_strings;
mod internal;
mod fletcher;

pub use internal::*;
mod btree;
use fletcher::fletcher64;

pub use internal::Paddr;


//pub enum Node<K, R> {
//    //HeaderNode(HeaderNode),
//    //MapNode(MapNode),
//    IndexNode(IndexNode<K>),
//    //LeafNode(LeafNode<R>),
//}

#[derive(Debug)]
pub struct Btree {
    body: BtreeNodeObject,
    info: BtreeInfo,
    pub records: Vec<Record<OmapKey, OmapVal>>,
}


#[derive(Debug)]
pub struct NxSuperblockObject {
    header: ObjPhys,
    pub body: NxSuperblock,
}

#[derive(Debug)]
pub struct CheckpointMapPhysObject {
    header: ObjPhys,
    body: CheckpointMapPhys,
}

#[derive(Debug)]
pub struct ObjectMapObject {
    header: ObjPhys,
    pub body: OmapPhys,
}

#[derive(Debug)]
pub struct BtreeNodeObject {
    header: ObjPhys,
    body: BtreeNodePhys,
}

#[derive(Debug)]
pub struct ApfsSuperblockObject {
    header: ObjPhys,
    pub body: ApfsSuperblock,
}

#[derive(Debug)]
pub enum APFSObject {
    Superblock(NxSuperblockObject),
    CheckpointMapping(CheckpointMapPhysObject),
    ObjectMap(ObjectMapObject),
    BtreeNode(BtreeNodeObject),
    ApfsSuperblock(ApfsSuperblockObject),
}

pub struct APFS<S: Read + Seek> {
    //file: File,
    source: S,
    block_size: usize,
}

impl APFS<File> {
    pub fn open<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let mut source = File::open(filename)?;
        //let block0 = APFS { source, block_size: NX_DEFAULT_BLOCK_SIZE }.load_block(Paddr(0)).unwrap();
        let mut block0 = [0; NX_DEFAULT_BLOCK_SIZE];
        source.read_exact(&mut block0[..])?;
        let mut cursor = Cursor::new(&block0[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        let superblock = NxSuperblock::import(&mut cursor).unwrap();
        Ok(APFS { source, block_size: superblock.block_size as usize })
    }
}

use btree::Record;

impl<S: Read + Seek> APFS<S> {
    fn load_block(&mut self, addr: Paddr) -> io::Result<Vec<u8>> {
        let mut block = vec![0; self.block_size];
        self.source.seek(SeekFrom::Start((addr.0 as u64) * self.block_size as u64))?;
        self.source.read_exact(&mut block)?;
        Ok(block)
    }

    pub fn load_object_addr(&mut self, addr: Paddr) -> io::Result<APFSObject> {
        let block = self.load_block(addr)?;
        let mut cursor = Cursor::new(&block[..]);
        let header = ObjPhys::import(&mut cursor)?;
        if header.cksum != fletcher64(&block[8..]) {
            return Err(io::Error::new(io::ErrorKind::Other, "Bad object checksum"));
        }
        let object = match header.r#type.r#type() {
            ObjectType::NxSuperblock =>
                APFSObject::Superblock(NxSuperblockObject {
                header,
                body: NxSuperblock::import(&mut cursor)?,
            }),
            ObjectType::CheckpointMap =>
                APFSObject::CheckpointMapping(CheckpointMapPhysObject {
                header,
                body: CheckpointMapPhys::import(&mut cursor)?,
            }),
            ObjectType::Omap =>
                APFSObject::ObjectMap(ObjectMapObject {
                header,
                body: OmapPhys::import(&mut cursor)?,
            }),
            ObjectType::Btree =>
                APFSObject::BtreeNode(BtreeNodeObject {
                header,
                body: BtreeNodePhys::import(&mut cursor)?,
            }),
            ObjectType::Fs =>
                APFSObject::ApfsSuperblock(ApfsSuperblockObject {
                header,
                body: ApfsSuperblock::import(&mut cursor)?,
            }),
            _ => { return Err(io::Error::new(io::ErrorKind::Other, format!("Unsupported type: {:?}", header.r#type.r#type()))); },
        };
        Ok(object)
    }

    pub fn load_object_oid(&mut self, oid: Oid, r#type: StorageType) -> io::Result<APFSObject> {
        Ok(match r#type {
            StorageType::Physical => {
                self.load_object_addr(Paddr(oid.0 as i64))?
            },
            _ => {
                panic!("Unsupported");
            },
        })
    }

    /*
    fn load_btree_node(&mut self, oid: Oid, r#type: StorageType) -> io::Result<Btree> {
        let object = self.load_object_oid(oid, r#type)?;
        let body = match object {
            APFSObject::BtreeNode(x) => x,
            _ => { panic!("Invalid type"); },
        };
        Ok(Btree { body })
    }
    */

    pub fn load_btree(&mut self, oid: Oid, r#type: StorageType) -> io::Result<Btree> {
        let object = self.load_object_oid(oid, r#type)?;
        let mut body = match object {
            APFSObject::BtreeNode(x) => x,
            _ => { panic!("Invalid type"); },
        };
        let info = BtreeInfo::import(&mut Cursor::new(&body.body.data[body.body.data.len()-40..]))?;
        body.body.data.truncate(body.body.data.len()-40);
        let toc = &body.body.data[body.body.table_space.off as usize..(body.body.table_space.off+body.body.table_space.len) as usize];
        let mut cursor = Cursor::new(toc);
        let mut items = vec![];
        assert!(body.body.flags.contains(BtnFlags::LEAF));
        let mut records = vec![];
        for _ in 0..body.body.nkeys {
            items.push(KVoff::import(&mut cursor)?);
            let key_data = &body.body.data[(body.body.table_space.off+body.body.table_space.len+items.last().unwrap().k) as usize..(body.body.table_space.off+body.body.table_space.len+items.last().unwrap().k + info.fixed.key_size as u16) as usize];
            let mut c2 = Cursor::new(key_data);
            let key = OmapKey::import(&mut c2)?;
            let val_data = &body.body.data[(body.body.data.len() as u16 - items.last().unwrap().v) as usize..(body.body.data.len() as u16 -  items.last().unwrap().v + info.fixed.val_size as u16) as usize];
            let mut c2 = Cursor::new(val_data);
            let val = OmapVal::import(&mut c2)?;
            let record = Record {
                key,
                value: val,
            };
            records.push(record);
        }
        Ok(Btree { body, info, records })
    }
}
