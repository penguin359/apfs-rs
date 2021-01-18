#![allow(dead_code)]

use std::io::{self, prelude::*};

use byteorder::{LittleEndian, ReadBytesExt};
use bitflags;
use uuid::{Bytes, Uuid};

#[cfg(test)]
mod test;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Paddr(i64);

impl Paddr {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self(source.read_i64::<LittleEndian>()?))
    }
}

struct Prange {
    pr_start_paddr: Paddr,
    pr_block_count: u64,
}

impl Prange {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            pr_start_paddr: Paddr::import(source)?,
            pr_block_count: source.read_u64::<LittleEndian>()?,
        })
    }
}

fn import_uuid(source: &mut dyn Read) -> io::Result<Uuid> {
    let mut data: Bytes = [0; 16];
    source.read_exact(&mut data)?;
    Ok(Uuid::from_bytes(data))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Oid(pub u64);

impl Oid {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self(source.read_u64::<LittleEndian>()?))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Xid(u64);

impl Xid {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self(source.read_u64::<LittleEndian>()?))
    }
}


#[derive(Debug)]
pub struct ObjPhys {
    pub o_cksum: u64,
    o_oid: Oid,
    o_xid: Xid,
    pub o_type: u32,
    o_subtype: u32,
}

impl ObjPhys {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            o_cksum: source.read_u64::<LittleEndian>()?,
            o_oid: Oid::import(source)?,
            o_xid: Xid::import(source)?,
            o_type: source.read_u32::<LittleEndian>()?,
            o_subtype: source.read_u32::<LittleEndian>()?,
        })
    }
}


const OID_NX_SUPERBLOCK                 : Oid = Oid(1);

const OID_INVALID                       : Oid = Oid(0);
const OID_RESERVED_COUNT                : u64 = 1024;


pub const OBJECT_TYPE_MASK                  : u32 = 0x0000ffff;
pub const OBJECT_TYPE_FLAGS_MASK            : u32 = 0xffff0000;

const OBJ_STORAGETYPE_MASK              : u32 = 0xc0000000;
const OBJECT_TYPE_FLAGS_DEFINED_MASK    : u32 = 0xf8000000;


#[repr(u32)]
pub enum ObjectType {
    NxSuperblock         = 0x00000001,

    Btree                 = 0x00000002,
    BtreeNode            = 0x00000003,

    Spaceman              = 0x00000005,
    SpacemanCab          = 0x00000006,
    SpacemanCib          = 0x00000007,
    SpacemanBitmap       = 0x00000008,
    SpacemanFreeQueue   = 0x00000009,

    ExtentListTree      = 0x0000000a,
    Omap                  = 0x0000000b,
    CheckpointMap        = 0x0000000c,

    Fs                    = 0x0000000d,
    Fstree                = 0x0000000e,
    Blockreftree          = 0x0000000f,
    Snapmetatree          = 0x00000010,

    NxReaper             = 0x00000011,
    NxReapList          = 0x00000012,
    OmapSnapshot         = 0x00000013,
    EfiJumpstart         = 0x00000014,
    FusionMiddleTree    = 0x00000015,
    NxFusionWbc         = 0x00000016,
    NxFusionWbcList    = 0x00000017,
    ErState              = 0x00000018,

    Gbitmap               = 0x00000019,
    GbitmapTree          = 0x0000001a,
    GbitmapBlock         = 0x0000001b,

    Invalid               = 0x00000000,
    Test                  = 0x000000ff,

    ContainerKeybag      = u32_code!(b"keys"),
    VolumeKeybag         = u32_code!(b"recs"),
    MediaKeybag          = u32_code!(b"mkey"),
}


pub enum StorageType {
    Virtual                       = 0x00000000,
    Ephemeral                     = 0x80000000,
    Physical                      = 0x40000000,
}

const OBJ_NOHEADER                      : u32 = 0x20000000;
const OBJ_ENCRYPTED                     : u32 = 0x10000000;
const OBJ_NONPERSISTENT                 : u32 = 0x08000000;


//typedef enum {
//      NX_CNTR_OBJ_CKSUM_SET = 0,
//      NX_CNTR_OBJ_CKSUM_FAIL = 1,
//
//      NX_NUM_COUNTERS = 32
//} nx_counter_id_t;


const NX_MAGIC: u32 = u32_code!(b"BSXN");
const NX_NUM_COUNTERS: usize = 32;
const NX_MAX_FILE_SYSTEMS: usize = 100;

