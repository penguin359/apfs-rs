#![allow(dead_code)]

use uuid::{Bytes, Uuid};

use std::io::{self, prelude::*};

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use byteorder::{LittleEndian, ReadBytesExt};

//use super::int_strings::u32_code;

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
struct Oid(u64);

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


//const MAX_CKSUM_SIZE: usize = 8;

struct ObjPhys {
    //o_cksum: [u8; MAX_CKSUM_SIZE],
    o_cksum: u64,
    o_oid: Oid,
    o_xid: Xid,
    o_type: u32,
    o_subtype: u32,
}

impl ObjPhys {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
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


const OBJECT_TYPE_MASK                  : u32 = 0x0000ffff;
const OBJECT_TYPE_FLAGS_MASK            : u32 = 0xffff0000;

const OBJ_STORAGETYPE_MASK              : u32 = 0xc0000000;
const OBJECT_TYPE_FLAGS_DEFINED_MASK    : u32 = 0xf8000000;


const OBJECT_TYPE_NX_SUPERBLOCK         : u32 = 0x00000001;

const OBJECT_TYPE_BTREE                 : u32 = 0x00000002;
const OBJECT_TYPE_BTREE_NODE            : u32 = 0x00000003;

const OBJECT_TYPE_SPACEMAN              : u32 = 0x00000005;
const OBJECT_TYPE_SPACEMAN_CAB          : u32 = 0x00000006;
const OBJECT_TYPE_SPACEMAN_CIB          : u32 = 0x00000007;
const OBJECT_TYPE_SPACEMAN_BITMAP       : u32 = 0x00000008;
const OBJECT_TYPE_SPACEMAN_FREE_QUEUE   : u32 = 0x00000009;

const OBJECT_TYPE_EXTENT_LIST_TREE      : u32 = 0x0000000a;
const OBJECT_TYPE_OMAP                  : u32 = 0x0000000b;
const OBJECT_TYPE_CHECKPOINT_MAP        : u32 = 0x0000000c;

const OBJECT_TYPE_FS                    : u32 = 0x0000000d;
const OBJECT_TYPE_FSTREE                : u32 = 0x0000000e;
const OBJECT_TYPE_BLOCKREFTREE          : u32 = 0x0000000f;
const OBJECT_TYPE_SNAPMETATREE          : u32 = 0x00000010;

const OBJECT_TYPE_NX_REAPER             : u32 = 0x00000011;
const OBJECT_TYPE_NX_REAP_LIST          : u32 = 0x00000012;
const OBJECT_TYPE_OMAP_SNAPSHOT         : u32 = 0x00000013;
const OBJECT_TYPE_EFI_JUMPSTART         : u32 = 0x00000014;
const OBJECT_TYPE_FUSION_MIDDLE_TREE    : u32 = 0x00000015;
const OBJECT_TYPE_NX_FUSION_WBC         : u32 = 0x00000016;
const OBJECT_TYPE_NX_FUSION_WBC_LIST    : u32 = 0x00000017;
const OBJECT_TYPE_ER_STATE              : u32 = 0x00000018;

const OBJECT_TYPE_GBITMAP               : u32 = 0x00000019;
const OBJECT_TYPE_GBITMAP_TREE          : u32 = 0x0000001a;
const OBJECT_TYPE_GBITMAP_BLOCK         : u32 = 0x0000001b;

const OBJECT_TYPE_INVALID               : u32 = 0x00000000;
const OBJECT_TYPE_TEST                  : u32 = 0x000000ff;

const OBJECT_TYPE_CONTAINER_KEYBAG      : u32 = u32_code!(b"keys");
const OBJECT_TYPE_VOLUME_KEYBAG         : u32 = u32_code!(b"recs");
const OBJECT_TYPE_MEDIA_KEYBAG          : u32 = u32_code!(b"mkey");


const OBJ_VIRTUAL                       : u32 = 0x00000000;
const OBJ_EPHEMERAL                     : u32 = 0x80000000;
const OBJ_PHYSICAL                      : u32 = 0x40000000;

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

struct NxSuperblock {
        nx_o: ObjPhys,
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
}

impl NxSuperblock {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            nx_o: ObjPhys::import(source)?,
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
            nx_fs_oid: [Oid(0); 100], //[Oid; NX_MAX_FILE_SYSTEMS],
            nx_counters: [0; 32], //[u64; NX_NUM_COUNTERS],
            nx_blocked_out_prange: Prange::import(source)?,
            nx_evict_mapping_tree_oid: Oid::import(source)?,
            nx_flags: source.read_u64::<LittleEndian>()?,
            nx_efi_jumpstart: Paddr::import(source)?,
            nx_fusion_uuid: import_uuid(source)?,
            nx_keylocker: Prange::import(source)?,
            nx_ephemeral_info: [0; 4], //[u64; NX_EPH_INFO_COUNT],

