#[macro_use]
extern crate bitflags;


#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    fn test_dir() -> PathBuf {
        let root = ::std::env::var_os("CARGO_MANIFEST_DIR").map(|x| PathBuf::from(x))
            .unwrap_or_else(|| ::std::env::current_dir().unwrap());
        root.join("testdata")
    }

    #[test]
    fn test_open_image() {
        let mut file_result = APFS::open(&test_dir().join("test-apfs.img"));
        assert!(file_result.is_ok());
    }

    #[test]
    fn test_open_nonexistent_image() {
        let mut file_result = APFS::open(&test_dir().join("nonexistent.img"));
        assert!(file_result.is_err());
    }

    #[test]
    fn test_load_block0() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let mut block_result = apfs.load_block(Oid(0));
        assert!(block_result.is_ok());
        let block = block_result.unwrap();
        assert_eq!(block.len(), 4096);
        let mut cursor = Cursor::new(&block[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        assert_eq!(header.o_cksum, fletcher64(&block[8..]), "cksum");
        assert_eq!(header.o_type & OBJECT_TYPE_MASK, OBJECT_TYPE_NX_SUPERBLOCK, "type");
        assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Ephemeral as u32, "type");
    }

    #[test]
    fn test_load_nonexistent_block() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let mut block_result = apfs.load_block(Oid(10000000));
        assert!(block_result.is_err());
    }

    #[test]
    fn test_load_block0_object() {
        let mut apfs = APFS::open(&test_dir().join("test-apfs.img")).unwrap();
        let object_result = apfs.load_object(Oid(0));
        assert!(object_result.is_ok());
        let object = object_result.unwrap();
        //assert_eq!(block.len(), 4096);
        //let mut cursor = Cursor::new(&block[..]);
        //let header = ObjPhys::import(&mut cursor).unwrap();
        //assert_eq!(header.o_cksum, fletcher64(&block[8..]), "cksum");
        //assert_eq!(header.o_type & OBJECT_TYPE_MASK, OBJECT_TYPE_NX_SUPERBLOCK, "type");
        //assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, OBJ_EPHEMERAL, "type");
    }
}

use std::fs::File;
use std::io::{self, prelude::*, Cursor, SeekFrom};
use std::path::Path;

#[macro_use]
mod int_strings;
mod internal;
mod fletcher;

use internal::*;
use fletcher::fletcher64;


struct NxSuperblockObject {
    header: ObjPhys,
    body: NxSuperblock,
}

struct CheckpointMapPhysObject {
    header: ObjPhys,
    body: CheckpointMapPhys,
}

enum APFSObject {
    Superblock(NxSuperblockObject),
    CheckpointMapping(CheckpointMapPhysObject),
}

struct APFS {
    file: File,
}

impl APFS {
    fn open(filename: &Path) -> io::Result<Self> {
        let file = File::open(filename)?;
        Ok(APFS { file })
    }

    fn load_block(&mut self, oid: Oid) -> io::Result<Vec<u8>> {
        let mut block = vec![0; 4096];
        self.file.seek(SeekFrom::Start((oid.0) * 4096))?;
        self.file.read_exact(&mut block)?;
        Ok(block)
    }

    fn load_object(&mut self, oid: Oid) -> io::Result<APFSObject> {
        let block = self.load_block(oid)?;
        let mut cursor = Cursor::new(&block[..]);
        let header = ObjPhys::import(&mut cursor)?;
        let body = NxSuperblock::import(&mut cursor)?;
        Ok(APFSObject::Superblock(NxSuperblockObject {
            header,
            body,
        }))
    }
}
