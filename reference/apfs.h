#include <stdint.h>

/**
 * General-Purpose Types
 */

typedef int64_t paddr_t;

struct prange {
	paddr_t pr_start_paddr;
	uint64_t pr_block_count;
};
typedef struct prange prange_t;

typedef unsigned char uuid_t[16];


/**
 * Objects
 */

typedef uint64_t oid_t;
typedef uint64_t xid_t;

#define OID_NX_SUPERBLOCK 1

#define OID_INVALID 0ULL
#define OID_RESERVED_COUNT 1024

#define MAX_CKSUM_SIZE 8

struct obj_phys {
uint8_t o_cksum[MAX_CKSUM_SIZE];
	oid_t o_oid;
	xid_t o_xid;
	uint32_t o_type;
	uint32_t o_subtype;
};
typedef struct obj_phys obj_phys_t;

#define OBJECT_TYPE_MASK 0x0000ffff
#define OBJECT_TYPE_FLAGS_MASK 0xffff0000

#define OBJ_STORAGETYPE_MASK 0xc0000000
#define OBJECT_TYPE_FLAGS_DEFINED_MASK 0xf8000000


#define OBJECT_TYPE_NX_SUPERBLOCK 0x00000001

#define OBJECT_TYPE_BTREE 0x00000002
#define OBJECT_TYPE_BTREE_NODE 0x00000003

#define OBJECT_TYPE_SPACEMAN 0x00000005
#define OBJECT_TYPE_SPACEMAN_CAB 0x00000006
#define OBJECT_TYPE_SPACEMAN_CIB 0x00000007
#define OBJECT_TYPE_SPACEMAN_BITMAP 0x00000008
#define OBJECT_TYPE_SPACEMAN_FREE_QUEUE 0x00000009

#define OBJECT_TYPE_EXTENT_LIST_TREE 0x0000000a
#define OBJECT_TYPE_OMAP 0x0000000b
#define OBJECT_TYPE_CHECKPOINT_MAP 0x0000000c

#define OBJECT_TYPE_FS 0x0000000d
#define OBJECT_TYPE_FSTREE 0x0000000e
#define OBJECT_TYPE_BLOCKREFTREE 0x0000000f
#define OBJECT_TYPE_SNAPMETATREE 0x00000010

#define OBJECT_TYPE_NX_REAPER 0x00000011
#define OBJECT_TYPE_NX_REAP_LIST 0x00000012
#define OBJECT_TYPE_OMAP_SNAPSHOT 0x00000013
#define OBJECT_TYPE_EFI_JUMPSTART 0x00000014
#define OBJECT_TYPE_FUSION_MIDDLE_TREE 0x00000015
#define OBJECT_TYPE_NX_FUSION_WBC 0x00000016
#define OBJECT_TYPE_NX_FUSION_WBC_LIST 0x00000017
#define OBJECT_TYPE_ER_STATE 0x00000018

#define OBJECT_TYPE_GBITMAP 0x00000019
#define OBJECT_TYPE_GBITMAP_TREE 0x0000001a
#define OBJECT_TYPE_GBITMAP_BLOCK 0x0000001b

#define OBJECT_TYPE_ER_RECOVERY_BLOCK 0x0000001c
#define OBJECT_TYPE_SNAP_META_EXT 0x0000001d
#define OBJECT_TYPE_INTEGRITY_META 0x0000001e
#define OBJECT_TYPE_FEXT_TREE 0x0000001f
#define OBJECT_TYPE_RESERVED_20 0x00000020

#define OBJECT_TYPE_INVALID 0x00000000
#define OBJECT_TYPE_TEST 0x000000ff

#define OBJECT_TYPE_CONTAINER_KEYBAG 'keys'
#define OBJECT_TYPE_VOLUME_KEYBAG 'recs'
#define OBJECT_TYPE_MEDIA_KEYBAG 'mkey'


#define OBJ_VIRTUAL 0x00000000
#define OBJ_EPHEMERAL 0x80000000
#define OBJ_PHYSICAL 0x40000000

#define OBJ_NOHEADER 0x20000000
#define OBJ_ENCRYPTED 0x10000000
#define OBJ_NONPERSISTENT 0x08000000

struct nx_efi_jumpstart {
	obj_phys_t nej_o;
	uint32_t nej_magic;
	uint32_t nej_version;
	uint32_t nej_efi_file_len;
	uint32_t nej_num_extents;
	uint64_t nej_reserved[16];
	prange_t nej_rec_extents[];
};
typedef struct nx_efi_jumpstart nx_efi_jumpstart_t;
#define NX_EFI_JUMPSTART_MAGIC 'RDSJ'
#define NX_EFI_JUMPSTART_VERSION 1

#define APFS_GPT_PARTITION_UUID "7C3457EF-0000-11AA-AA11-00306543ECAC"


/**
 * Container
 */

#define NX_MAGIC 'BSXN'
#define NX_MAX_FILE_SYSTEMS 100

#define NX_EPH_INFO_COUNT 4
#define NX_EPH_MIN_BLOCK_COUNT 8
#define NX_MAX_FILE_SYSTEM_EPH_STRUCTS 4
#define NX_TX_MIN_CHECKPOINT_COUNT 4
#define NX_EPH_INFO_VERSION_1 1

typedef enum {
	NX_CNTR_OBJ_CKSUM_SET = 0,
	NX_CNTR_OBJ_CKSUM_FAIL = 1,

	NX_NUM_COUNTERS = 32
} nx_counter_id_t;

struct nx_superblock {
	obj_phys_t nx_o;
	uint32_t nx_magic;
	uint32_t nx_block_size;
	uint64_t nx_block_count;

	uint64_t nx_features;
	uint64_t nx_readonly_compatible_features;
	uint64_t nx_incompatible_features;

	uuid_t nx_uuid;

	oid_t nx_next_oid;
	xid_t nx_next_xid;

	uint32_t nx_xp_desc_blocks;
	uint32_t nx_xp_data_blocks;
	paddr_t nx_xp_desc_base;
	paddr_t nx_xp_data_base;
	uint32_t nx_xp_desc_next;
	uint32_t nx_xp_data_next;
	uint32_t nx_xp_desc_index;
	uint32_t nx_xp_desc_len;
	uint32_t nx_xp_data_index;
	uint32_t nx_xp_data_len;

	oid_t nx_spaceman_oid;
	oid_t nx_omap_oid;
	oid_t nx_reaper_oid;

	uint32_t nx_test_type;

	uint32_t nx_max_file_systems;
	oid_t nx_fs_oid[NX_MAX_FILE_SYSTEMS];
	uint64_t nx_counters[NX_NUM_COUNTERS];
	prange_t nx_blocked_out_prange;
	oid_t nx_evict_mapping_tree_oid;
	uint64_t nx_flags;
	paddr_t nx_efi_jumpstart;
	uuid_t nx_fusion_uuid;
	prange_t nx_keylocker;
	uint64_t nx_ephemeral_info[NX_EPH_INFO_COUNT];

	oid_t nx_test_oid;

	oid_t nx_fusion_mt_oid;
	oid_t nx_fusion_wbc_oid;
	prange_t nx_fusion_wbc;

	uint64_t nx_newest_mounted_version;