            nx_test_oid: Oid::import(source)?,

            nx_fusion_mt_oid: Oid::import(source)?,
            nx_fusion_wbc_oid: Oid::import(source)?,
            nx_fusion_wbc: Prange::import(source)?,
        })
    }
}


//#define NX_RESERVED_1 0x00000001LL
//#define NX_RESERVED_2 0x00000002LL
//#define NX_CRYPTO_SW 0x00000004LL
//
//
//#define NX_FEATURE_DEFRAG 0x0000000000000001ULL
//#define NX_FEATURE_LCFD 0x0000000000000002ULL
//#define NX_SUPPORTED_FEATURES_MASK (NX_FEATURE_DEFRAG | NX_FEATURE_LCFD)
//
//
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
//
//
//struct checkpoint_mapping {
//      uint32_t cpm_type;
//      uint32_t cpm_subtype;
//      uint32_t cpm_size;
//      uint32_t cpm_pad;
//      oid_t cpm_fs_oid;
//      oid_t cpm_oid;
//      oid_t cpm_paddr;
//};
//typedef struct checkpoint_mapping checkpoint_mapping_t;
//
//
//struct checkpoint_map_phys {
//      obj_phys_t cpm_o;
//      uint32_t cpm_flags;
//      uint32_t cpm_count;
//      checkpoint_mapping_t cpm_map[];
//};
//
//#define CHECKPOINT_MAP_LAST 0x00000001
//
//
//struct evict_mapping_val {
//      paddr_t dst_paddr;
//      uint64_t len;
//} __attribute__((packed));
//typedef struct evict_mapping_val evict_mapping_val_t;
//
//

#[repr(u32)]
enum ObjectType {
    NxSuperblock        = 0x00000001,
    Btree               = 0x00000002,
    BtreeNode           = 0x00000003,
    Spaceman            = 0x00000005,
    SpacemanCab         = 0x00000006,
    SpacemanCib         = 0x00000007,
    SpacemanBitmap      = 0x00000008,
    SpacemanFreeQueue   = 0x00000009,
    ExtentListTree      = 0x0000000a,
    Omap                = 0x0000000b,
    CheckpointMap       = 0x0000000c,
    Fs                  = 0x0000000d,
    Fstree              = 0x0000000e,
    Blockreftree        = 0x0000000f,
    Snapmetatree        = 0x00000010,
    NxReaper            = 0x00000011,
    NxReapList          = 0x00000012,
    OmapSnapshot        = 0x00000013,
    EfiJumpstart        = 0x00000014,
    FusionMiddleTree    = 0x00000015,
    NxFusionWbc         = 0x00000016,
    NxFusionWbcList     = 0x00000017,
    ErState             = 0x00000018,
    Gbitmap             = 0x00000019,
    GbitmapTree         = 0x0000001a,
    GbitmapBlock        = 0x0000001b,
    ErRecoveryBlock     = 0x0000001c,
    SnapMetaExt         = 0x0000001d,
    IntegrityMeta       = 0x0000001e,
    FextTree            = 0x0000001f,
    Reserved20          = 0x00000020,

    Invalid             = 0x00000000,
    Test                = 0x000000ff,
}
