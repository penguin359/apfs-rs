#![allow(dead_code)]

use std::io::{self, prelude::*};

use num_derive::FromPrimitive;

use byteorder::{LittleEndian, ReadBytesExt};
use bitflags;
use uuid::{Bytes, Uuid};

#[cfg(test)]
mod test;


// General-Purpose Types

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Paddr(pub i64);

impl Paddr {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self(source.read_i64::<LittleEndian>()?))
    }
}

#[derive(Debug)]
struct Prange {
    start_paddr: Paddr,
    block_count: u64,
}

impl Prange {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            start_paddr: Paddr::import(source)?,
            block_count: source.read_u64::<LittleEndian>()?,
        })
    }
}

fn import_uuid(source: &mut dyn Read) -> io::Result<Uuid> {
    let mut data: Bytes = [0; 16];
    source.read_exact(&mut data)?;
    Ok(Uuid::from_bytes(data))
}


// Objects

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Oid(pub u64);

impl Oid {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self(source.read_u64::<LittleEndian>()?))
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Xid(pub u64);

impl Xid {
    fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self(source.read_u64::<LittleEndian>()?))
    }
}


const MAX_CKSUM_SIZE: usize = 8;

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
#[derive(Debug, FromPrimitive)]
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

    ErRecoveryBlock = 0x0000001c,
    SnapMetaExt = 0x0000001d,
    IntegrityMeta = 0x0000001e,
    FextTree = 0x0000001f,
    Reserved20 = 0x00000020,

    Invalid               = 0x00000000,
    Test                  = 0x000000ff,

    ContainerKeybag       = 0x7379656b,  // u32_code!(b"keys"),
    VolumeKeybag          = 0x73636572,  // u32_code!(b"recs"),
    MediaKeybag           = 0x79656b6d,  // u32_code!(b"mkey"),
}


pub enum StorageType {
    Virtual                       = 0x00000000,
    Ephemeral                     = 0x80000000,
    Physical                      = 0x40000000,
}

bitflags! {
    struct ObjTypeFlags: u32 {
        const OBJ_NOHEADER        = 0x20000000;
        const OBJ_ENCRYPTED       = 0x10000000;
        const OBJ_NONPERSISTENT   = 0x08000000;
    }
}


// Container

enum CounterId {
      CntrObjCksumSet = 0,
      CntrObjCksumFail = 1,

      NumCounters = 32,
}

const NX_MAGIC: u32 = u32_code!(b"BSXN");
const NX_MAX_FILE_SYSTEMS: usize = 100;

const NX_EPH_INFO_COUNT: usize = 4;
const NX_EPH_MIN_BLOCK_COUNT: usize = 8;
const NX_MAX_FILE_SYSTEM_EPH_STRUCTS: usize = 4;
const NX_TX_MIN_CHECKPOINT_COUNT: usize = 4;
const NX_EPH_INFO_VERSION_1: usize = 1;

const NX_NUM_COUNTERS: usize = 32;

bitflags! {
    struct SuperblockFlags: u64 {
        const RESERVED_1 = 0x00000001;
        const RESERVED_2 = 0x00000002;
        const CRYPTO_SW = 0x00000004;
    }
}

bitflags! {
    struct SuperblockFeatureFlags: u64 {
        const DEFRAG = 0x0000000000000001;
        const LCFD = 0x0000000000000002;
        const SUPPORTED_MASK = Self::DEFRAG.bits | Self::LCFD.bits;
    }
}

bitflags! {
    struct SuperblockRocompatFlags: u64 {
        const SUPPORTED_MASK = 0;
    }
}

bitflags! {
    struct SuperblockIncompatFlags: u64 {
        const VERSION1 = 0x0000000000000001;
        const VERSION2 = 0x0000000000000002;
        const FUSION = 0x0000000000000100;
        const SUPPORTED_MASK = Self::VERSION2.bits | Self::FUSION.bits;
    }
}

#[derive(Debug)]
pub struct NxSuperblock {
        //nx_o: ObjPhys,
        pub nx_magic: u32,
        pub nx_block_size: u32,
        pub nx_block_count: u64,

        nx_features: SuperblockFeatureFlags,
        nx_readonly_compatible_features: SuperblockRocompatFlags,
        nx_incompatible_features: SuperblockIncompatFlags,

        nx_uuid: Uuid,

        nx_next_oid: Oid,
        nx_next_xid: Xid,

        pub nx_xp_desc_blocks: u32,
        pub nx_xp_data_blocks: u32,
        pub nx_xp_desc_base: Paddr,
        pub nx_xp_data_base: Paddr,
        pub nx_xp_desc_next: u32,
        pub nx_xp_data_next: u32,
        pub nx_xp_desc_index: u32,
        pub nx_xp_desc_len: u32,
        pub nx_xp_data_index: u32,
        pub nx_xp_data_len: u32,

        pub nx_spaceman_oid: Oid,
        pub nx_omap_oid: Oid,
        pub nx_reaper_oid: Oid,