	prange_t nx_mkb_locker;
};
typedef struct nx_superblock nx_superblock_t;

#define NX_RESERVED_1 0x00000001LL
#define NX_RESERVED_2 0x00000002LL
#define NX_CRYPTO_SW 0x00000004LL

#define NX_FEATURE_DEFRAG 0x0000000000000001ULL
#define NX_FEATURE_LCFD 0x0000000000000002ULL
#define NX_SUPPORTED_FEATURES_MASK (NX_FEATURE_DEFRAG | NX_FEATURE_LCFD)

#define NX_SUPPORTED_ROCOMPAT_MASK (0x0ULL)

#define NX_INCOMPAT_VERSION1 0x0000000000000001ULL
#define NX_INCOMPAT_VERSION2 0x0000000000000002ULL
#define NX_INCOMPAT_FUSION 0x0000000000000100ULL
#define NX_SUPPORTED_INCOMPAT_MASK (NX_INCOMPAT_VERSION2 | NX_INCOMPAT_FUSION)

#define NX_MINIMUM_BLOCK_SIZE 4096
#define NX_DEFAULT_BLOCK_SIZE 4096
#define NX_MAXIMUM_BLOCK_SIZE 65536

#define NX_MINIMUM_CONTAINER_SIZE 1048576

struct checkpoint_mapping {
	uint32_t cpm_type;
	uint32_t cpm_subtype;
	uint32_t cpm_size;
	uint32_t cpm_pad;
	oid_t cpm_fs_oid;
	oid_t cpm_oid;
	oid_t cpm_paddr;
};
typedef struct checkpoint_mapping checkpoint_mapping_t;

struct checkpoint_map_phys {
	obj_phys_t cpm_o;
	uint32_t cpm_flags;
	uint32_t cpm_count;
	checkpoint_mapping_t cpm_map[];
};

#define CHECKPOINT_MAP_LAST 0x00000001

struct evict_mapping_val {
	paddr_t dst_paddr;
	uint64_t len;
} __attribute__((packed));
typedef struct evict_mapping_val evict_mapping_val_t;


/**
 * Object Maps
 */

struct omap_phys {
	obj_phys_t om_o;
	uint32_t om_flags;
	uint32_t om_snap_count;
	uint32_t om_tree_type;
	uint32_t om_snapshot_tree_type;
	oid_t om_tree_oid;
	oid_t om_snapshot_tree_oid;
	xid_t om_most_recent_snap;
	xid_t om_pending_revert_min;
	xid_t om_pending_revert_max;
};
typedef struct omap_phys omap_phys_t;

struct omap_key {
	oid_t ok_oid;
	xid_t ok_xid;
};
typedef struct omap_key omap_key_t;

struct omap_val {
	uint32_t ov_flags;
	uint32_t ov_size;
	paddr_t ov_paddr;
};
typedef struct omap_val omap_val_t;

struct omap_snapshot {
	uint32_t oms_flags;
	uint32_t oms_pad;
	oid_t oms_oid;
};
typedef struct omap_snapshot omap_snapshot_t;

#define OMAP_VAL_DELETED 0x00000001
#define OMAP_VAL_SAVED 0x00000002
#define OMAP_VAL_ENCRYPTED 0x00000004
#define OMAP_VAL_NOHEADER 0x00000008
#define OMAP_VAL_CRYPTO_GENERATION 0x00000010

#define OMAP_SNAPSHOT_DELETED 0x00000001
#define OMAP_SNAPSHOT_REVERTED 0x00000002

#define OMAP_MANUALLY_MANAGED 0x00000001
#define OMAP_ENCRYPTING 0x00000002
#define OMAP_DECRYPTING 0x00000004
#define OMAP_KEYROLLING 0x00000008
#define OMAP_CRYPTO_GENERATION 0x00000010

#define OMAP_VALID_FLAGS 0x0000001f

#define OMAP_MAX_SNAP_COUNT UINT32_MAX

#define OMAP_REAP_PHASE_MAP_TREE 1
#define OMAP_REAP_PHASE_SNAPSHOT_TREE 2


/**
 * Encryption
 * 
 * Early definitions needed only
 */

typedef uint32_t cp_key_class_t;
typedef uint32_t cp_key_os_version_t;
typedef uint16_t cp_key_revision_t;
typedef uint32_t crypto_flags_t;

struct wrapped_meta_crypto_state {
	uint16_t major_version;
	uint16_t minor_version;
	crypto_flags_t cpflags;
	cp_key_class_t persistent_class;
	cp_key_os_version_t key_os_version;
	cp_key_revision_t key_revision;
	uint16_t unused;
} __attribute__((aligned(2), packed));
typedef struct wrapped_meta_crypto_state wrapped_meta_crypto_state_t;


/**
 * Volumes
 */

#define APFS_MODIFIED_NAMELEN 32

struct apfs_modified_by {
	uint8_t id[APFS_MODIFIED_NAMELEN];
	uint64_t timestamp;
	xid_t last_xid;
};
typedef struct apfs_modified_by apfs_modified_by_t;

#define APFS_MAGIC 'BSPA'
#define APFS_MAX_HIST 8
#define APFS_VOLNAME_LEN 256

struct apfs_superblock {
	obj_phys_t apfs_o;

	uint32_t apfs_magic;
	uint32_t apfs_fs_index;

	uint64_t apfs_features;
	uint64_t apfs_readonly_compatible_features;
	uint64_t apfs_incompatible_features;

	uint64_t apfs_unmount_time;

	uint64_t apfs_fs_reserve_block_count;
	uint64_t apfs_fs_quota_block_count;
	uint64_t apfs_fs_alloc_count;

	wrapped_meta_crypto_state_t apfs_meta_crypto;

	uint32_t apfs_root_tree_type;
	uint32_t apfs_extentref_tree_type;
	uint32_t apfs_snap_meta_tree_type;

	oid_t apfs_omap_oid;
	oid_t apfs_root_tree_oid;
	oid_t apfs_extentref_tree_oid;
	oid_t apfs_snap_meta_tree_oid;

	xid_t apfs_revert_to_xid;
	oid_t apfs_revert_to_sblock_oid;

	uint64_t apfs_next_obj_id;
	uint64_t apfs_num_files;
	uint64_t apfs_num_directories;
	uint64_t apfs_num_symlinks;
	uint64_t apfs_num_other_fsobjects;
	uint64_t apfs_num_snapshots;

	uint64_t apfs_total_blocks_alloced;
	uint64_t apfs_total_blocks_freed;

	uuid_t apfs_vol_uuid;
	uint64_t apfs_last_mod_time;

	uint64_t apfs_fs_flags;

	apfs_modified_by_t apfs_formatted_by;
	apfs_modified_by_t apfs_modified_by[APFS_MAX_HIST];

	uint8_t apfs_volname[APFS_VOLNAME_LEN];
	uint32_t apfs_next_doc_id;

	uint16_t apfs_role;
	uint16_t reserved;

	xid_t apfs_root_to_xid;
	oid_t apfs_er_state_oid;

	uint64_t apfs_cloneinfo_id_epoch;
	uint64_t apfs_cloneinfo_xid;

	oid_t apfs_snap_meta_ext_oid;

	uuid_t apfs_volume_group_id;

	oid_t apfs_integrity_meta_oid;

