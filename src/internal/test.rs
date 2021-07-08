use super::*;

use std::fs::File;
use std::io::Cursor;
use std::io::SeekFrom;
use std::path::PathBuf;

use crate::fletcher::fletcher64;

fn test_dir() -> PathBuf {
    let root = ::std::env::var_os("CARGO_MANIFEST_DIR").map(|x| PathBuf::from(x))
        .unwrap_or_else(|| ::std::env::current_dir().unwrap());
    root.join("testdata")
}

#[test]
fn test_load_superblock() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    assert_eq!(header.o_cksum, fletcher64(&buffer[8..]), "cksum");
    assert_eq!(header.o_oid, Oid(1), "oid");
    assert_eq!(header.o_xid, Xid(4), "xid");
    assert_eq!(header.o_type & OBJECT_TYPE_MASK, ObjectType::NxSuperblock as u32, "type");
    assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Ephemeral as u32, "type");
    assert_eq!(header.o_subtype, 0, "subtype");
    assert_eq!(superblock.nx_magic, NX_MAGIC, "magic");
    assert_eq!(superblock.nx_block_size, 4096, "block_size");
    assert_eq!(superblock.nx_block_count, 0x9f6, "block_count");
    assert_eq!(superblock.nx_features, 0, "features");
    assert_eq!(superblock.nx_readonly_compatible_features, 0, "ro_compat_features");
    assert_eq!(superblock.nx_incompatible_features, 2, "imcompat_features");
    assert_eq!(superblock.nx_uuid, Uuid::parse_str("0d8c95d045744d3585d31c9cdb8043bc").unwrap(), "uuid");
    assert_eq!(superblock.nx_next_oid, Oid(0x406), "next_oid");
    assert_eq!(superblock.nx_next_xid, Xid(5), "next_xid");
    assert_eq!(superblock.nx_xp_desc_blocks, 8, "desc blocks");
    assert_eq!(superblock.nx_xp_data_blocks, 52, "data blocks");
    assert_eq!(superblock.nx_xp_desc_base, Paddr(1), "desc base");
    assert_eq!(superblock.nx_xp_data_base, Paddr(9), "data base");
    assert_eq!(superblock.nx_xp_desc_next, 0, "desc next");
    assert_eq!(superblock.nx_xp_data_next, 14, "data next");
    assert_eq!(superblock.nx_xp_desc_index, 6, "desc index");
    assert_eq!(superblock.nx_xp_desc_len, 2, "desc len");
    assert_eq!(superblock.nx_xp_data_index, 10, "data index");
    assert_eq!(superblock.nx_xp_data_len, 4, "data len");
    assert_eq!(superblock.nx_spaceman_oid, Oid(0x400), "spaceman oid");
    assert_eq!(superblock.nx_omap_oid, Oid(0x067), "omap oid");
    assert_eq!(superblock.nx_reaper_oid, Oid(0x401), "reaper oid");
    assert_eq!(superblock.nx_test_type, 0, "test type");
    assert_eq!(superblock.nx_max_file_systems, 1, "max file systems");
    assert_eq!(superblock.nx_fs_oid[0], Oid(0x402), "fs oid");
    assert_eq!(superblock.nx_counters[0], 42, "counters");
    assert_eq!(superblock.nx_blocked_out_prange.pr_start_paddr, Paddr(0), "blocked_out_prange");
    assert_eq!(superblock.nx_blocked_out_prange.pr_block_count, 0, "blocked_out_prange");
    assert_eq!(superblock.nx_evict_mapping_tree_oid, Oid(0), "evict_mapping_tree_oid");
    assert_eq!(superblock.nx_flags, 0, "flags");
    assert_eq!(superblock.nx_efi_jumpstart, Paddr(0), "efi_jumpstart");
    assert_eq!(superblock.nx_fusion_uuid, Uuid::nil(), "fusion_uuid");
    assert_eq!(superblock.nx_keylocker.pr_start_paddr, Paddr(0), "keylocker");
    assert_eq!(superblock.nx_keylocker.pr_block_count, 0, "keylocker");
    assert_eq!(superblock.nx_ephemeral_info[0], 0x0100040001, "ephemeral_info");
    assert_eq!(superblock.nx_test_oid, Oid(0), "test_oid");
    assert_eq!(superblock.nx_fusion_mt_oid, Oid(0), "fusion_mt_oid");
    assert_eq!(superblock.nx_fusion_wbc_oid, Oid(0), "fusion_wbc_oid");
    assert_eq!(superblock.nx_fusion_wbc.pr_start_paddr, Paddr(0), "fusion_wbc");
    assert_eq!(superblock.nx_fusion_wbc.pr_block_count, 0, "fusion_wbc");
    assert_eq!(superblock.nx_newest_mounted_version, 0, "newest_mounted_version");
    assert_eq!(superblock.nx_mkb_locker.pr_start_paddr, Paddr(0), "mkb_locker");
    assert_eq!(superblock.nx_mkb_locker.pr_block_count, 0, "mkb_locker");
}