        nx_test_type: u32,

        nx_max_file_systems: u32,
        nx_fs_oid: [Oid; NX_MAX_FILE_SYSTEMS],
        nx_counters: [u64; CounterId::NumCounters as usize],
        nx_blocked_out_prange: Prange,
        nx_evict_mapping_tree_oid: Oid,
        nx_flags: SuperblockFlags,
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

            nx_features: SuperblockFeatureFlags::from_bits(source.read_u64::<LittleEndian>()?)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unknown flags"))?,
            nx_readonly_compatible_features: SuperblockRocompatFlags::from_bits(source.read_u64::<LittleEndian>()?)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unknown flags"))?,
            nx_incompatible_features: SuperblockIncompatFlags::from_bits(source.read_u64::<LittleEndian>()?)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unknown flags"))?,

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
            nx_flags: SuperblockFlags::from_bits(source.read_u64::<LittleEndian>()?)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unknown flags"))?,
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

const NX_MINIMUM_BLOCK_SIZE: usize = 4096;
const NX_DEFAULT_BLOCK_SIZE: usize = 4096;
const NX_MAXIMUM_BLOCK_SIZE: usize = 65536;

const NX_MINIMUM_CONTAINER_SIZE: usize = 1048576;


#[derive(Debug)]
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

#[derive(Debug)]
pub struct CheckpointMapPhys {
      //cpm_o:        ObjPhys,
      flags:    CpmFlags,
      count:    u32,
      map:      Vec<CheckpointMapping>,
}

impl CheckpointMapPhys {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        let mut checkpoint_map = Self {
            //cpm_o: ObjPhys::import(source)?,
            flags: CpmFlags::from_bits(source.read_u32::<LittleEndian>()?)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unknown flags"))?,
            count: source.read_u32::<LittleEndian>()?,
            map: vec![],
        };
        for _ in 0..checkpoint_map.count {
            checkpoint_map.map.push(CheckpointMapping::import(source)?);
        }
        Ok(checkpoint_map)
    }
}

struct EvictMappingVal {
    dst_paddr: Paddr,
    len: u64,
}

impl EvictMappingVal {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            dst_paddr: Paddr::import(source)?,
            len: source.read_u64::<LittleEndian>()?,
        })
    }
}


// File system objects

pub const APFS_MAGIC: u32   = u32_code!(b"BSXN");


bitflags! {
    struct OmFlags: u32 {
        const MANUALLY_MANAGED     = 0x00000001;
        const ENCRYPTING           = 0x00000002;
        const DECRYPTING           = 0x00000004;
        const KEYROLLING           = 0x00000008;
        const CRYPTO_GENERATION    = 0x00000010;

        const VALID_FLAGS          = 0x0000001f;
    }
}

#[derive(Debug)]
pub struct OmapPhys {
        //om_o: ObjPhys,
        om_flags: OmFlags,
        om_snap_count: u32,
        om_tree_type: u32,
        om_snapshot_tree_type: u32,
        pub om_tree_oid: Oid,
        om_snapshot_tree_oid: Oid,
        om_most_recent_snap: Xid,
        om_pending_revert_min: Xid,
        om_pending_revert_max: Xid,
}

impl OmapPhys {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            //om_o: ObjPhys::import(source)?,
            om_flags: OmFlags::from_bits(source.read_u32::<LittleEndian>()?).unwrap(),
            om_snap_count: source.read_u32::<LittleEndian>()?,
            om_tree_type: source.read_u32::<LittleEndian>()?,
            om_snapshot_tree_type: source.read_u32::<LittleEndian>()?,
            om_tree_oid: Oid::import(source)?,
            om_snapshot_tree_oid: Oid::import(source)?,
            om_most_recent_snap: Xid::import(source)?,
            om_pending_revert_min: Xid::import(source)?,
            om_pending_revert_max: Xid::import(source)?,
        })
    }
}

#[derive(Debug)]
pub struct OmapKey {
        pub oid: Oid,
        pub xid: Xid,
}

impl OmapKey {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            oid: Oid::import(source)?,
            xid: Xid::import(source)?,
        })
    }
}

bitflags! {
    struct OvFlags: u32 {
        const VAL_DELETED               = 0x00000001;
        const VAL_SAVED                 = 0x00000002;
        const VAL_ENCRYPTED             = 0x00000004;
        const VAL_NOHEADER              = 0x00000008;
        const VAL_CRYPTO_GENERATION     = 0x00000010;
    }
}

pub struct OmapVal {
        ov_flags: OvFlags,
        ov_size: u32,
        ov_paddr: Paddr,
}

impl OmapVal {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            ov_flags: OvFlags::from_bits(source.read_u32::<LittleEndian>()?).unwrap(),
            ov_size: source.read_u32::<LittleEndian>()?,
            ov_paddr: Paddr::import(source)?,
        })
    }
}


// B-Tree data structures