	oid_t apfs_fext_tree_oid;
	uint32_t apfs_fext_tree_type;

	uint32_t reserved_type;
	oid_t reserved_oid;
};

#define APFS_FS_UNENCRYPTED 0x00000001LL
#define APFS_FS_RESERVED_2 0x00000002LL
#define APFS_FS_RESERVED_4 0x00000004LL
#define APFS_FS_ONEKEY 0x00000008LL
#define APFS_FS_SPILLEDOVER 0x00000010LL
#define APFS_FS_RUN_SPILLOVER_CLEANER 0x00000020LL
#define APFS_FS_ALWAYS_CHECK_EXTENTREF 0x00000040LL
#define APFS_FS_RESERVED_80 0x00000080LL
#define APFS_FS_RESERVED_100 0x00000100LL
#define APFS_FS_FLAGS_VALID_MASK (APFS_FS_UNENCRYPTED \
				| APFS_FS_RESERVED_2 \
				| APFS_FS_RESERVED_4 \
				| APFS_FS_ONEKEY \
				| APFS_FS_SPILLEDOVER \
				| APFS_FS_RUN_SPILLOVER_CLEANER \
				| APFS_FS_ALWAYS_CHECK_EXTENTREF \
				| APFS_FS_RESERVED_80 \
				| APFS_FS_RESERVED_100)
#define APFS_FS_CRYPTOFLAGS (APFS_FS_UNENCRYPTED \
				| APFS_FS_ONEKEY)

#define APFS_VOL_ROLE_NONE 0x0000

#define APFS_VOL_ROLE_SYSTEM 0x0001
#define APFS_VOL_ROLE_USER 0x0002
#define APFS_VOL_ROLE_RECOVERY 0x0004
#define APFS_VOL_ROLE_VM 0x0008
#define APFS_VOL_ROLE_PREBOOT 0x0010
#define APFS_VOL_ROLE_INSTALLER 0x0020

#define APFS_VOL_ROLE_DATA (1 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_BASEBAND (2 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_UPDATE (3 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_XART (4 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_HARDWARE (5 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_BACKUP (6 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_RESERVED_7 (7 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_RESERVED_8 (8 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_ENTERPRISE (9 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_RESERVED_10 (10 << APFS_VOLUME_ENUM_SHIFT)
#define APFS_VOL_ROLE_PRELOGIN (11 << APFS_VOLUME_ENUM_SHIFT)

#define APFS_VOLUME_ENUM_SHIFT 6

#define APFS_FEATURE_DEFRAG_PRERELEASE 0x00000001LL
#define APFS_FEATURE_HARDLINK_MAP_RECORDS 0x00000002LL
#define APFS_FEATURE_DEFRAG 0x00000004LL
#define APFS_FEATURE_STRICTATIME 0x00000008LL
#define APFS_FEATURE_VOLGRP_SYSTEM_INO_SPACE 0x00000010LL

#define APFS_SUPPORTED_FEATURES_MASK (APFS_FEATURE_DEFRAG \
				| APFS_FEATURE_DEFRAG_PRERELEASE \
				| APFS_FEATURE_HARDLINK_MAP_RECORDS \
				| APFS_FEATURE_STRICTATIME \
				| APFS_FEATURE_VOLGRP_SYSTEM_INO_SPACE)

#define APFS_SUPPORTED_ROCOMPAT_MASK (0x0ULL)

#define APFS_INCOMPAT_CASE_INSENSITIVE 0x00000001LL
#define APFS_INCOMPAT_DATALESS_SNAPS 0x00000002LL
#define APFS_INCOMPAT_ENC_ROLLED 0x00000004LL
#define APFS_INCOMPAT_NORMALIZATION_INSENSITIVE 0x00000008LL
#define APFS_INCOMPAT_INCOMPLETE_RESTORE 0x00000010LL
#define APFS_INCOMPAT_SEALED_VOLUME 0x00000020LL
#define APFS_INCOMPAT_RESERVED_40 0x00000040LL

#define APFS_SUPPORTED_INCOMPAT_MASK (APFS_INCOMPAT_CASE_INSENSITIVE \
				| APFS_INCOMPAT_DATALESS_SNAPS \
				| APFS_INCOMPAT_ENC_ROLLED \
				| APFS_INCOMPAT_NORMALIZATION_INSENSITIVE \
				| APFS_INCOMPAT_INCOMPLETE_RESTORE \
				| APFS_INCOMPAT_SEALED_VOLUME \
				| APFS_INCOMPAT_RESERVED_40)

/**
 * File-System Objects
 */

struct j_key {
	uint64_t obj_id_and_type;
} __attribute__((packed));
typedef struct j_key j_key_t;

#define OBJ_ID_MASK 0x0fffffffffffffffULL
#define OBJ_TYPE_MASK 0xf000000000000000ULL
#define OBJ_TYPE_SHIFT 60

#define SYSTEM_OBJ_ID_MARK 0x0fffffff00000000ULL

struct j_inode_key {
	j_key_t hdr;
} __attribute__((packed));
typedef struct j_inode_key_t j_inode_key_t;

typedef uint32_t uid_t;
typedef uint32_t gid_t;

typedef uint16_t mode_t;

#define S_IFMT 0170000

#define S_IFIFO 0010000
#define S_IFCHR 0020000
#define S_IFDIR 0040000
#define S_IFBLK 0060000
#define S_IFREG 0100000
#define S_IFLNK 0120000
#define S_IFSOCK 0140000
#define S_IFWHT 0160000

#define DT_UNKNOWN 0
#define DT_FIFO 1
#define DT_CHR 2
#define DT_DIR 4
#define DT_BLK 6
#define DT_REG 8
#define DT_LNK 10
#define DT_SOCK 12
#define DT_WHT 14

struct j_inode_val {
	uint64_t parent_id;
	uint64_t private_id;

	uint64_t create_time;
	uint64_t mod_time;
	uint64_t change_time;
	uint64_t access_time;

	uint64_t internal_flags;

	union {
		int32_t nchildren;
		int32_t nlink;
	};

	cp_key_class_t default_protection_class;
	uint32_t write_generation_counter;
	uint32_t bsd_flags;
	uid_t owner;
	gid_t group;
	mode_t mode;
	uint16_t pad1;
	uint64_t uncompressed_size;
	uint8_t xfields[];
} __attribute__((packed));
typedef struct j_inode_val j_inode_val_t;

struct j_drec_key {
	j_key_t hdr;
	uint16_t name_len;
	uint8_t name[0];
} __attribute__((packed));
typedef struct j_drec_key j_drec_key_t;

struct j_drec_hashed_key {
	j_key_t hdr;
	uint32_t name_len_and_hash;
	uint8_t name[0];
} __attribute__((packed));
typedef struct j_drec_hashed_key j_drec_hashed_key_t;

#define J_DREC_LEN_MASK 0x000003ff
#define J_DREC_HASH_MASK 0xfffff400
#define J_DREC_HASH_SHIFT 10

struct j_drec_val {
	uint64_t file_id;
	uint64_t date_added;
	uint16_t flags;
	uint8_t xfields[];
} __attribute__((packed));
typedef struct j_drec_val j_drec_val_t;

