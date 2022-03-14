#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_derive;


#[cfg(test)]
mod tests {
    use std::{path::PathBuf, collections::HashMap};

    use super::*;

    pub struct DummySource {
        pub position: u64,
        pub block_size: u64,
        pub blocks: HashMap<u64, Vec<u8>>,
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

    pub fn test_dir() -> PathBuf {
        let root = ::std::env::var_os("CARGO_MANIFEST_DIR").map(|x| PathBuf::from(x))
            .unwrap_or_else(|| ::std::env::current_dir().unwrap());
        root.join("testdata")
    }

    #[test]
    fn test_open_image() {
        let file_result = APFS::open(test_dir().join("test-apfs.img"));
        assert!(file_result.is_ok());
    }

    #[test]
    fn test_open_nonexistent_image() {
        let file_result = APFS::open(test_dir().join("nonexistent.img"));
        assert!(file_result.is_err());
    }

    pub const TEST_APFS_FILE : &str = "test-apfs.img";
    pub const TEST_16KB_APFS_FILE : &str = "apfs-16k-cs.img";

    fn load_test_apfs_image(file: &str) -> APFS<File> {
        APFS::open(test_dir().join(file)).unwrap()
    }

    #[test]
    fn test_load_block0() {
        let mut apfs = load_test_apfs_image(TEST_APFS_FILE);
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
        let mut apfs = load_test_apfs_image(TEST_16KB_APFS_FILE);
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
        let mut apfs = load_test_apfs_image(TEST_APFS_FILE);
        let block_result = apfs.load_block(Paddr(10000000));
        assert!(block_result.is_err());
    }