const BTREE_TOC_ENTRY_INCREMENT: usize = 8;
const BTREE_TOC_ENTRY_MAX_UNUSED: usize = (2 * BTREE_TOC_ENTRY_INCREMENT);

const BTREE_NODE_SIZE_DEFAULT: usize = 4096;
const BTREE_NODE_MIN_ENTRY_COUNT: usize = 4;

#[derive(Debug)]
pub struct Nloc {
    pub off: u16,
    pub len: u16,
}

const BTOFF_INVALID: u16 = 0xffff;

impl Nloc {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            off: source.read_u16::<LittleEndian>()?,
            len: source.read_u16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
struct KVloc {
    k: Nloc,
    v: Nloc,
}

impl KVloc {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            k: Nloc::import(source)?,
            v: Nloc::import(source)?,
        })
    }
}

#[derive(Debug)]
struct KVoff {
    k: u16,
    v: u16,
}

impl KVoff {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            k: source.read_u16::<LittleEndian>()?,
            v: source.read_u16::<LittleEndian>()?,
        })
    }
}

bitflags! {
    pub struct BtnFlags: u16 {
        const ROOT               = 0x0001;
        const LEAF               = 0x0002;

        const FIXED_KV_SIZE      = 0x0004;
        const HASHED             = 0x0008;
        const NOHEADER           = 0x0010;

        const CHECK_KOFF_INVAL   = 0x8000;
    }
}

#[derive(Debug)]
pub struct BtreeNodePhys {
        //btn_o: ObjPhys,
        pub btn_flags: BtnFlags,
        pub btn_level: u16,
        pub btn_nkeys: u32,
        pub btn_table_space: Nloc,
        pub btn_free_space: Nloc,
        pub btn_key_free_list: Nloc,
        pub btn_val_free_list: Nloc,
        pub btn_data: Vec<u8>,
}

impl BtreeNodePhys {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        let mut value = Self {
            //btn_o: ObjPhys::import(source)?,
            btn_flags: BtnFlags::from_bits(source.read_u16::<LittleEndian>()?)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unknown flags"))?,
            btn_level: source.read_u16::<LittleEndian>()?,
            btn_nkeys: source.read_u32::<LittleEndian>()?,
            btn_table_space: Nloc::import(source)?,
            btn_free_space: Nloc::import(source)?,
            btn_key_free_list: Nloc::import(source)?,
            btn_val_free_list: Nloc::import(source)?,
            btn_data: vec![],
        };
        source.read_to_end(&mut value.btn_data)?;
        Ok(value)
    }
}

bitflags! {
    struct BtFlags: u32 {
        const UINT64_KEYS         = 0x00000001;
        const SEQUENTIAL_INSERT   = 0x00000002;
        const ALLOW_GHOSTS        = 0x00000004;
        const EPHEMERAL           = 0x00000008;
        const PHYSICAL            = 0x00000010;
        const NONPERSISTENT       = 0x00000020;
        const KV_NONALIGNED       = 0x00000040;
        const HASHED              = 0x00000080;
        const NOHEADER            = 0x00000100;
    }
}

#[derive(Debug)]
pub struct BtreeInfoFixed {
        //bt_o: ObjPhys,
        bt_flags: BtFlags,
        bt_node_size: u32,
        bt_key_size: u32,
        bt_val_size: u32,
}

impl BtreeInfoFixed {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            //bt_o: ObjPhys::import(source)?,
            bt_flags: BtFlags::from_bits(source.read_u32::<LittleEndian>()?)
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Unknown flags"))?,
            bt_node_size: source.read_u32::<LittleEndian>()?,
            bt_key_size: source.read_u32::<LittleEndian>()?,
            bt_val_size: source.read_u32::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct BtreeInfo {
        bt_fixed: BtreeInfoFixed,
        bt_longest_key: u32,
        bt_longest_val: u32,
        bt_key_count: u64,
        bt_node_count: u64,
}

impl BtreeInfo {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        Ok(Self {
            bt_fixed: BtreeInfoFixed::import(source)?,
            bt_longest_key: source.read_u32::<LittleEndian>()?,
            bt_longest_val: source.read_u32::<LittleEndian>()?,
            bt_key_count: source.read_u64::<LittleEndian>()?,
            bt_node_count: source.read_u64::<LittleEndian>()?,
        })
    }
}

const BTREE_NODE_HASH_SIZE_MAX: usize = 64;

#[derive(Debug)]
struct BtnIndexNodeVal {
    binv_child_oid: Oid,
    binv_child_hash: [u8; BTREE_NODE_HASH_SIZE_MAX],
}

impl BtnIndexNodeVal {
    pub fn import(source: &mut dyn Read) -> io::Result<Self> {
        let mut value = Self {
            binv_child_oid: Oid::import(source)?,
            binv_child_hash: [0; BTREE_NODE_HASH_SIZE_MAX],
        };
        source.read_exact(&mut value.binv_child_hash)?;
        Ok(value)
    }
}