struct j_dir_stats_key {
	j_key_t hdr;
} __attribute__((packed));
typedef struct j_dir_stats_key j_dir_stats_key_t;

struct j_dir_stats_val {
	uint64_t num_children;
	uint64_t total_size;
	uint64_t chained_key;
	uint64_t gen_count;
} __attribute__((packed));
typedef struct j_dir_stats_val j_dir_stats_val_t;

struct j_xattr_key {
	j_key_t hdr;
	uint16_t name_len;
	uint8_t name[0];
} __attribute__((packed));
typedef struct j_xattr_key j_xattr_key_t;

struct j_xattr_val {
	uint16_t flags;
	uint16_t xdata_len;
	uint8_t xdata[0];
} __attribute__((packed));
typedef struct j_xattr_val j_xattr_val_t;


/**
 * File-System Constants
 */

typedef enum {
	APFS_TYPE_ANY = 0,

	APFS_TYPE_SNAP_METADATA = 1,
	APFS_TYPE_EXTENT = 2,
	APFS_TYPE_INODE = 3,
	APFS_TYPE_XATTR = 4,
	APFS_TYPE_SIBLING_LINK = 5,
	APFS_TYPE_DSTREAM_ID = 6,
	APFS_TYPE_CRYPTO_STATE = 7,
	APFS_TYPE_FILE_EXTENT = 8,
	APFS_TYPE_DIR_REC = 9,
	APFS_TYPE_DIR_STATS = 10,
	APFS_TYPE_SNAP_NAME = 11,
	APFS_TYPE_SIBLING_MAP = 12,
	APFS_TYPE_FILE_INFO = 13,

	APFS_TYPE_MAX_VALID = 13,
	APFS_TYPE_MAX = 15,

	APFS_TYPE_INVALID = 15,
} j_obj_types;

typedef enum {
	APFS_KIND_ANY = 0,
	APFS_KIND_NEW = 1,
	APFS_KIND_UPDATE = 2,
	APFS_KIND_DEAD = 3,
	APFS_KIND_UPDATE_REFCNT = 4,

	APFS_KIND_INVALID = 255
} j_obj_kinds;

typedef enum {
	INODE_IS_APFS_PRIVATE = 0x00000001,
	INODE_MAINTAIN_DIR_STATS = 0x00000002,
	INODE_DIR_STATS_ORIGIN = 0x00000004,
	INODE_PROT_CLASS_EXPLICIT = 0x00000008,
	INODE_WAS_CLONED = 0x00000010,
	INODE_FLAG_UNUSED = 0x00000020,
	INODE_HAS_SECURITY_EA = 0x00000040,
	INODE_BEING_TRUNCATED = 0x00000080,
	INODE_HAS_FINDER_INFO = 0x00000100,
	INODE_IS_SPARSE = 0x00000200,
	INODE_WAS_EVER_CLONED = 0x00000400,
	INODE_ACTIVE_FILE_TRIMMED = 0x00000800,
	INODE_PINNED_TO_MAIN = 0x00001000,
	INODE_PINNED_TO_TIER2 = 0x00002000,
	INODE_HAS_RSRC_FORK = 0x00004000,
	INODE_NO_RSRC_FORK = 0x00008000,
	INODE_ALLOCATION_SPILLEDOVER = 0x00010000,
	INODE_FAST_PROMOTE = 0x00020000,
	INODE_HAS_UNCOMPRESSED_SIZE = 0x00040000,
	INODE_IS_PURGEABLE = 0x00080000,
	INODE_WANTS_TO_BE_PURGEABLE = 0x00100000,
	INODE_IS_SYNC_ROOT = 0x00200000,
	INODE_SNAPSHOT_COW_EXEMPTION = 0x00400000,


	INODE_INHERITED_INTERNAL_FLAGS = (INODE_MAINTAIN_DIR_STATS \
				| INODE_SNAPSHOT_COW_EXEMPTION),

	INODE_CLONED_INTERNAL_FLAGS = (INODE_HAS_RSRC_FORK \
				| INODE_NO_RSRC_FORK \
				| INODE_HAS_FINDER_INFO \
				| INODE_SNAPSHOT_COW_EXEMPTION),
} j_inode_flags;

#define APFS_VALID_INTERNAL_INODE_FLAGS (INODE_IS_APFS_PRIVATE \
				| INODE_MAINTAIN_DIR_STATS \
				| INODE_DIR_STATS_ORIGIN \
				| INODE_PROT_CLASS_EXPLICIT \
				| INODE_WAS_CLONED \
				| INODE_HAS_SECURITY_EA \
				| INODE_BEING_TRUNCATED \
				| INODE_HAS_FINDER_INFO \
				| INODE_IS_SPARSE \
				| INODE_WAS_EVER_CLONED \
				| INODE_ACTIVE_FILE_TRIMMED \
				| INODE_PINNED_TO_MAIN \
				| INODE_PINNED_TO_TIER2 \
				| INODE_HAS_RSRC_FORK \
				| INODE_NO_RSRC_FORK \
				| INODE_ALLOCATION_SPILLEDOVER \
				| INODE_FAST_PROMOTE \
				| INODE_HAS_UNCOMPRESSED_SIZE \
				| INODE_IS_PURGEABLE \
				| INODE_WANTS_TO_BE_PURGEABLE \
				| INODE_IS_SYNC_ROOT \
				| INODE_SNAPSHOT_COW_EXEMPTION)

#define APFS_INODE_PINNED_MASK (INODE_PINNED_TO_MAIN | INODE_PINNED_TO_TIER2)

typedef enum {
	XATTR_DATA_STREAM = 0x00000001,
	XATTR_DATA_EMBEDDED = 0x00000002,
	XATTR_FILE_SYSTEM_OWNED = 0x00000004,
	XATTR_RESERVED_8 = 0x00000008,
} j_xattr_flags;

typedef enum {
	DREC_TYPE_MASK = 0x000f,
	RESERVED_10 = 0x0010
} dir_rec_flags;

#define INVALID_INO_NUM 0

#define ROOT_DIR_PARENT 1
#define ROOT_DIR_INO_NUM 2
#define PRIV_DIR_INO_NUM 3
#define SNAP_DIR_INO_NUM 6
#define PURGEABLE_DIR_INO_NUM 7

#define MIN_USER_INO_NUM 16

#define UNIFIED_ID_SPACE_MARK 0x0800000000000000ULL

#define XATTR_MAX_EMBEDDED_SIZE 3804
#define SYMLINK_EA_NAME ”com.apple.fs.symlink”
#define FIRMLINK_EA_NAME ”com.apple.fs.firmlink”
#define APFS_COW_EXEMPT_COUNT_NAME ”com.apple.fs.cow-exempt-file-count”

#define OWNING_OBJ_ID_INVALID ~0ULL
#define OWNING_OBJ_ID_UNKNOWN ~1ULL

#define JOBJ_MAX_KEY_SIZE 832
#define JOBJ_MAX_VALUE_SIZE 3808

#define MIN_DOC_ID 3

#define FEXT_CRYPTO_ID_IS_TWEAK 0x01

/**
 * Data Streams
 */

struct j_phys_ext_key {
	j_key_t hdr;
} __attribute__((packed));
typedef struct j_phys_ext_key j_phys_ext_key_t;

