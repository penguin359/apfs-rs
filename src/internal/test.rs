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
    let mut buffer = [0u8; NX_DEFAULT_BLOCK_SIZE];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    assert_eq!(header.cksum, fletcher64(&buffer[8..]), "bad checksum");
    assert_eq!(header.oid, Oid(1), "oid");
    assert_eq!(header.xid, Xid(4), "xid");
    assert_eq!(header.r#type.r#type(), ObjectType::NxSuperblock, "type");
    assert_eq!(header.r#type.storage(), StorageType::Ephemeral, "storage");
    assert_eq!(header.r#type.flags(), ObjTypeFlags::empty(), "flags");
    assert_eq!(header.subtype.r#type(), ObjectType::Invalid, "subtype");
    assert_eq!(superblock.magic, NX_MAGIC, "magic");
    assert_eq!(superblock.block_size, 4096, "block_size");
    assert_eq!(superblock.block_count, 0x9f6, "block_count");
    assert_eq!(superblock.features, SuperblockFeatureFlags::empty(), "features");
    assert_eq!(superblock.readonly_compatible_features, SuperblockRocompatFlags::empty(), "ro_compat_features");
    assert_eq!(superblock.incompatible_features, SuperblockIncompatFlags::VERSION2, "imcompat_features");
    assert_eq!(superblock.uuid, Uuid::parse_str("0d8c95d045744d3585d31c9cdb8043bc").unwrap(), "uuid");
    assert_eq!(superblock.next_oid, Oid(0x406), "next_oid");
    assert_eq!(superblock.next_xid, Xid(5), "next_xid");
    assert_eq!(superblock.xp_desc_blocks, 8, "desc blocks");
    assert_eq!(superblock.xp_data_blocks, 52, "data blocks");
    assert_eq!(superblock.xp_desc_base, Paddr(1), "desc base");
    assert_eq!(superblock.xp_data_base, Paddr(9), "data base");
    assert_eq!(superblock.xp_desc_next, 0, "desc next");
    assert_eq!(superblock.xp_data_next, 14, "data next");
    assert_eq!(superblock.xp_desc_index, 6, "desc index");
    assert_eq!(superblock.xp_desc_len, 2, "desc len");
    assert_eq!(superblock.xp_data_index, 10, "data index");
    assert_eq!(superblock.xp_data_len, 4, "data len");
    assert_eq!(superblock.spaceman_oid, Oid(0x400), "spaceman oid");
    assert_eq!(superblock.omap_oid, Oid(0x067), "omap oid");
    assert_eq!(superblock.reaper_oid, Oid(0x401), "reaper oid");
    assert_eq!(superblock.test_type, 0, "test type");
    assert_eq!(superblock.max_file_systems, 1, "max file systems");
    assert_eq!(superblock.fs_oid[0], Oid(0x402), "fs oid");
    assert_eq!(superblock.counters[0], 42, "counters");
    assert_eq!(superblock.blocked_out_prange.start_paddr, Paddr(0), "blocked_out_prange");
    assert_eq!(superblock.blocked_out_prange.block_count, 0, "blocked_out_prange");
    assert_eq!(superblock.evict_mapping_tree_oid, Oid(0), "evict_mapping_tree_oid");
    assert_eq!(superblock.flags, SuperblockFlags::empty(), "flags");
    assert_eq!(superblock.efi_jumpstart, Paddr(0), "efi_jumpstart");
    assert_eq!(superblock.fusion_uuid, Uuid::nil(), "fusion_uuid");
    assert_eq!(superblock.keylocker.start_paddr, Paddr(0), "keylocker");
    assert_eq!(superblock.keylocker.block_count, 0, "keylocker");
    assert_eq!(superblock.ephemeral_info[0], 0x0100040001, "ephemeral_info");
    assert_eq!(superblock.test_oid, Oid(0), "test_oid");
    assert_eq!(superblock.fusion_mt_oid, Oid(0), "fusion_mt_oid");
    assert_eq!(superblock.fusion_wbc_oid, Oid(0), "fusion_wbc_oid");
    assert_eq!(superblock.fusion_wbc.start_paddr, Paddr(0), "fusion_wbc");
    assert_eq!(superblock.fusion_wbc.block_count, 0, "fusion_wbc");
    assert_eq!(superblock.newest_mounted_version, 0, "newest_mounted_version");
    assert_eq!(superblock.mkb_locker.start_paddr, Paddr(0), "mkb_locker");
    assert_eq!(superblock.mkb_locker.block_count, 0, "mkb_locker");
}

#[test]
#[cfg_attr(not(feature = "expensive_tests"), ignore)]
fn test_load_superblock_16k() {
    let mut buffer = [0u8; NX_DEFAULT_BLOCK_SIZE];
    let mut file = File::open(test_dir().join("apfs-16k-cs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    let block_size = superblock.block_size;
    let mut buffer = vec![0u8; block_size as usize];
    file.seek(SeekFrom::Start(0));
    file.read_exact(&mut buffer).unwrap();
    assert_eq!(header.cksum, fletcher64(&buffer[8..]), "bad checksum");
    // assert_eq!(header.oid, Oid(1), "oid");
    // assert_eq!(header.xid, Xid(4), "xid");
    // assert_eq!(header.r#type.r#type(), ObjectType::NxSuperblock, "type");
    // assert_eq!(header.r#type.storage(), StorageType::Ephemeral, "type");
    // assert_eq!(header.subtype, 0, "subtype");
    // assert_eq!(superblock.magic, NX_MAGIC, "magic");
    // assert_eq!(superblock.block_size, 4096, "block_size");
    // assert_eq!(superblock.block_count, 0x9f6, "block_count");
    // assert_eq!(superblock.features, SuperblockFeatureFlags::empty(), "features");
    // assert_eq!(superblock.readonly_compatible_features, SuperblockRocompatFlags::empty(), "ro_compat_features");
    // assert_eq!(superblock.incompatible_features, SuperblockIncompatFlags::VERSION2, "imcompat_features");
    // assert_eq!(superblock.uuid, Uuid::parse_str("0d8c95d045744d3585d31c9cdb8043bc").unwrap(), "uuid");
    // assert_eq!(superblock.next_oid, Oid(0x406), "next_oid");
    // assert_eq!(superblock.next_xid, Xid(5), "next_xid");
    // assert_eq!(superblock.xp_desc_blocks, 8, "desc blocks");
    // assert_eq!(superblock.xp_data_blocks, 52, "data blocks");
    // assert_eq!(superblock.xp_desc_base, Paddr(1), "desc base");
    // assert_eq!(superblock.xp_data_base, Paddr(9), "data base");
    // assert_eq!(superblock.xp_desc_next, 0, "desc next");
    // assert_eq!(superblock.xp_data_next, 14, "data next");
    // assert_eq!(superblock.xp_desc_index, 6, "desc index");
    // assert_eq!(superblock.xp_desc_len, 2, "desc len");
    // assert_eq!(superblock.xp_data_index, 10, "data index");
    // assert_eq!(superblock.xp_data_len, 4, "data len");
    // assert_eq!(superblock.spaceman_oid, Oid(0x400), "spaceman oid");
    // assert_eq!(superblock.omap_oid, Oid(0x067), "omap oid");
    // assert_eq!(superblock.reaper_oid, Oid(0x401), "reaper oid");
    // assert_eq!(superblock.test_type, 0, "test type");
    // assert_eq!(superblock.max_file_systems, 1, "max file systems");
    // assert_eq!(superblock.fs_oid[0], Oid(0x402), "fs oid");
    // assert_eq!(superblock.counters[0], 42, "counters");
    // assert_eq!(superblock.blocked_out_prange.start_paddr, Paddr(0), "blocked_out_prange");
    // assert_eq!(superblock.blocked_out_prange.block_count, 0, "blocked_out_prange");
    // assert_eq!(superblock.evict_mapping_tree_oid, Oid(0), "evict_mapping_tree_oid");
    // assert_eq!(superblock.flags, SuperblockFlags::empty(), "flags");
    // assert_eq!(superblock.efi_jumpstart, Paddr(0), "efi_jumpstart");
    // assert_eq!(superblock.fusion_uuid, Uuid::nil(), "fusion_uuid");
    // assert_eq!(superblock.keylocker.start_paddr, Paddr(0), "keylocker");
    // assert_eq!(superblock.keylocker.block_count, 0, "keylocker");
    // assert_eq!(superblock.ephemeral_info[0], 0x0100040001, "ephemeral_info");
    // assert_eq!(superblock.test_oid, Oid(0), "test_oid");
    // assert_eq!(superblock.fusion_mt_oid, Oid(0), "fusion_mt_oid");
    // assert_eq!(superblock.fusion_wbc_oid, Oid(0), "fusion_wbc_oid");
    // assert_eq!(superblock.fusion_wbc.start_paddr, Paddr(0), "fusion_wbc");
    // assert_eq!(superblock.fusion_wbc.block_count, 0, "fusion_wbc");
    // assert_eq!(superblock.newest_mounted_version, 0, "newest_mounted_version");
    // assert_eq!(superblock.mkb_locker.start_paddr, Paddr(0), "mkb_locker");
    // assert_eq!(superblock.mkb_locker.block_count, 0, "mkb_locker");
}

#[test]
fn test_load_checkpoints() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    assert_eq!(header.cksum, fletcher64(&buffer[8..]));
    assert_eq!(superblock.magic, NX_MAGIC);
    for idx in 0..superblock.xp_desc_blocks {
        file.seek(SeekFrom::Start((superblock.xp_desc_base.0 as u64 + idx as u64) * 4096)).unwrap();
        file.read_exact(&mut buffer).unwrap();
        let mut cursor = Cursor::new(&buffer[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        assert_eq!(header.cksum, fletcher64(&buffer[8..]));
        if header.r#type.r#type() == ObjectType::CheckpointMap {
            println!("Checkpoint map");
            //assert_eq!(header.o_type.storage(), OBJ_PHYSICAL);
        } else if header.r#type.r#type() == ObjectType::NxSuperblock {
            println!("Superblock");
            let mut cursor = Cursor::new(&buffer[..]);
            let header = ObjPhys::import(&mut cursor).unwrap();
            let superblock = NxSuperblock::import(&mut cursor).unwrap();
            assert_eq!(superblock.magic, NX_MAGIC);
            println!("  TX ID: {:?}", header.xid);
            println!("  Desc blocks: {}", superblock.xp_desc_blocks);
            println!("  Desc base: {:?}", superblock.xp_desc_base);
            println!("  Desc next: {:?}", superblock.xp_desc_next);
            println!("  Desc index: {:?}", superblock.xp_desc_index);
            println!("  Desc len: {:?}", superblock.xp_desc_len);
            println!("  Data blocks: {}", superblock.xp_data_blocks);
            println!("  Data base: {:?}", superblock.xp_data_base);
            println!("  Data next: {:?}", superblock.xp_data_next);
            println!("  Data index: {:?}", superblock.xp_data_index);
            println!("  Data len: {}", superblock.xp_data_len);
            assert_eq!(header.r#type.storage(), StorageType::Ephemeral);
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
    assert_eq!(superblock.xp_desc_blocks, 8);
    let idx = 6;
    file.seek(SeekFrom::Start((superblock.xp_desc_base.0 as u64 + idx as u64) * 4096)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let mapping = CheckpointMapPhys::import(&mut cursor).unwrap();
    assert_eq!(header.cksum, fletcher64(&buffer[8..]), "bad checksum");
    assert_eq!(header.oid, Oid(7), "oid");
    assert_eq!(header.xid, Xid(4), "xid");
    assert_eq!(header.r#type.r#type(), ObjectType::CheckpointMap, "type");
    assert_eq!(header.r#type.storage(), StorageType::Physical, "storage");
    assert_eq!(header.r#type.flags(), ObjTypeFlags::empty(), "flags");
    assert_eq!(header.subtype.r#type(), ObjectType::Invalid, "subtype");
    assert_eq!(mapping.flags, CpmFlags::LAST, "flags");
    assert_eq!(mapping.count, 4, "count");

    assert_eq!(mapping.map[0].r#type.r#type(), ObjectType::Spaceman, "type");
    assert_eq!(mapping.map[0].r#type.storage(), StorageType::Ephemeral, "storage");
    assert_eq!(mapping.map[0].subtype.r#type(), ObjectType::Invalid, "subtype");
    assert_eq!(mapping.map[0].size, 4096, "size");
    assert_eq!(mapping.map[0].pad, 0, "pad");
    assert_eq!(mapping.map[0].fs_oid, Oid(0), "fs oid");
    assert_eq!(mapping.map[0].oid, Oid(0x400), "oid");
    assert_eq!(mapping.map[0].paddr, Oid(0x13), "paddr");

    assert_eq!(mapping.map[1].r#type.r#type(), ObjectType::Btree, "type");
    assert_eq!(mapping.map[1].r#type.storage(), StorageType::Ephemeral, "type");
    assert_eq!(mapping.map[1].subtype.r#type(), ObjectType::SpacemanFreeQueue, "subtype");
    assert_eq!(mapping.map[1].size, 4096, "size");
    assert_eq!(mapping.map[1].pad, 0, "pad");
    assert_eq!(mapping.map[1].fs_oid, Oid(0), "fs oid");
    assert_eq!(mapping.map[1].oid, Oid(0x403), "oid");
    assert_eq!(mapping.map[1].paddr, Oid(0x14), "paddr");

    assert_eq!(mapping.map[2].r#type.r#type(), ObjectType::Btree, "type");
    assert_eq!(mapping.map[2].r#type.storage(), StorageType::Ephemeral, "type");
    assert_eq!(mapping.map[2].subtype.r#type(), ObjectType::SpacemanFreeQueue, "subtype");
    assert_eq!(mapping.map[2].size, 4096, "size");
    assert_eq!(mapping.map[2].pad, 0, "pad");
    assert_eq!(mapping.map[2].fs_oid, Oid(0), "fs oid");
    assert_eq!(mapping.map[2].oid, Oid(0x405), "oid");
    assert_eq!(mapping.map[2].paddr, Oid(0x15), "paddr");

    assert_eq!(mapping.map[3].r#type.r#type(), ObjectType::NxReaper, "type");
    assert_eq!(mapping.map[3].r#type.storage(), StorageType::Ephemeral, "type");
    assert_eq!(mapping.map[3].subtype.r#type(), ObjectType::Invalid, "subtype");
    assert_eq!(mapping.map[3].size, 4096, "size");
    assert_eq!(mapping.map[3].pad, 0, "pad");
    assert_eq!(mapping.map[3].fs_oid, Oid(0), "fs oid");
    assert_eq!(mapping.map[3].oid, Oid(0x401), "oid");
    assert_eq!(mapping.map[3].paddr, Oid(0x16), "paddr");
}

#[test]
fn test_load_checkpoint_data() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let _header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    for idx in 0..superblock.xp_data_blocks {
        file.seek(SeekFrom::Start((superblock.xp_data_base.0 as u64 + idx as u64) * 4096)).unwrap();
        file.read_exact(&mut buffer).unwrap();
        let mut cursor = Cursor::new(&buffer[..]);
        let header = ObjPhys::import(&mut cursor).unwrap();
        if header.r#type.r#type() == ObjectType::Invalid {
            continue;
        }
        assert_eq!(header.cksum, fletcher64(&buffer[8..]));
        //if header.o_type.r#type() == ObjectType::CheckpointMap {
        //println!("  Data block type: {:?}", header);
        println!("  Data block type: {:?} - {:?}", header.r#type.r#type(), header.subtype.r#type());
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
    file.seek(SeekFrom::Start(superblock.omap_oid.0 * 4096)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let omap = OmapPhys::import(&mut cursor).unwrap();
    assert_eq!(header.cksum, fletcher64(&buffer[8..]), "bad checksum");
    assert_eq!(header.oid, Oid(0x067), "oid");
    assert_eq!(header.xid, Xid(4), "xid");
    assert_eq!(header.r#type.r#type(), ObjectType::Omap, "type");
    assert_eq!(header.r#type.storage(), StorageType::Physical, "storage");
    assert_eq!(header.subtype.r#type(), ObjectType::Invalid, "subtype");
    assert_eq!(omap.flags, OmFlags::MANUALLY_MANAGED, "flags");
    assert_eq!(omap.snap_count, 0, "snap_count");
    assert_eq!(omap.tree_type.r#type(), ObjectType::Btree, "tree type");
    assert_eq!(omap.tree_type.storage(), StorageType::Physical, "tree storage");
    assert_eq!(omap.tree_type.flags(), ObjTypeFlags::empty(), "tree flags");
    assert_eq!(omap.snapshot_tree_type.r#type(), ObjectType::Btree, "snapshot_tree type");
    assert_eq!(omap.snapshot_tree_type.storage(), StorageType::Physical, "snapshot_tree storage");
    assert_eq!(omap.snapshot_tree_type.flags(), ObjTypeFlags::empty(), "snapshot_tree flags");
    assert_eq!(omap.tree_oid, Oid(0x068), "tree_oid");
    assert_eq!(omap.snapshot_tree_oid, Oid(0), "snapshot_tree_oid");
    assert_eq!(omap.most_recent_snap, Xid(0), "most_recent_snap");
    assert_eq!(omap.pending_revert_min, Xid(0), "pending_revert_min");
    assert_eq!(omap.pending_revert_max, Xid(0), "pending_revert_max");
}

#[test]
fn test_load_object_map_btree() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let superblock = NxSuperblock::import(&mut cursor).unwrap();
    file.seek(SeekFrom::Start(superblock.omap_oid.0 * 4096)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let omap = OmapPhys::import(&mut cursor).unwrap();
    file.seek(SeekFrom::Start(omap.tree_oid.0 * 4096)).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).unwrap();
    let node = BtreeNodePhys::import(&mut cursor).unwrap();
    assert_eq!(header.cksum, fletcher64(&buffer[8..]), "bad checksum");
    assert_eq!(header.oid, Oid(0x068), "oid");
    assert_eq!(header.xid, Xid(4), "xid");
    assert_eq!(header.r#type.r#type(), ObjectType::Btree, "type");
    assert_eq!(header.r#type.storage(), StorageType::Physical, "storage");
    assert_eq!(header.subtype.r#type(), ObjectType::Omap, "subtype");
    assert_eq!(node.flags, BtnFlags::ROOT | BtnFlags::LEAF | BtnFlags::FIXED_KV_SIZE, "flags");
    assert_eq!(node.level, 0, "level");
    assert_eq!(node.nkeys, 1, "nkeys");
    assert_eq!(node.table_space.off, 0, "table space off");
    assert_eq!(node.table_space.len, 0x01c0, "table space len");
    assert_eq!(node.free_space.off, 0x20, "free space off");
    assert_eq!(node.free_space.len, 0x0da0, "free space len");
    assert_eq!(node.key_free_list.off, 0x10, "key free list off");
    assert_eq!(node.key_free_list.len, 0x0010, "key free list len");
    assert_eq!(node.val_free_list.off, 0x20, "val free list off");
    assert_eq!(node.val_free_list.len, 0x0010, "val free list len");
    let mut cursor = Cursor::new(&node.data[..]);
    let mut entries = Vec::new();
    for _ in 0..node.table_space.len/4 {
        entries.push(KVoff::import(&mut cursor).unwrap());
    }
    assert_eq!(entries[0].k, 0, "table entry 0 key off");
    assert_eq!(entries[0].v, 0x0010, "table entry 0 val off");
    assert_eq!(entries[1].k, 0, "table entry 1 key off");
    assert_eq!(entries[1].v, 0x0010, "table entry 1 val off");
    assert_eq!(entries[2].k, 0, "table entry 2 key off");
    assert_eq!(entries[2].v, 0x0000, "table entry 2 val off");
    let key = OmapKey::import(&mut cursor).unwrap();
    let mut cursor = Cursor::new(&node.data[node.data.len()-40-entries[0].v as usize..node.data.len()-40]);
    let value = OmapVal::import(&mut cursor).unwrap();
    assert_eq!(key.oid, Oid(0x402), "key oid");
    assert_eq!(key.xid, Xid(4), "key xid");
    assert_eq!(value.flags, OvFlags::empty(), "value flags");
    assert_eq!(value.size, 4096, "value size");
    assert_eq!(value.paddr, Paddr(0x66), "value paddr");
    let mut cursor = Cursor::new(&buffer[buffer.len()-40..]);
    let info = BtreeInfo::import(&mut cursor).unwrap();
    assert_eq!(info.fixed.flags, BtFlags::SEQUENTIAL_INSERT | BtFlags::PHYSICAL, "flags");
    assert_eq!(info.fixed.node_size, 4096, "node size");
    assert_eq!(info.fixed.key_size, 16, "key size");
    assert_eq!(info.fixed.val_size, 16, "val size");
    assert_eq!(info.longest_key, 16, "longest key");
    assert_eq!(info.longest_val, 16, "longest val");
    assert_eq!(info.key_count, 1, "key count");
    assert_eq!(info.node_count, 1, "node count");
}

#[test]
fn test_load_object_map_btree_mock() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("btree.blob")).expect("Unable to load blob");
    file.read_exact(&mut buffer).expect("Short read");
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).expect("failed to decoded object header");
    let node = BtreeNodePhys::import(&mut cursor).expect("failed to decoded b-tree header");
    assert_eq!(header.cksum, fletcher64(&buffer[8..]), "bad checksum");
    assert_eq!(header.oid, Oid(0x068), "oid");
    assert_eq!(header.xid, Xid(4), "xid");
    assert_eq!(header.r#type.r#type(), ObjectType::Btree, "type");
    assert_eq!(header.r#type.storage(), StorageType::Physical, "storage");
    assert_eq!(header.subtype.r#type(), ObjectType::Omap, "subtype");
    assert_eq!(node.flags, BtnFlags::ROOT | BtnFlags::LEAF | BtnFlags::FIXED_KV_SIZE, "flags");
    assert_eq!(node.level, 0, "level");
    assert_eq!(node.nkeys, 6, "nkeys");
    assert_eq!(node.table_space.off, 0, "table space off");
    assert_eq!(node.table_space.len, 0x01c0, "table space len");
    assert_eq!(node.free_space.off, 0x20, "free space off");
    assert_eq!(node.free_space.len, 0x0da0, "free space len");
    assert_eq!(node.key_free_list.off, 0x10, "key free list off");
    assert_eq!(node.key_free_list.len, 0x0010, "key free list len");
    assert_eq!(node.val_free_list.off, 0x20, "val free list off");
    assert_eq!(node.val_free_list.len, 0x0010, "val free list len");
    let mut cursor = Cursor::new(&node.data[..]);
    let mut entries = Vec::new();
    for _ in 0..node.table_space.len/4 {
        entries.push(KVoff::import(&mut cursor).expect("failed to decoded b-tree toc"));
    }
    assert_eq!(entries[0].k, 0x0000, "table entry 0 key off");
    assert_eq!(entries[0].v, 0x0010, "table entry 0 val off");
    assert_eq!(entries[1].k, 0x0010, "table entry 1 key off");
    assert_eq!(entries[1].v, 0x0020, "table entry 1 val off");
    assert_eq!(entries[2].k, 0x0020, "table entry 2 key off");
    assert_eq!(entries[2].v, 0x0030, "table entry 2 val off");
    assert_eq!(entries[3].k, 0x0030, "table entry 3 key off");
    assert_eq!(entries[3].v, 0x0040, "table entry 3 val off");
    assert_eq!(entries[4].k, 0x0040, "table entry 4 key off");
    assert_eq!(entries[4].v, 0x0050, "table entry 4 val off");
    assert_eq!(entries[5].k, 0x0050, "table entry 5 key off");
    assert_eq!(entries[5].v, 0x0060, "table entry 5 val off");
    assert_eq!(entries[6].k, 0x0000, "table entry 6 key off");
    assert_eq!(entries[6].v, 0x0000, "table entry 6 val off");
    let mut keys = Vec::new();
    for _ in 0..node.nkeys {
        keys.push(OmapKey::import(&mut cursor).expect("failed to decode omap key"));
    }
    assert_eq!(keys[0].oid, Oid(0x0586), "key 0 oid");
    assert_eq!(keys[0].xid, Xid(0x2000), "key 0 xid");
    assert_eq!(keys[1].oid, Oid(0x0588), "key 1 oid");
    assert_eq!(keys[1].xid, Xid(0x2101), "key 1 xid");
    assert_eq!(keys[2].oid, Oid(0x0588), "key 2 oid");
    assert_eq!(keys[2].xid, Xid(0x2202), "key 2 xid");
    assert_eq!(keys[3].oid, Oid(0x0588), "key 3 oid");
    assert_eq!(keys[3].xid, Xid(0x2300), "key 3 xid");
    assert_eq!(keys[4].oid, Oid(0x0589), "key 4 oid");
    assert_eq!(keys[4].xid, Xid(0x1000), "key 4 xid");
    assert_eq!(keys[5].oid, Oid(0x0589), "key 5 oid");
    assert_eq!(keys[5].xid, Xid(0x2000), "key 5 xid");
    let mut cursor = Cursor::new(&node.data[node.data.len()-40-entries[5].v as usize..node.data.len()-40]);
    let mut values = Vec::new();
    for _ in 0..node.nkeys {
        values.insert(0, OmapVal::import(&mut cursor).expect("failed to decode omap value"));
    }
    assert_eq!(values[0].flags, OvFlags::empty(), "value 0 flags");
    assert_eq!(values[0].size, 4096,              "value 0 size");
    assert_eq!(values[0].paddr, Paddr(0x400),     "value 0 paddr");
    assert_eq!(values[1].flags, OvFlags::empty(), "value 1 flags");
    assert_eq!(values[1].size, 4096,              "value 1 size");
    assert_eq!(values[1].paddr, Paddr(0x200),     "value 1 paddr");
    assert_eq!(values[2].flags, OvFlags::empty(), "value 2 flags");
    assert_eq!(values[2].size, 4096,              "value 2 size");
    assert_eq!(values[2].paddr, Paddr(0x300),     "value 2 paddr");
    assert_eq!(values[3].flags, OvFlags::empty(), "value 3 flags");
    assert_eq!(values[3].size, 4096,              "value 3 size");
    assert_eq!(values[3].paddr, Paddr(0x100),     "value 3 paddr");
    assert_eq!(values[4].flags, OvFlags::empty(), "value 4 flags");
    assert_eq!(values[4].size, 4096,              "value 4 size");
    assert_eq!(values[4].paddr, Paddr(0x500),     "value 4 paddr");
    assert_eq!(values[5].flags, OvFlags::empty(), "value 5 flags");
    assert_eq!(values[5].size, 4096,              "value 5 size");
    assert_eq!(values[5].paddr, Paddr(0x600),     "value 5 paddr");
    let mut cursor = Cursor::new(&buffer[buffer.len()-40..]);
    let info = BtreeInfo::import(&mut cursor).expect("failed to decoded b-tree info");
    assert_eq!(info.fixed.flags, BtFlags::SEQUENTIAL_INSERT | BtFlags::PHYSICAL, "flags");
    assert_eq!(info.fixed.node_size, 4096, "node size");
    assert_eq!(info.fixed.key_size, 16, "key size");
    assert_eq!(info.fixed.val_size, 16, "val size");
    assert_eq!(info.longest_key, 16, "longest key");
    assert_eq!(info.longest_val, 16, "longest val");
    assert_eq!(info.key_count, 6, "key count");
    assert_eq!(info.node_count, 6, "node count");
}

#[test]
fn test_load_non_leaf_object_map_btree() {
    let mut buffer = [0u8; 4096];
    let mut file = File::open(test_dir().join("object-map-root-nonleaf.blob")).unwrap();
    file.read_exact(&mut buffer).unwrap();
    let mut cursor = Cursor::new(&buffer[..]);
    let header = ObjPhys::import(&mut cursor).expect("failed to decoded object header");
    let node = BtreeNodePhys::import(&mut cursor).expect("failed to decoded b-tree header");
    assert_eq!(header.cksum, fletcher64(&buffer[8..]), "bad checksum");
    assert_eq!(header.oid, Oid(0x1047d4), "oid");
    assert_eq!(header.xid, Xid(0x95fc62), "xid");
    assert_eq!(header.r#type.r#type(), ObjectType::Btree, "type");
    assert_eq!(header.r#type.storage(), StorageType::Physical, "storage");
    assert_eq!(header.subtype.r#type(), ObjectType::Omap, "subtype");
    assert_eq!(node.flags, BtnFlags::ROOT | BtnFlags::FIXED_KV_SIZE, "flags");
    assert_eq!(node.level, 2, "level");
    assert_eq!(node.nkeys, 85, "nkeys");
    assert_eq!(node.table_space.off, 0, "table space off");
    assert_eq!(node.table_space.len, 0x0240, "table space len");
    assert_eq!(node.free_space.off, 0x640, "free space off");
    assert_eq!(node.free_space.len, 0x400, "free space len");
    assert_eq!(node.key_free_list.off, 0x10, "key free list off");
    assert_eq!(node.key_free_list.len, 0xf0, "key free list len");
    assert_eq!(node.val_free_list.off, 0x10, "val free list off");
    assert_eq!(node.val_free_list.len, 0x0078, "val free list len");
    let mut cursor = Cursor::new(&node.data[..]);
    let mut entries = Vec::new();
    for _ in 0..node.table_space.len/4 {
        entries.push(KVoff::import(&mut cursor).unwrap());
    }
    assert_eq!(entries[0].k, 0, "table entry 0 key off");
    assert_eq!(entries[0].v, 0x8, "table entry 0 val off");
    assert_eq!(entries[1].k, 0x30, "table entry 1 key off");
    assert_eq!(entries[1].v, 0x20, "table entry 1 val off");
    assert_eq!(entries[2].k, 0x440, "table entry 2 key off");
    assert_eq!(entries[2].v, 0x228, "table entry 2 val off");
    let mut cursor = Cursor::new(&node.data[node.table_space.len as usize + entries[0].k as usize..]);
    let key = OmapKey::import(&mut cursor).unwrap();
    let mut cursor = Cursor::new(&node.data[node.data.len()-40-entries[0].v as usize..node.data.len()-40]);
    let value = Oid::import(&mut cursor).unwrap();
    assert_eq!(key.oid, Oid(0x404), "key oid");
    assert_eq!(key.xid, Xid(0x95d8c3), "key xid");
    assert_eq!(value.0, 0x107ab1, "value oid");
    let mut cursor = Cursor::new(&node.data[node.table_space.len as usize + entries[1].k as usize..]);
    let key = OmapKey::import(&mut cursor).unwrap();
    let mut cursor = Cursor::new(&node.data[node.data.len()-40-entries[1].v as usize..node.data.len()-40]);
    let value = Oid::import(&mut cursor).unwrap();
    assert_eq!(key.oid, Oid(0x2eda), "key oid");
    assert_eq!(key.xid, Xid(0x6), "key xid");
    assert_eq!(value.0, 0x148050, "value oid");
    let mut cursor = Cursor::new(&node.data[node.table_space.len as usize + entries[2].k as usize..]);
    let key = OmapKey::import(&mut cursor).unwrap();
    let mut cursor = Cursor::new(&node.data[node.data.len()-40-entries[2].v as usize..node.data.len()-40]);
    let value = Oid::import(&mut cursor).unwrap();
    assert_eq!(key.oid, Oid(0x5807), "key oid");
    assert_eq!(key.xid, Xid(0x8de0ea), "key xid");
    assert_eq!(value.0, 0x1447ea, "value oid");
    let mut cursor = Cursor::new(&buffer[buffer.len()-40..]);
    let info = BtreeInfo::import(&mut cursor).unwrap();
    assert_eq!(info.fixed.flags, BtFlags::SEQUENTIAL_INSERT | BtFlags::PHYSICAL, "flags");
    assert_eq!(info.fixed.node_size, 4096, "node size");
    assert_eq!(info.fixed.key_size, 16, "key size");
    assert_eq!(info.fixed.val_size, 16, "val size");
    assert_eq!(info.longest_key, 16, "longest key");
    assert_eq!(info.longest_val, 16, "longest val");
    assert_eq!(info.key_count, 0x08a167, "key count");
    assert_eq!(info.node_count, 0x1f36, "node count");
}