    pub fn load_test_apfs_superblock(file: &str) -> (APFS<File>, NxSuperblockObject) {
        let mut apfs = load_test_apfs_image(file);
        let object_result = apfs.load_object_addr(Paddr(0));
        assert!(object_result.is_ok());
        let object = object_result.unwrap();
        let superblock = match object {
            APFSObject::Superblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        (apfs, superblock)
    }

    #[test]
    fn test_load_block0_object() {
        let (_, superblock) = load_test_apfs_superblock(TEST_APFS_FILE);
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
        let (mut apfs, superblock) = load_test_apfs_superblock(TEST_APFS_FILE);
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

    fn load_test_apfs_checkpoints() -> APFS<DummySource> {
        const BLOCK_SIZE: usize = 4096;

        let mut source = File::open(test_dir().join(TEST_APFS_FILE)).expect("Unable to load blob");
        let mut block = vec![0u8; BLOCK_SIZE];
        let mut dummy_source = DummySource {
            position: 0,
            block_size: BLOCK_SIZE as u64,
            blocks: HashMap::new(),
        };
        for idx in 0..60 {
            source.read_exact(&mut block).unwrap();
            dummy_source.blocks.insert(idx, block.clone());
        }
        APFS { source: dummy_source, block_size: BLOCK_SIZE }
    }

    #[test]
    fn can_load_test_apfs_checkpoints() {
        let apfs = load_test_apfs_checkpoints();
    }

    #[test]
    fn can_mount_test_apfs_checkpoints() {
        let apfs = load_test_apfs_checkpoints();
        let mount = APFSMount::mount(apfs).unwrap();
        assert_eq!(mount.superblock.header.oid, Oid(1));
        assert_eq!(mount.superblock.header.r#type.r#type(), ObjectType::NxSuperblock);
        assert_eq!(mount.superblock.header.r#type.storage(), StorageType::Ephemeral);
        assert!(mount.superblock.header.r#type.flags().is_empty());
        assert_eq!(mount.superblock.body.magic, NX_MAGIC);
    }

    #[test]
    fn mounting_test_apfs_has_correct_superblock() {
        let apfs = load_test_apfs_checkpoints();
        let mount = APFSMount::mount(apfs).unwrap();
        assert_eq!(mount.superblock.header.oid, Oid(1));
        assert_eq!(mount.superblock.header.r#type.r#type(), ObjectType::NxSuperblock);
        assert_eq!(mount.superblock.header.r#type.storage(), StorageType::Ephemeral);
        assert!(mount.superblock.header.r#type.flags().is_empty());
        assert_eq!(mount.superblock.header.xid, Xid(4));
        assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumSet as usize], 35);
        assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumFail as usize], 0);
    }

    fn rotate_checkpoint_descriptor(apfs: &mut APFS<DummySource>, rotations: u8) {
        // Rotate the 8 blocks that make up the checkpoint descriptor
        for _ in 0..rotations {
            let tmp = apfs.source.blocks.remove(&1).unwrap();
            for idx in 1..8 {
                let next = apfs.source.blocks.remove(&(idx+1)).unwrap();
                apfs.source.blocks.insert(idx, next);
            }
            apfs.source.blocks.insert(8, tmp);
        }
    }

    fn corrupt_descriptor_block(apfs: &mut APFS<DummySource>, block: u64) {
        apfs.source.blocks.get_mut(&block).unwrap()[0] = 0xff;
    }

    #[test]
    fn mounting_test_apfs_has_correct_superblock_when_rotated() {
        for rotations in 1..7 {
            let mut apfs = load_test_apfs_checkpoints();
            rotate_checkpoint_descriptor(&mut apfs, rotations);
            let mount = APFSMount::mount(apfs).unwrap();
            assert_eq!(mount.superblock.header.oid, Oid(1));
            assert_eq!(mount.superblock.header.r#type.r#type(), ObjectType::NxSuperblock);
            assert_eq!(mount.superblock.header.r#type.storage(), StorageType::Ephemeral);
            assert!(mount.superblock.header.r#type.flags().is_empty());
            assert_eq!(mount.superblock.header.xid, Xid(4));
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumSet as usize], 35);
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumFail as usize], 0);
        }
    }

    #[test]
    fn mounting_test_apfs_when_other_superblocks_corrupted() {
        for rotations in 0..7 {
            let mut apfs = load_test_apfs_checkpoints();
            for idx in 1..7 {
                corrupt_descriptor_block(&mut apfs, idx);
            }
            rotate_checkpoint_descriptor(&mut apfs, rotations);
            let mount = APFSMount::mount(apfs).unwrap();
            assert_eq!(mount.superblock.header.oid, Oid(1));
            assert_eq!(mount.superblock.header.r#type.r#type(), ObjectType::NxSuperblock);
            assert_eq!(mount.superblock.header.r#type.storage(), StorageType::Ephemeral);
            assert!(mount.superblock.header.r#type.flags().is_empty());
            assert_eq!(mount.superblock.header.xid, Xid(4));
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumSet as usize], 35);
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumFail as usize], 0);
        }
    }

    #[test]
    fn mounting_test_apfs_when_latest_superblock_corrupted() {
        for rotations in 0..7 {
            let mut apfs = load_test_apfs_checkpoints();
            corrupt_descriptor_block(&mut apfs, 8);
            rotate_checkpoint_descriptor(&mut apfs, rotations);
            let mount = APFSMount::mount(apfs).unwrap();
            assert_eq!(mount.superblock.header.oid, Oid(1));
            assert_eq!(mount.superblock.header.r#type.r#type(), ObjectType::NxSuperblock);
            assert_eq!(mount.superblock.header.r#type.storage(), StorageType::Ephemeral);
            assert!(mount.superblock.header.r#type.flags().is_empty());
            assert_eq!(mount.superblock.header.xid, Xid(3));
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumSet as usize], 25);
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumFail as usize], 0);
        }
    }

    #[test]
    fn mounting_test_apfs_when_latest_checkpoint_map_corrupted() {
        for rotations in 0..7 {
            let mut apfs = load_test_apfs_checkpoints();
            corrupt_descriptor_block(&mut apfs, 7);
            rotate_checkpoint_descriptor(&mut apfs, rotations);
            let mount = APFSMount::mount(apfs).unwrap();
            assert_eq!(mount.superblock.header.oid, Oid(1));
            assert_eq!(mount.superblock.header.r#type.r#type(), ObjectType::NxSuperblock);
            assert_eq!(mount.superblock.header.r#type.storage(), StorageType::Ephemeral);
            assert!(mount.superblock.header.r#type.flags().is_empty());
            assert_eq!(mount.superblock.header.xid, Xid(3));
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumSet as usize], 25);
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumFail as usize], 0);
        }
    }

    #[test]
    fn mounting_test_apfs_when_multiple_checkpoints_corrupted() {
        for rotations in 0..7 {
            let mut apfs = load_test_apfs_checkpoints();
            corrupt_descriptor_block(&mut apfs, 6);
            corrupt_descriptor_block(&mut apfs, 7);
            rotate_checkpoint_descriptor(&mut apfs, rotations);
            let mount = APFSMount::mount(apfs).unwrap();
            assert_eq!(mount.superblock.header.oid, Oid(1));
            assert_eq!(mount.superblock.header.r#type.r#type(), ObjectType::NxSuperblock);
            assert_eq!(mount.superblock.header.r#type.storage(), StorageType::Ephemeral);
            assert!(mount.superblock.header.r#type.flags().is_empty());
            assert_eq!(mount.superblock.header.xid, Xid(2));
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumSet as usize], 16);
            assert_eq!(mount.superblock.body.counters[CounterId::CntrObjCksumFail as usize], 0);
        }
    }
}

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, prelude::*, Cursor, SeekFrom};
use std::path::Path;

use btree::{Key, Value, Record};
use num_traits::FromPrimitive;

pub use btree::{Btree, OmapRecord, ApfsKey, ApfsValue, LeafRecord, LeafValue, NonLeafRecord, AnyRecords, InodeXdata, SpacemanFreeQueueValue, BtreeTypes, load_btree_generic};

#[macro_use]
mod int_strings;
mod internal;
mod fletcher;

pub use internal::*;
mod btree;
use fletcher::fletcher64;

pub use internal::Paddr;



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
    pub header: ObjPhys,
    pub body: BtreeNodePhys,
}