struct j_phys_ext_val {
	uint64_t len_and_kind;
	uint64_t owning_obj_id;
	int32_t refcnt;
} __attribute__((packed));
typedef struct j_phys_ext_val j_phys_ext_val_t;

#define PEXT_LEN_MASK 0x0fffffffffffffffULL
#define PEXT_KIND_MASK 0xf000000000000000ULL
#define PEXT_KIND_SHIFT 60

struct j_file_extent_key {
	j_key_t hdr;
	uint64_t logical_addr;
} __attribute__((packed));
typedef struct j_file_extent_key j_file_extent_key_t;

struct j_file_extent_val {
	uint64_t len_and_flags;
	uint64_t phys_block_num;
	uint64_t crypto_id;
} __attribute__((packed));
typedef struct j_file_extent_val j_file_extent_val_t;

#define J_FILE_EXTENT_LEN_MASK 0x00ffffffffffffffULL
#define J_FILE_EXTENT_FLAG_MASK 0xff00000000000000ULL
#define J_FILE_EXTENT_FLAG_SHIFT 56

struct j_dstream_id_key {
	j_key_t hdr;
} __attribute__((packed));
typedef struct j_dstream_id_key j_dstream_id_key_t;

struct j_dstream_id_val {
	uint32_t refcnt;
} __attribute__((packed));
typedef struct j_dstream_id_val j_dstream_id_val_t;

struct j_dstream {
	uint64_t size;
	uint64_t alloced_size;
	uint64_t default_crypto_id;
	uint64_t total_bytes_written;
	uint64_t total_bytes_read;
} __attribute__((aligned(8),packed));
typedef struct j_dstream j_dstream_t;

struct j_xattr_dstream {
	uint64_t xattr_obj_id;
	j_dstream_t dstream;
};
typedef struct j_xattr_dstream j_xattr_dstream_t;


/**
 * Extended Fields
 */

struct xf_blob {
	uint16_t xf_num_exts;
	uint16_t xf_used_data;
	uint8_t xf_data[];
};
typedef struct xf_blob xf_blob_t;

struct x_field {
	uint8_t x_type;
	uint8_t x_flags;
	uint16_t x_size;
};
typedef struct x_field x_field_t;

#define DREC_EXT_TYPE_SIBLING_ID 1

#define INO_EXT_TYPE_SNAP_XID 1
#define INO_EXT_TYPE_DELTA_TREE_OID 2
#define INO_EXT_TYPE_DOCUMENT_ID 3
#define INO_EXT_TYPE_NAME 4
#define INO_EXT_TYPE_PREV_FSIZE 5
#define INO_EXT_TYPE_RESERVED_6 6
#define INO_EXT_TYPE_FINDER_INFO 7
#define INO_EXT_TYPE_DSTREAM 8
#define INO_EXT_TYPE_RESERVED_9 9
#define INO_EXT_TYPE_DIR_STATS_KEY 10
#define INO_EXT_TYPE_FS_UUID 11
#define INO_EXT_TYPE_RESERVED_12 12
#define INO_EXT_TYPE_SPARSE_BYTES 13
#define INO_EXT_TYPE_RDEV 14
#define INO_EXT_TYPE_PURGEABLE_FLAGS 15
#define INO_EXT_TYPE_ORIG_SYNC_ROOT_ID 16

#define XF_DATA_DEPENDENT 0x0001
#define XF_DO_NOT_COPY 0x0002
#define XF_RESERVED_4 0x0004
#define XF_CHILDREN_INHERIT 0x0008
#define XF_USER_FIELD 0x0010
#define XF_SYSTEM_FIELD 0x0020
#define XF_RESERVED_40 0x0040
#define XF_RESERVED_80 0x0080


/**
 * Siblings
 */

struct j_sibling_key {
	j_key_t hdr;
	uint64_t sibling_id;
} __attribute__((packed));
typedef struct j_sibling_key j_sibling_key_t;

struct j_sibling_val {
	uint64_t parent_id;
	uint16_t name_len;
	uint8_t name[0];
} __attribute__((packed));
typedef struct j_sibling_val j_sibling_val_t;

struct j_sibling_map_key {
	j_key_t hdr;
} __attribute__((packed));
typedef struct j_sibling_map_key j_sibling_map_key_t;

struct j_sibling_map_val {
	uint64_t file_id;
} __attribute__((packed));
typedef struct j_sibling_map_val j_sibling_map_val_t;


/**
 * Snapshot Metadata
 */

struct j_snap_metadata_key {
	j_key_t hdr;
} __attribute__((packed));
typedef struct j_snap_metadata_key j_snap_metadata_key_t;

struct j_snap_metadata_val {
	oid_t extentref_tree_oid;
	oid_t sblock_oid;
	uint64_t create_time;
	uint64_t change_time;
	uint64_t inum;
	uint32_t extentref_tree_type;
	uint32_t flags;
	uint16_t name_len;
	uint8_t name[0];
} __attribute__((packed));
typedef struct j_snap_metadata_val j_snap_metadata_val_t;

struct j_snap_name_key {
	j_key_t hdr;
	uint16_t name_len;
	uint8_t name[0];
} __attribute__((packed));
typedef struct j_snap_name_key j_snap_name_key_t;

struct j_snap_name_val {
	xid_t snap_xid;
} __attribute__((packed));
typedef struct j_snap_name_val j_snap_name_val_t;

typedef enum {
	SNAP_META_PENDING_DATALESS = 0x00000001,
	SNAP_META_MERGE_IN_PROGRESS = 0x00000002,
} snap_meta_flags;

struct snap_meta_ext {
	uint32_t sme_version;

	uint32_t sme_flags;
	xid_t sme_snap_xid;
	uuid_t sme_uuid;

	uint64_t sme_token;
} __attribute__((packed));
typedef struct snap_meta_ext snap_meta_ext_t;

struct snap_meta_ext_obj_phys {
	obj_phys_t smeop_o;
	snap_meta_ext_t smeop_sme;
};
typedef struct snap_meta_ext_obj_phys snap_meta_ext_obj_phys_t;


/**
 * B-Trees
 */

struct nloc {
	uint16_t off;
	uint16_t len;
};
typedef struct nloc nloc_t;

struct btree_node_phys {
	obj_phys_t btn_o;
	uint16_t btn_flags;
	uint16_t btn_level;
	uint32_t btn_nkeys;
	nloc_t btn_table_space;
	nloc_t btn_free_space;
	nloc_t btn_key_free_list;
	nloc_t btn_val_free_list;
	uint64_t btn_data[];
};
typedef struct btree_node_phys btree_node_phys_t;

struct btree_info_fixed {
	uint32_t bt_flags;
	uint32_t bt_node_size;
	uint32_t bt_key_size;
	uint32_t bt_val_size;
};
typedef struct btree_info_fixed btree_info_fixed_t;

struct btree_info {
	btree_info_fixed_t bt_fixed;
	uint32_t bt_longest_key;
	uint32_t bt_longest_val;
	uint64_t bt_key_count;
	uint64_t bt_node_count;
};
typedef struct btree_info btree_info_t;

#define BTREE_NODE_HASH_SIZE_MAX 64