const NX_EPH_INFO_COUNT: usize = 4;
//#define NX_EPH_MIN_BLOCK_COUNT 8
//#define NX_MAX_FILE_SYSTEM_EPH_STRUCTS 4
//#define NX_TX_MIN_CHECKPOINT_COUNT 4
//#define NX_EPH_INFO_VERSION_1 1

pub struct NxSuperblock {
        //nx_o: ObjPhys,
        nx_magic: u32,
        nx_block_size: u32,
        nx_block_count: u64,

        nx_features: u64,
        nx_readonly_compatible_features: u64,
        nx_incompatible_features: u64,

        nx_uuid: Uuid,

        nx_next_oid: Oid,
        nx_next_xid: Xid,

        nx_xp_desc_blocks: u32,
        nx_xp_data_blocks: u32,
        nx_xp_desc_base: Paddr,
        nx_xp_data_base: Paddr,
        nx_xp_desc_next: u32,
        nx_xp_data_next: u32,
        nx_xp_desc_index: u32,
        nx_xp_desc_len: u32,
        nx_xp_data_index: u32,
        nx_xp_data_len: u32,

        nx_spaceman_oid: Oid,
        nx_omap_oid: Oid,
        nx_reaper_oid: Oid,

        nx_test_type: u32,

        nx_max_file_systems: u32,
        nx_fs_oid: [Oid; NX_MAX_FILE_SYSTEMS],
        nx_counters: [u64; NX_NUM_COUNTERS],
        nx_blocked_out_prange: Prange,
        nx_evict_mapping_tree_oid: Oid,
        nx_flags: u64,
        nx_efi_jumpstart: Paddr,
        nx_fusion_uuid: Uuid,
        nx_keylocker: Prange,
        nx_ephemeral_info: [u64; NX_EPH_INFO_COUNT],

        nx_test_oid: Oid,

        nx_fusion_mt_oid: Oid,
        nx_fusion_wbc_oid: Oid,
        nx_fusion_wbc: Prange,

        nx_newest_mounted_version: u64,

        nx_mkb_locker: Prange,
}

impl NxSuperblock {
    fn import_fs_oids(source: &mut dyn Read) -> io::Result<[Oid; NX_MAX_FILE_SYSTEMS]> {
        let mut oids = [Oid(0); NX_MAX_FILE_SYSTEMS];
        for entry in oids.iter_mut() {
            *entry = Oid::import(source)?;
        }
        return Ok(oids);
    }

    fn import_counters(source: &mut dyn Read) -> io::Result<[u64; NX_NUM_COUNTERS]> {
        let mut counters = [0; NX_NUM_COUNTERS];
        for entry in counters.iter_mut() {
            *entry = source.read_u64::<LittleEndian>()?;
        }
        return Ok(counters);
    }

    fn import_ephemeral_info(source: &mut dyn Read) -> io::Result<[u64; NX_EPH_INFO_COUNT]> {
        let mut info = [0; NX_EPH_INFO_COUNT];
        for entry in info.iter_mut() {
            *entry = source.read_u64::<LittleEndian>()?;
        }
        return Ok(info);
    }

    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            //nx_o: ObjPhys::import(source)?,
            nx_magic: source.read_u32::<LittleEndian>()?,
            nx_block_size: source.read_u32::<LittleEndian>()?,
            nx_block_count: source.read_u64::<LittleEndian>()?,

            nx_features: source.read_u64::<LittleEndian>()?,
            nx_readonly_compatible_features: source.read_u64::<LittleEndian>()?,
            nx_incompatible_features: source.read_u64::<LittleEndian>()?,

            nx_uuid: import_uuid(source)?,

            nx_next_oid: Oid::import(source)?,
            nx_next_xid: Xid::import(source)?,

            nx_xp_desc_blocks: source.read_u32::<LittleEndian>()?,
            nx_xp_data_blocks: source.read_u32::<LittleEndian>()?,
            nx_xp_desc_base: Paddr::import(source)?,
            nx_xp_data_base: Paddr::import(source)?,
            nx_xp_desc_next: source.read_u32::<LittleEndian>()?,
            nx_xp_data_next: source.read_u32::<LittleEndian>()?,
            nx_xp_desc_index: source.read_u32::<LittleEndian>()?,
            nx_xp_desc_len: source.read_u32::<LittleEndian>()?,
            nx_xp_data_index: source.read_u32::<LittleEndian>()?,
            nx_xp_data_len: source.read_u32::<LittleEndian>()?,