#[test]
fn test_load_checkpoints() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    assert_eq!(header.o_cksum, fletcher64(&buffer[8..]));
    assert_eq!(superblock.nx_magic, NX_MAGIC);
    for idx in 0..superblock.nx_xp_desc_blocks {
        file.seek(SeekFrom::Start((superblock.nx_xp_desc_base.0 as u64 + idx as u64) * 4096)).unwrap();
        file.read_exact(&mut buffer).unwrap();
        let mut cursor = Cursor::new(&buffer[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        assert_eq!(header.o_cksum, fletcher64(&buffer[8..]));
        if header.o_type & OBJECT_TYPE_MASK == ObjectType::CheckpointMap as u32 {
            println!("Checkpoint map");
            //assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, OBJ_PHYSICAL);
        } else if header.o_type & OBJECT_TYPE_MASK == ObjectType::NxSuperblock as u32 {
            println!("Superblock");
            let mut cursor = Cursor::new(&buffer[..]);
            let header = ObjPhys::import(&mut cursor).unwrap();
            let superblock = NxSuperblock::import(&mut cursor).unwrap();
            assert_eq!(superblock.nx_magic, NX_MAGIC);
            println!("  TX ID: {:?}", header.o_xid);
            println!("  Desc blocks: {}", superblock.nx_xp_desc_blocks);
            println!("  Desc base: {:?}", superblock.nx_xp_desc_base);
            println!("  Desc next: {:?}", superblock.nx_xp_desc_next);
            println!("  Desc index: {:?}", superblock.nx_xp_desc_index);
            println!("  Desc len: {:?}", superblock.nx_xp_desc_len);
            println!("  Data blocks: {}", superblock.nx_xp_data_blocks);
            println!("  Data base: {:?}", superblock.nx_xp_data_base);
            println!("  Data next: {:?}", superblock.nx_xp_data_next);
            println!("  Data index: {:?}", superblock.nx_xp_data_index);
            println!("  Data len: {}", superblock.nx_xp_data_len);
            assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Ephemeral as u32);
        } else {
            panic!("Unrecognized block type");
        }
    }
    //panic!("Dump");
}