struct btn_index_node_val {
	oid_t binv_child_oid;
	uint8_t binv_child_hash[BTREE_NODE_HASH_SIZE_MAX];
};
typedef struct btn_index_node_val btn_index_node_val_t;

#define BTOFF_INVALID 0xffff

struct kvloc {
	nloc_t k;
	nloc_t v;
};
typedef struct kvloc kvloc_t;

struct kvoff {
	uint16_t k;
	uint16_t v;
};
typedef struct kvoff kvoff_t;

#define BTREE_UINT64_KEYS 0x00000001
#define BTREE_SEQUENTIAL_INSERT 0x00000002
#define BTREE_ALLOW_GHOSTS 0x00000004
#define BTREE_EPHEMERAL 0x00000008
#define BTREE_PHYSICAL 0x00000010
#define BTREE_NONPERSISTENT 0x00000020
#define BTREE_KV_NONALIGNED 0x00000040
#define BTREE_HASHED 0x00000080
#define BTREE_NOHEADER 0x00000100

#define BTREE_TOC_ENTRY_INCREMENT 8
#define BTREE_TOC_ENTRY_MAX_UNUSED (2 * BTREE_TOC_ENTRY_INCREMENT)

#define BTNODE_ROOT 0x0001
#define BTNODE_LEAF 0x0002

#define BTNODE_FIXED_KV_SIZE 0x0004
#define BTNODE_HASHED 0x0008
#define BTNODE_NOHEADER 0x0010

#define BTNODE_CHECK_KOFF_INVAL 0x8000

#define BTREE_NODE_SIZE_DEFAULT 4096
#define BTREE_NODE_MIN_ENTRY_COUNT 4


/**
 * Encryption
 */

struct j_crypto_key {
	j_key_t hdr;
} __attribute__((packed));
typedef struct j_crypto_key j_crypto_key_t;

struct wrapped_crypto_state {
	uint16_t major_version;
	uint16_t minor_version;
	crypto_flags_t cpflags;
	cp_key_class_t persistent_class;
	cp_key_os_version_t key_os_version;
	cp_key_revision_t key_revision;
	uint16_t key_len;
	uint8_t persistent_key[0];
} __attribute__((aligned(2), packed));
typedef struct wrapped_crypto_state wrapped_crypto_state_t;

struct j_crypto_val {
	uint32_t refcnt;
	wrapped_crypto_state_t state;
} __attribute__((aligned(4),packed));
typedef struct j_crypto_val j_crypto_val_t;

#define CP_MAX_WRAPPEDKEYSIZE 128

#define PROTECTION_CLASS_DIR_NONE 0
#define PROTECTION_CLASS_A 1
#define PROTECTION_CLASS_B 2
#define PROTECTION_CLASS_C 3
#define PROTECTION_CLASS_D 4
#define PROTECTION_CLASS_F 6
#define PROTECTION_CLASS_M 14

#define CP_EFFECTIVE_CLASSMASK 0x0000001f

#define CRYPTO_SW_ID 4
#define CRYPTO_RESERVED_5 5

#define APFS_UNASSIGNED_CRYPTO_ID (~0ULL)

struct keybag_entry {
	uuid_t ke_uuid;
	uint16_t ke_tag;
	uint16_t ke_keylen;
	uint8_t padding[4];
	uint8_t ke_keydata[];
};
typedef struct keybag_entry keybag_entry_t;

struct kb_locker {
	uint16_t kl_version;
	uint16_t kl_nkeys;
	uint32_t kl_nbytes;
	uint8_t padding[8];
	keybag_entry_t kl_entries[];
};
typedef struct kb_locker kb_locker_t;

#define APFS_KEYBAG_VERSION 2

#define APFS_VOL_KEYBAG_ENTRY_MAX_SIZE 512
#define APFS_FV_PERSONAL_RECOVERY_KEY_UUID "EBC6C064-0000-11AA-AA11-00306543ECAC"

struct media_keybag {
	obj_phys_t mk_obj;
	kb_locker_t mk_locker;
};
typedef struct media_keybag media_keybag_t;

enum {
	KB_TAG_UNKNOWN = 0,
	KB_TAG_RESERVED_1 = 1,
	KB_TAG_VOLUME_KEY = 2,
	KB_TAG_VOLUME_UNLOCK_RECORDS = 3,
	KB_TAG_VOLUME_PASSPHRASE_HINT = 4,
	KB_TAG_WRAPPING_M_KEY = 5,
	KB_TAG_VOLUME_M_KEY = 6,
	KB_TAG_RESERVED_F8 = 0xF8
};


/**
 * Sealed Volumes
 */

typedef enum {
	APFS_HASH_INVALID = 0,
	APFS_HASH_SHA256 = 0x1,
	APFS_HASH_SHA512_256 = 0x2,
	APFS_HASH_SHA384 = 0x3,
	APFS_HASH_SHA512 = 0x4,

	APFS_HASH_MIN = APFS_HASH_SHA256,
	APFS_HASH_MAX = APFS_HASH_SHA512,

	APFS_HASH_DEFAULT = APFS_HASH_SHA256,
} apfs_hash_type_t;

struct integrity_meta_phys {
	obj_phys_t im_o;
	uint32_t im_version;
	uint32_t im_flags;
	apfs_hash_type_t im_hash_type;
	uint32_t im_root_hash_offset;
	xid_t im_broken_xid;
	uint64_t im_reserved[9];
} __attribute__((packed));
typedef struct integrity_meta_phys integrity_meta_phys_t;

enum {
	INTEGRITY_META_VERSION_INVALID = 0,
	INTEGRITY_META_VERSION_1 = 1,
	INTEGRITY_META_VERSION_2 = 2,
	INTEGRITY_META_VERSION_HIGHEST = INTEGRITY_META_VERSION_2
};

#define APFS_SEAL_BROKEN (1U << 0)

#define APFS_HASH_CCSHA256_SIZE 32
#define APFS_HASH_CCSHA512_256_SIZE 32
#define APFS_HASH_CCSHA384_SIZE 48
#define APFS_HASH_CCSHA512_SIZE 64

#define APFS_HASH_MAX_SIZE 64

struct fext_tree_key {
	uint64_t private_id;
	uint64_t logical_addr;
} __attribute__((packed));
typedef struct fext_tree_key fext_tree_key_t;

struct fext_tree_val {
	uint64_t len_and_flags;
	uint64_t phys_block_num;
} __attribute__((packed));
typedef struct fext_tree_val fext_tree_val_t;

struct j_file_info_key {
	j_key_t hdr;
	uint64_t info_and_lba;
} __attribute__((packed));
typedef struct j_key_t j_file_info_key_t;

#define J_FILE_INFO_LBA_MASK 0x00ffffffffffffffULL
#define J_FILE_INFO_TYPE_MASK 0xff00000000000000ULL
#define J_FILE_INFO_TYPE_SHIFT 56

struct j_file_data_hash_val {
	uint16_t hashed_len;
	uint8_t hash_size;
	uint8_t hash[0];
} __attribute__((packed));
typedef struct j_file_data_hash_val j_file_data_hash_val_t;

struct j_file_info_val {
	union {
		j_file_data_hash_val_t dhash;
	};
} __attribute__((packed));
typedef struct j_file_data_hash_val_t j_file_info_val_t;