            nx_spaceman_oid: Oid::import(source)?,
            nx_omap_oid: Oid::import(source)?,
            nx_reaper_oid: Oid::import(source)?,

            nx_test_type: source.read_u32::<LittleEndian>()?,

            nx_max_file_systems: source.read_u32::<LittleEndian>()?,
            nx_fs_oid: Self::import_fs_oids(source)?,
            nx_counters: Self::import_counters(source)?,
            nx_blocked_out_prange: Prange::import(source)?,
            nx_evict_mapping_tree_oid: Oid::import(source)?,
            nx_flags: source.read_u64::<LittleEndian>()?,
            nx_efi_jumpstart: Paddr::import(source)?,
            nx_fusion_uuid: import_uuid(source)?,
            nx_keylocker: Prange::import(source)?,
            nx_ephemeral_info: Self::import_ephemeral_info(source)?,

            nx_test_oid: Oid::import(source)?,

            nx_fusion_mt_oid: Oid::import(source)?,
            nx_fusion_wbc_oid: Oid::import(source)?,
            nx_fusion_wbc: Prange::import(source)?,

            nx_newest_mounted_version: source.read_u64::<LittleEndian>()?,

            nx_mkb_locker: Prange::import(source)?,
        })
    }
}


//#define NX_RESERVED_1 0x00000001LL
//#define NX_RESERVED_2 0x00000002LL
//#define NX_CRYPTO_SW 0x00000004LL


const NX_FEATURE_DEFRAG: u64 = 0x0000000000000001;
const NX_FEATURE_LCFD: u64 = 0x0000000000000002;
const NX_SUPPORTED_FEATURES_MASK: u64 = NX_FEATURE_DEFRAG | NX_FEATURE_LCFD;


//#define NX_SUPPORTED_ROCOMPAT_MASK (0x0ULL)
//
//
//#define NX_INCOMPAT_VERSION1 0x0000000000000001ULL
//#define NX_INCOMPAT_VERSION2 0x0000000000000002ULL
//#define NX_INCOMPAT_FUSION 0x0000000000000100ULL
//#define NX_SUPPORTED_INCOMPAT_MASK (NX_INCOMPAT_VERSION2 | NX_INCOMPAT_FUSION)
//
//
//#define NX_MINIMUM_BLOCK_SIZE 4096
//#define NX_DEFAULT_BLOCK_SIZE 4096
//#define NX_MAXIMUM_BLOCK_SIZE 65536
//
//#define NX_MINIMUM_CONTAINER_SIZE 1048576


struct CheckpointMapping {
    cpm_type:       u32,
    cpm_subtype:    u32,
    cpm_size:       u32,
    cpm_pad:        u32,
    cpm_fs_oid:     Oid,
    cpm_oid:        Oid,
    cpm_paddr:      Oid,
}

impl CheckpointMapping {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            cpm_type: source.read_u32::<LittleEndian>()?,
            cpm_subtype: source.read_u32::<LittleEndian>()?,
            cpm_size: source.read_u32::<LittleEndian>()?,
            cpm_pad: source.read_u32::<LittleEndian>()?,
            cpm_fs_oid: Oid::import(source)?,
            cpm_oid: Oid::import(source)?,
            cpm_paddr: Oid::import(source)?,
        })
    }
}


bitflags! {
    struct CpmFlags: u32 {
        const CHECKPOINT_MAP_LAST = 0x00000001;
    }
}

pub struct CheckpointMapPhys {
      //cpm_o:        ObjPhys,
      cpm_flags:    CpmFlags,
      cpm_count:    u32,
      cpm_map:      Vec<CheckpointMapping>,
}

impl CheckpointMapPhys {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        let mut checkpoint_map = Self {
            //cpm_o: ObjPhys::import(source)?,
            cpm_flags: CpmFlags::from_bits(source.read_u32::<LittleEndian>()?).unwrap(),
            cpm_count: source.read_u32::<LittleEndian>()?,
            cpm_map: vec![],
        };
        for _ in 0..checkpoint_map.cpm_count {
            checkpoint_map.cpm_map.push(CheckpointMapping::import(source)?);
        }
        Ok(checkpoint_map)
    }
}



//struct evict_mapping_val {
//      paddr_t dst_paddr;
//      uint64_t len;
//} __attribute__((packed));
//typedef struct evict_mapping_val evict_mapping_val_t;




pub const APFS_MAGIC: u32   = u32_code!(b"BSXN");