#[test]
fn test_load_checkpoint_mappings() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let _header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    assert_eq!(superblock.nx_xp_desc_blocks, 8);
    let idx = 6;
    file.seek(SeekFrom::Start((superblock.nx_xp_desc_base.0 as u64 + idx as u64) * 4096)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let mapping = CheckpointMapPhys::import(&mut cursor).unwrap();
    assert_eq!(header.o_cksum, fletcher64(&buffer[8..]), "cksum");
    assert_eq!(header.o_oid, Oid(7), "oid");
    assert_eq!(header.o_xid, Xid(4), "xid");
    assert_eq!(header.o_type & OBJECT_TYPE_MASK, ObjectType::CheckpointMap as u32, "type");
    //assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, OBJ_PHYSICAL, "type");
    assert_eq!(header.o_subtype, 0, "subtype");
    assert_eq!(mapping.cpm_flags, CpmFlags::CHECKPOINT_MAP_LAST, "flags");
    assert_eq!(mapping.cpm_count, 4, "count");

    assert_eq!(mapping.cpm_map[0].cpm_type & OBJECT_TYPE_MASK, ObjectType::Spaceman as u32, "type");
    assert_eq!(mapping.cpm_map[0].cpm_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Ephemeral as u32, "type");
    assert_eq!(mapping.cpm_map[0].cpm_subtype, 0, "subtype");
    assert_eq!(mapping.cpm_map[0].cpm_size, 4096, "size");
    assert_eq!(mapping.cpm_map[0].cpm_pad, 0, "pad");
    assert_eq!(mapping.cpm_map[0].cpm_fs_oid, Oid(0), "fs oid");
    assert_eq!(mapping.cpm_map[0].cpm_oid, Oid(0x400), "oid");
    assert_eq!(mapping.cpm_map[0].cpm_paddr, Oid(0x13), "paddr");

    assert_eq!(mapping.cpm_map[1].cpm_type & OBJECT_TYPE_MASK, ObjectType::Btree as u32, "type");
    assert_eq!(mapping.cpm_map[1].cpm_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Ephemeral as u32, "type");
    assert_eq!(mapping.cpm_map[1].cpm_subtype, ObjectType::SpacemanFreeQueue as u32, "subtype");
    assert_eq!(mapping.cpm_map[1].cpm_size, 4096, "size");
    assert_eq!(mapping.cpm_map[1].cpm_pad, 0, "pad");
    assert_eq!(mapping.cpm_map[1].cpm_fs_oid, Oid(0), "fs oid");
    assert_eq!(mapping.cpm_map[1].cpm_oid, Oid(0x403), "oid");
    assert_eq!(mapping.cpm_map[1].cpm_paddr, Oid(0x14), "paddr");

    assert_eq!(mapping.cpm_map[2].cpm_type & OBJECT_TYPE_MASK, ObjectType::Btree as u32, "type");
    assert_eq!(mapping.cpm_map[2].cpm_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Ephemeral as u32, "type");
    assert_eq!(mapping.cpm_map[2].cpm_subtype, ObjectType::SpacemanFreeQueue as u32, "subtype");
    assert_eq!(mapping.cpm_map[2].cpm_size, 4096, "size");
    assert_eq!(mapping.cpm_map[2].cpm_pad, 0, "pad");
    assert_eq!(mapping.cpm_map[2].cpm_fs_oid, Oid(0), "fs oid");
    assert_eq!(mapping.cpm_map[2].cpm_oid, Oid(0x405), "oid");
    assert_eq!(mapping.cpm_map[2].cpm_paddr, Oid(0x15), "paddr");

    assert_eq!(mapping.cpm_map[3].cpm_type & OBJECT_TYPE_MASK, ObjectType::NxReaper as u32, "type");
    assert_eq!(mapping.cpm_map[3].cpm_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Ephemeral as u32, "type");
    assert_eq!(mapping.cpm_map[3].cpm_subtype, 0, "subtype");
    assert_eq!(mapping.cpm_map[3].cpm_size, 4096, "size");
    assert_eq!(mapping.cpm_map[3].cpm_pad, 0, "pad");
    assert_eq!(mapping.cpm_map[3].cpm_fs_oid, Oid(0), "fs oid");
    assert_eq!(mapping.cpm_map[3].cpm_oid, Oid(0x401), "oid");
    assert_eq!(mapping.cpm_map[3].cpm_paddr, Oid(0x16), "paddr");
}

#[test]
fn test_load_checkpoint_data() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let _header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    for idx in 0..superblock.nx_xp_data_blocks {
        file.seek(SeekFrom::Start((superblock.nx_xp_data_base.0 as u64 + idx as u64) * 4096)).unwrap();
        file.read_exact(&mut buffer).unwrap();
        let mut cursor = Cursor::new(&buffer[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        if header.o_type == 0 {
            continue;
        }
        assert_eq!(header.o_cksum, fletcher64(&buffer[8..]));
        //if header.o_type & OBJECT_TYPE_MASK == ObjectType::CheckpointMap as u32 {
        //println!("  Data block type: {:?}", header);
        println!("  Data block type: {:?} - {:?}", header.o_type & OBJECT_TYPE_MASK, header.o_subtype);
    }
    //panic!("Dump");
}

#[test]
fn test_load_object_map() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    file.seek(SeekFrom::Start(superblock.nx_omap_oid.0 * 4096)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let omap = OmapPhys::import(&mut cursor).unwrap();
    assert_eq!(header.o_cksum, fletcher64(&buffer[8..]), "cksum");
    assert_eq!(header.o_oid, Oid(0x067), "oid");
    assert_eq!(header.o_xid, Xid(4), "xid");
    assert_eq!(header.o_type & OBJECT_TYPE_MASK, ObjectType::Omap as u32, "type");
    assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Physical as u32, "type");
    assert_eq!(header.o_subtype, 0, "subtype");
    assert_eq!(omap.om_flags, OmFlags::MANUALLY_MANAGED, "flags");
    assert_eq!(omap.om_snap_count, 0, "snap_count");
    assert_eq!(omap.om_tree_type, StorageType::Physical as u32 | ObjectType::Btree as u32, "tree_type");
    assert_eq!(omap.om_snapshot_tree_type, StorageType::Physical as u32 | ObjectType::Btree as u32, "snapshot_tree_type");
    assert_eq!(omap.om_tree_oid, Oid(0x068), "tree_oid");
    assert_eq!(omap.om_snapshot_tree_oid, Oid(0), "snapshot_tree_oid");
    assert_eq!(omap.om_most_recent_snap, Xid(0), "most_recent_snap");
    assert_eq!(omap.om_pending_revert_min, Xid(0), "pending_revert_min");
    assert_eq!(omap.om_pending_revert_max, Xid(0), "pending_revert_max");
}