typedef enum {
	APFS_FILE_INFO_DATA_HASH = 1,
} j_obj_file_info_type;


/**
 * Space Manager
 */

struct chunk_info {
	uint64_t ci_xid;
	uint64_t ci_addr;
	uint32_t ci_block_count;
	uint32_t ci_free_count;
	paddr_t ci_bitmap_addr;
};
typedef struct chunk_info chunk_info_t;

struct chunk_info_block {
	obj_phys_t cib_o;
	uint32_t cib_index;
	uint32_t cib_chunk_info_count;
	chunk_info_t cib_chunk_info[];
};
typedef struct chunk_info_block chunk_info_block_t;

struct cib_addr_block {
	obj_phys_t cab_o;
	uint32_t cab_index;
	uint32_t cab_cib_count;
	paddr_t cab_cib_addr[];
};
typedef struct cib_addr_block cib_addr_block_t;

typedef uint64_t spaceman_free_queue_val_t;

struct spaceman_free_queue_key {
	xid_t sfqk_xid;
	paddr_t sfqk_paddr;
};
typedef struct spaceman_free_queue_key spaceman_free_queue_key_t;

struct spaceman_free_queue_entry {
	spaceman_free_queue_key_t sfqe_key;
	spaceman_free_queue_val_t sfqe_count;
};
typedef struct spaceman_free_queue_entry spaceman_free_queue_entry_t;

struct spaceman_free_queue {
	uint64_t sfq_count;
	oid_t sfq_tree_oid;
	xid_t sfq_oldest_xid;
	uint16_t sfq_tree_node_limit;
	uint16_t sfq_pad16;
	uint32_t sfq_pad32;
	uint64_t sfq_reserved;
};
typedef struct spaceman_free_queue spaceman_free_queue_t;

struct spaceman_device {
	uint64_t sm_block_count;
	uint64_t sm_chunk_count;
	uint32_t sm_cib_count;
	uint32_t sm_cab_count;
	uint64_t sm_free_count;
	uint32_t sm_addr_offset;
	uint32_t sm_reserved;
	uint64_t sm_reserved2;
};
typedef struct spaceman_device spaceman_device_t;

struct spaceman_allocation_zone_boundaries {
	uint64_t saz_zone_start;
	uint64_t saz_zone_end;
};
typedef struct spaceman_allocation_zone_boundaries
	spaceman_allocation_zone_boundaries_t;

#define SM_ALLOCZONE_INVALID_END_BOUNDARY 0
#define SM_ALLOCZONE_NUM_PREVIOUS_BOUNDARIES 7

struct spaceman_allocation_zone_info_phys {
	spaceman_allocation_zone_boundaries_t saz_current_boundaries;
	spaceman_allocation_zone_boundaries_t
		saz_previous_boundaries[SM_ALLOCZONE_NUM_PREVIOUS_BOUNDARIES];
	uint16_t saz_zone_id;
	uint16_t saz_previous_boundary_index;
	uint32_t saz_reserved;
};
typedef struct spaceman_allocation_zone_info_phys
	spaceman_allocation_zone_info_phys_t;

enum smdev {
	SD_MAIN = 0,
	SD_TIER2 = 1,
	SD_COUNT = 2
};

#define SM_DATAZONE_ALLOCZONE_COUNT 8

struct spaceman_datazone_info_phys {
	spaceman_allocation_zone_info_phys_t
		sdz_allocation_zones[SD_COUNT][SM_DATAZONE_ALLOCZONE_COUNT];
};
typedef struct spaceman_datazone_info_phys spaceman_datazone_info_phys_t;

enum sfq {
	SFQ_IP = 0,
	SFQ_MAIN = 1,
	SFQ_TIER2 = 2,
	SFQ_COUNT = 3
};

struct spaceman_phys {
	obj_phys_t sm_o;
	uint32_t sm_block_size;
	uint32_t sm_blocks_per_chunk;
	uint32_t sm_chunks_per_cib;
	uint32_t sm_cibs_per_cab;
	spaceman_device_t sm_dev[SD_COUNT];
	uint32_t sm_flags;
	uint32_t sm_ip_bm_tx_multiplier;
	uint64_t sm_ip_block_count;
	uint32_t sm_ip_bm_size_in_blocks;
	uint32_t sm_ip_bm_block_count;
	paddr_t sm_ip_bm_base;
	paddr_t sm_ip_base;
	uint64_t sm_fs_reserve_block_count;
	uint64_t sm_fs_reserve_alloc_count;
	spaceman_free_queue_t sm_fq[SFQ_COUNT];
	uint16_t sm_ip_bm_free_head;
	uint16_t sm_ip_bm_free_tail;
	uint32_t sm_ip_bm_xid_offset;
	uint32_t sm_ip_bitmap_offset;
	uint32_t sm_ip_bm_free_next_offset;
	uint32_t sm_version;
	uint32_t sm_struct_size;
	spaceman_datazone_info_phys_t sm_datazone;
};
typedef struct spaceman_phys spaceman_phys_t;

#define SM_FLAG_VERSIONED 0x00000001

#define CI_COUNT_MASK 0x000fffff
#define CI_COUNT_RESERVED_MASK 0xfff00000

#define SPACEMAN_IP_BM_TX_MULTIPLIER 16
#define SPACEMAN_IP_BM_INDEX_INVALID 0xffff
#define SPACEMAN_IP_BM_BLOCK_COUNT_MAX 0xfffe


/**
 * Reaper
 */

struct nx_reaper_phys {
obj_phys_t nr_o;
	uint64_t nr_next_reap_id;
	uint64_t nr_completed_id;
	oid_t nr_head;
	oid_t nr_tail;
	uint32_t nr_flags;
	uint32_t nr_rlcount;
	uint32_t nr_type;
	uint32_t nr_size;
	oid_t nr_fs_oid;
	oid_t nr_oid;
	xid_t nr_xid;
	uint32_t nr_nrle_flags;
	uint32_t nr_state_buffer_size;
	uint8_t nr_state_buffer[];
};
typedef struct nx_reaper_phys nx_reaper_phys_t;

struct nx_reap_list_entry {
	uint32_t nrle_next;
	uint32_t nrle_flags;
	uint32_t nrle_type;
	uint32_t nrle_size;
	oid_t nrle_fs_oid;
	oid_t nrle_oid;
	xid_t nrle_xid;
};
typedef struct nx_reap_list_entry nx_reap_list_entry_t;

struct nx_reap_list_phys {
	obj_phys_t nrl_o;
	oid_t nrl_next;
	uint32_t nrl_flags;
	uint32_t nrl_max;
	uint32_t nrl_count;
	uint32_t nrl_first;
	uint32_t nrl_last;
	uint32_t nrl_free;
	nx_reap_list_entry_t nrl_entries[];
};
typedef struct nx_reap_list_phys nx_reap_list_phys_t;

enum {
	APFS_REAP_PHASE_START = 0,
	APFS_REAP_PHASE_SNAPSHOTS = 1,
	APFS_REAP_PHASE_ACTIVE_FS = 2,
	APFS_REAP_PHASE_DESTROY_OMAP = 3,
	APFS_REAP_PHASE_DONE = 4
};

#define NR_BHM_FLAG 0x00000001
#define NR_CONTINUE 0x00000002