#[derive(Debug)]
pub struct ApfsSuperblockObject {
    header: ObjPhys,
    pub body: ApfsSuperblock,
}

#[derive(Debug)]
pub struct ChunkInfoBlockObject {
    header: ObjPhys,
    pub body: ChunkInfoBlock,
}

#[derive(Debug)]
pub struct SpacemanObject {
    header: ObjPhys,
    pub body: SpacemanPhys,
}

#[derive(Debug)]
pub struct NxReaperObject {
    header: ObjPhys,
    pub body: NxReaperPhys,
}

#[derive(Debug)]
pub struct NxEfiJumpstartObject {
    header: ObjPhys,
    pub body: NxEfiJumpstart,
}

#[derive(Debug)]
pub enum APFSObject {
    Superblock(NxSuperblockObject),
    CheckpointMapping(CheckpointMapPhysObject),
    ObjectMap(ObjectMapObject),
    Btree(BtreeNodeObject),
    BtreeNode(BtreeNodeObject),
    ApfsSuperblock(ApfsSuperblockObject),
    Spaceman(SpacemanObject),
    SpacemanCib(ChunkInfoBlockObject),
    NxReaper(NxReaperObject),
    EfiJumpstart(NxEfiJumpstartObject),
}

pub struct APFS<S: Read + Seek> {
    source: S,
    block_size: usize,
}

impl APFS<File> {
    pub fn open<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let mut source = File::open(filename)?;
        APFS::open_source(source)
    }

    pub fn load_btree<V: LeafValue>(&mut self, oid: Oid, r#type: StorageType) -> io::Result<btree::Btree<V>> {
        btree::Btree::load_btree(self, oid, r#type)
    }
}

impl<S: Read + Seek> APFS<S> {
    pub fn open_source(mut source: S) -> io::Result<Self> {
        let mut block0 = [0; NX_MINIMUM_BLOCK_SIZE];
        source.read_exact(&mut block0[..])?;
        let mut cursor = Cursor::new(&block0[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        let superblock = NxSuperblock::import(&mut cursor).unwrap();
        Ok(APFS { source, block_size: superblock.block_size as usize })
    }

    pub fn load_block(&mut self, addr: Paddr) -> io::Result<Vec<u8>> {
        println!("Loading block {}", addr.0);
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
                APFSObject::Btree(BtreeNodeObject {
                header,
                body: BtreeNodePhys::import(&mut cursor)?,
            }),
            ObjectType::BtreeNode =>
                APFSObject::BtreeNode(BtreeNodeObject {
                header,
                body: BtreeNodePhys::import(&mut cursor)?,
            }),
            ObjectType::Fs =>
                APFSObject::ApfsSuperblock(ApfsSuperblockObject {
                header,
                body: ApfsSuperblock::import(&mut cursor)?,
            }),
            ObjectType::Spaceman =>
                APFSObject::Spaceman(SpacemanObject {
                header,
                body: SpacemanPhys::import(&mut cursor)?,
            }),
            ObjectType::SpacemanCib =>
                APFSObject::SpacemanCib(ChunkInfoBlockObject {
                header,
                body: ChunkInfoBlock::import(&mut cursor)?,
            }),
            ObjectType::NxReaper =>
                APFSObject::NxReaper(NxReaperObject {
                header,
                body: NxReaperPhys::import(&mut cursor)?,
            }),
            ObjectType::EfiJumpstart =>
                APFSObject::EfiJumpstart(NxEfiJumpstartObject {
                header,
                body: NxEfiJumpstart::import(&mut cursor)?,
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
}

struct APFSMount<S: Read + Seek> {
    apfs: APFS<S>,
    superblock: NxSuperblockObject,
}

impl<S: Read + Seek> APFSMount<S> {
    fn mount(mut apfs: APFS<S>) -> io::Result<Self> {
        let mut superblock = match apfs.load_object_addr(Paddr(0)).unwrap() {
            APFSObject::Superblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        let mut best_xid = 0;
        for idx in 0..superblock.body.xp_desc_blocks {
            let object = apfs.load_object_addr(Paddr(superblock.body.xp_desc_base.0+idx as i64));
            if let Ok(APFSObject::Superblock(body)) = object {
                let map_object = apfs.load_object_addr(Paddr(superblock.body.xp_desc_base.0 + ((idx + superblock.body.xp_desc_blocks - 1) % superblock.body.xp_desc_blocks) as i64));
                if let Ok(APFSObject::CheckpointMapping(map_body)) = map_object {
                    if body.header.xid.0 > best_xid {
                        best_xid = body.header.xid.0;
                        superblock = body;
                    }
                }
            }
        }
        Ok(APFSMount { apfs, superblock })
    }
}
