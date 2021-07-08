#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate bitflags;


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
        assert_eq!(header.cksum, fletcher64(&block[8..]), "cksum");
        assert_eq!(header.objtype & OBJECT_TYPE_MASK, ObjectType::NxSuperblock as u32, "type");
        assert_eq!(header.objtype & OBJECT_TYPE_FLAGS_MASK, StorageType::Ephemeral as u32, "type");
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
        //assert_eq!(header.o_cksum, fletcher64(&block[8..]), "cksum");
        //assert_eq!(header.o_type & OBJECT_TYPE_MASK, ObjectType::NxSuperblock as u32, "type");
        //assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, OBJ_EPHEMERAL, "type");
    }

    #[test]
    fn test_load_block0_bad_checksum() {
        let block = [0u8; 4096];
        let mut source = Cursor::new(&block[..]);
        let mut apfs = APFS { source };
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
        //assert_eq!(header.o_cksum, fletcher64(&block[8..]), "cksum");
        //assert_eq!(header.o_type & OBJECT_TYPE_MASK, ObjectType::NxSuperblock as u32, "type");
        //assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, OBJ_EPHEMERAL, "type");
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

use internal::*;
mod btree;
use fletcher::fletcher64;

pub use internal::Paddr;


//pub enum Node<K, R> {
//    //HeaderNode(HeaderNode),
//    //MapNode(MapNode),
//    IndexNode(IndexNode<K>),
//    //LeafNode(LeafNode<R>),
//}

struct Btree {
    body: BtreeNodeObject,
    info: BtreeInfo,
    records: Vec<u8>,
}


#[derive(Debug)]
pub struct NxSuperblockObject {
    header: ObjPhys,
    body: NxSuperblock,
}

#[derive(Debug)]
pub struct CheckpointMapPhysObject {
    header: ObjPhys,
    body: CheckpointMapPhys,
}

#[derive(Debug)]
pub struct ObjectMapObject {
    header: ObjPhys,
    body: OmapPhys,
}

#[derive(Debug)]
pub struct BtreeNodeObject {
    header: ObjPhys,
    body: BtreeNodePhys,
}

#[derive(Debug)]
pub enum APFSObject {
    Superblock(NxSuperblockObject),
    CheckpointMapping(CheckpointMapPhysObject),
    ObjectMap(ObjectMapObject),
    BtreeNode(BtreeNodeObject),
}

pub struct APFS<S: Read + Seek> {
    //file: File,
    source: S,
}

impl APFS<File> {
    pub fn open<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let source = File::open(filename)?;
        Ok(APFS { source })
    }
}

impl<S: Read + Seek> APFS<S> {
    fn load_block(&mut self, addr: Paddr) -> io::Result<Vec<u8>> {
        let mut block = vec![0; 4096];
        self.source.seek(SeekFrom::Start((addr.0 as u64) * 4096))?;
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
        let r#type = FromPrimitive::from_u32(header.objtype & OBJECT_TYPE_MASK);
        let object = match r#type {
            Some(ObjectType::NxSuperblock) =>
                APFSObject::Superblock(NxSuperblockObject {
                header,
                body: NxSuperblock::import(&mut cursor)?,
            }),
            Some(ObjectType::CheckpointMap) =>
                APFSObject::CheckpointMapping(CheckpointMapPhysObject {
                header,
                body: CheckpointMapPhys::import(&mut cursor)?,
            }),
            Some(ObjectType::Omap) =>
                APFSObject::ObjectMap(ObjectMapObject {
                header,
                body: OmapPhys::import(&mut cursor)?,
            }),
            Some(ObjectType::Btree) =>
                APFSObject::BtreeNode(BtreeNodeObject {
                header,
                body: BtreeNodePhys::import(&mut cursor)?,
            }),
            _ => { return Err(io::Error::new(io::ErrorKind::Other, format!("Unsupported type: {:?}", r#type))); },
        };
        Ok(object)
    }

    fn load_object_oid(&mut self, oid: Oid, r#type: StorageType) -> io::Result<APFSObject> {
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

    fn load_btree(&mut self, oid: Oid, r#type: StorageType) -> io::Result<Btree> {
        let object = self.load_object_oid(oid, r#type)?;
        let mut body = match object {
            APFSObject::BtreeNode(x) => x,
            _ => { panic!("Invalid type"); },
        };
        let info = BtreeInfo::import(&mut Cursor::new(&body.body.data[body.body.data.len()-40..]))?;
        body.body.data.truncate(body.body.data.len()-40);
        let toc = &body.body.data[body.body.table_space.off as usize..(body.body.table_space.off+body.body.table_space.len) as usize];
        let cursor = Cursor::new(toc);
        let records = vec![0];
        Ok(Btree { body, info, records })
    }
}