#define NRLE_VALID 0x00000001
#define NRLE_REAP_ID_RECORD 0x00000002
#define NRLE_CALL 0x00000004
#define NRLE_COMPLETION 0x00000008
#define NRLE_CLEANUP 0x00000010

#define NRL_INDEX_INVALID 0xffffffff

struct omap_reap_state {
	uint32_t omr_phase;
	omap_key_t omr_ok;
};
typedef struct omap_reap_state omap_reap_state_t;

struct omap_cleanup_state {
	uint32_t omc_cleaning;
	uint32_t omc_omsflags;
	xid_t omc_sxidprev;
	xid_t omc_sxidstart;
	xid_t omc_sxidend;
	xid_t omc_sxidnext;
	omap_key_t omc_curkey;
};
typedef struct omap_cleanup_state omap_cleanup_state_t;

struct apfs_reap_state {
	uint64_t last_pbn;
	xid_t cur_snap_xid;
	uint32_t phase;
} __attribute__((packed));
typedef struct apfs_reap_state apfs_reap_state_t;

/**
 * Encryption Rolling
 */

struct er_state_phys_header {
	obj_phys_t ersb_o;
	uint32_t ersb_magic;
	uint32_t ersb_version;
};
typedef struct er_state_phys_header er_state_phys_header_t;

struct er_state_phys {
	er_state_phys_header_t ersb_header;
	uint64_t ersb_flags;
	uint64_t ersb_snap_xid;
	uint64_t ersb_current_fext_obj_id;
	uint64_t ersb_file_offset;
	uint64_t ersb_progress;
	uint64_t ersb_total_blk_to_encrypt;
	oid_t ersb_blockmap_oid;
	uint64_t ersb_tidemark_obj_id;
	uint64_t ersb_recovery_extents_count;
	oid_t ersb_recovery_list_oid;
	uint64_t ersb_recovery_length;
};
typedef struct er_state_phys er_state_phys_t;

struct er_state_phys_v1 {
	er_state_phys_header_t ersb_header;
	uint64_t ersb_flags;
	uint64_t ersb_snap_xid;
	uint64_t ersb_current_fext_obj_id;
	uint64_t ersb_file_offset;
	uint64_t ersb_fext_pbn;
	uint64_t ersb_paddr;
	uint64_t ersb_progress;
	uint64_t ersb_total_blk_to_encrypt;
	uint64_t ersb_blockmap_oid;
	uint32_t ersb_checksum_count;
	uint32_t ersb_reserved;
	uint64_t ersb_fext_cid;
	uint8_t ersb_checksum[0];
};
typedef struct er_state_phys er_state_phys_v1_t;

enum er_phase_enum {
	ER_PHASE_OMAP_ROLL = 1,
	ER_PHASE_DATA_ROLL = 2,
	ER_PHASE_SNAP_ROLL = 3,
};
typedef enum er_phase_enum er_phase_t;

struct er_recovery_block_phys {
	obj_phys_t erb_o;
	uint64_t erb_offset;
	oid_t erb_next_oid;
	uint8_t erb_data[0];
};
typedef struct er_recovery_block_phys er_recovery_block_phys_t;

struct gbitmap_block_phys {
	obj_phys_t bmb_o;
	uint64_t bmb_field[0];
};
typedef struct gbitmap_block_phys gbitmap_block_phys_t;

struct gbitmap_phys {
	obj_phys_t bm_o;
	oid_t bm_tree_oid;
	uint64_t bm_bit_count;
	uint64_t bm_flags;
};
typedef struct gbitmap_phys gbitmap_phys_t;

enum {
	ER_512B_BLOCKSIZE = 0,
	ER_2KiB_BLOCKSIZE = 1,
	ER_4KiB_BLOCKSIZE = 2,
	ER_8KiB_BLOCKSIZE = 3,
	ER_16KiB_BLOCKSIZE = 4,
	ER_32KiB_BLOCKSIZE = 5,
	ER_64KiB_BLOCKSIZE = 6,
};

#define ERSB_FLAG_ENCRYPTING 0x00000001
#define ERSB_FLAG_DECRYPTING 0x00000002
#define ERSB_FLAG_KEYROLLING 0x00000004
#define ERSB_FLAG_PAUSED 0x00000008
#define ERSB_FLAG_FAILED 0x00000010
#define ERSB_FLAG_CID_IS_TWEAK 0x00000020
#define ERSB_FLAG_FREE_1 0x00000040
#define ERSB_FLAG_FREE_2 0x00000080

#define ERSB_FLAG_CM_BLOCK_SIZE_MASK 0x00000F00
#define ERSB_FLAG_CM_BLOCK_SIZE_SHIFT 8

#define ERSB_FLAG_ER_PHASE_MASK 0x00003000
#define ERSB_FLAG_ER_PHASE_SHIFT 12
#define ERSB_FLAG_FROM_ONEKEY 0x00004000

#define ER_CHECKSUM_LENGTH 8
#define ER_MAGIC 'FLAB'
#define ER_VERSION 1

#define ER_MAX_CHECKSUM_COUNT_SHIFT 16
#define ER_CUR_CHECKSUM_COUNT_MASK 0x0000FFFF


/**
 * Fusion
 */

typedef struct {
	obj_phys_t fwp_objHdr;
	uint64_t fwp_version;
	oid_t fwp_listHeadOid;
	oid_t fwp_listTailOid;
	uint64_t fwp_stableHeadOffset;
	uint64_t fwp_stableTailOffset;
	uint32_t fwp_listBlocksCount;
	uint32_t fwp_reserved;
	uint64_t fwp_usedByRC;
	prange_t fwp_rcStash;
} fusion_wbc_phys_t;

typedef struct {
	paddr_t fwle_wbcLba;
	paddr_t fwle_targetLba;
	uint64_t fwle_length;
} fusion_wbc_list_entry_t;

typedef struct {
	obj_phys_t fwlp_objHdr;
	uint64_t fwlp_version;
	uint64_t fwlp_tailOffset;
	uint32_t fwlp_indexBegin;
	uint32_t fwlp_indexEnd;
	uint32_t fwlp_indexMax;
	uint32_t fwlp_reserved;
	fusion_wbc_list_entry_t fwlp_listEntries[];
} fusion_wbc_list_phys_t;

#define FUSION_TIER2_DEVICE_BYTE_ADDR 0x4000000000000000ULL
#define FUSION_TIER2_DEVICE_BLOCK_ADDR(_blksize) \
		(FUSION_TIER2_DEVICE_BYTE_ADDR >> __builtin_ctzl(_blksize))
#define FUSION_BLKNO(_fusion_tier2, _blkno, _blksize) \
		((_fusion_tier2) \
		? (FUSION_TIER2_DEVICE_BLOCK_ADDR(_blksize) | (_blkno)) \
		: (_blkno))

typedef paddr_t fusion_mt_key_t;

typedef struct {
	paddr_t fmv_lba;
	uint32_t fmv_length;
	uint32_t fmv_flags;
} fusion_mt_val_t;

#define FUSION_MT_DIRTY (1 << 0)
#define FUSION_MT_TENANT (1 << 1)
#define FUSION_MT_ALLFLAGS (FUSION_MT_DIRTY | FUSION_MT_TENANT)