#[test]
fn test_load_object_map_btree() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    file.seek(SeekFrom::Start(superblock.nx_omap_oid.0 * 4096)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let omap = OmapPhys::import(&mut cursor).unwrap();
    file.seek(SeekFrom::Start(omap.om_tree_oid.0 * 4096)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let node = BtreeNodePhys::import(&mut cursor).unwrap();
    assert_eq!(header.o_cksum, fletcher64(&buffer[8..]), "cksum");
    assert_eq!(header.o_oid, Oid(0x068), "oid");
    assert_eq!(header.o_xid, Xid(4), "xid");
    assert_eq!(header.o_type & OBJECT_TYPE_MASK, ObjectType::Btree as u32, "type");
    assert_eq!(header.o_type & OBJECT_TYPE_FLAGS_MASK, StorageType::Physical as u32, "type");
    assert_eq!(header.o_subtype, ObjectType::Omap as u32, "subtype");
    assert_eq!(node.btn_flags, BtnFlags::ROOT | BtnFlags::LEAF | BtnFlags::FIXED_KV_SIZE, "flags");
    assert_eq!(node.btn_level, 0, "level");
    assert_eq!(node.btn_nkeys, 1, "nkeys");
    assert_eq!(node.btn_table_space.off, 0, "table space off");
    assert_eq!(node.btn_table_space.len, 0x01c0, "table space len");
    assert_eq!(node.btn_free_space.off, 0x20, "free space off");
    assert_eq!(node.btn_free_space.len, 0x0da0, "free space len");
    assert_eq!(node.btn_key_free_list.off, 0x10, "key free list off");
    assert_eq!(node.btn_key_free_list.len, 0x0010, "key free list len");
    assert_eq!(node.btn_val_free_list.off, 0x20, "val free list off");
    assert_eq!(node.btn_val_free_list.len, 0x0010, "val free list len");
    let mut cursor = Cursor::new(&node.btn_data[..]);
    let mut entries = Vec::new();
    for _ in 0..node.btn_table_space.len/4 {
        entries.push(KVoff::import(&mut cursor).unwrap());
    }
    assert_eq!(entries[0].k, 0, "table entry 0 key off");
    assert_eq!(entries[0].v, 0x0010, "table entry 0 val off");
    assert_eq!(entries[1].k, 0, "table entry 1 key off");
    assert_eq!(entries[1].v, 0x0010, "table entry 1 val off");
    assert_eq!(entries[2].k, 0, "table entry 2 key off");
    assert_eq!(entries[2].v, 0x0000, "table entry 2 val off");
    let key = OmapKey::import(&mut cursor).unwrap();
    let mut cursor = Cursor::new(&node.btn_data[node.btn_data.len()-40-entries[0].v as usize..node.btn_data.len()-40]);
    let value = OmapVal::import(&mut cursor).unwrap();
    assert_eq!(key.oid, Oid(0x402), "key oid");
    assert_eq!(key.xid, Xid(4), "key xid");
    assert_eq!(value.ov_flags, OvFlags::empty(), "value flags");
    assert_eq!(value.ov_size, 4096, "value size");
    assert_eq!(value.ov_paddr, Paddr(0x66), "value paddr");
    let mut cursor = Cursor::new(&buffer[4096-40..]);
    let info = BtreeInfo::import(&mut cursor).unwrap();
    assert_eq!(info.bt_fixed.bt_flags, BtFlags::SEQUENTIAL_INSERT | BtFlags::PHYSICAL, "flags");
    assert_eq!(info.bt_fixed.bt_node_size, 4096, "node size");
    assert_eq!(info.bt_fixed.bt_key_size, 16, "key size");
    assert_eq!(info.bt_fixed.bt_val_size, 16, "val size");
    assert_eq!(info.bt_longest_key, 16, "longest key");
    assert_eq!(info.bt_longest_val, 16, "longest val");
    assert_eq!(info.bt_key_count, 1, "key count");
    assert_eq!(info.bt_node_count, 1, "node count");
}
