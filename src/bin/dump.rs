use std::{fs::File, cmp::min, io::{Cursor, BufRead}, env::VarError};
// use std::{convert::TryInto, borrow::Borrow, io::Write, os::unix::prelude::OsStrExt};

// use aes_keywrap::Aes128KeyWrap;
// use aes_keywrap_rs::{aes_unwrap_key, aes_unwrap_key_and_iv};
use apfs::{APFS, APFSObject, Btree, Oid, Paddr, StorageType, OvFlags, OmapVal, OmapRecord, ApfsValue, AnyRecords, InoExtType, InodeXdata, OmapKey, ObjectType, SpacemanFreeQueueValue, NX_EFI_JUMPSTART_MAGIC, NX_EFI_JUMPSTART_VERSION, load_btree_generic, LeafValue, BtreeTypes, MediaKeybag, ObjPhys, KbTag, Prange};
use der::{Decoder, TagNumber, asn1::OctetString, DecodeValue, FixedTag, Any};
use lzy_pbkdf2::pbkdf2_hmac_sha256;
// use der_derive::Sequence;

use std::{env, collections::HashMap};

use aes::{Aes128, cipher::KeyInit, cipher::generic_array::GenericArray};
use xts_mode::{Xts128, get_tweak_default};

fn dump_btree_records<V>(name: &str, btree: &Btree<V>, apfs: &mut APFS<File>, records: &AnyRecords<V>) where V: LeafValue {
    match records {
        AnyRecords::Leaf(_) => {},
        AnyRecords::NonLeaf(children, _) => {
            for child in children {
                let node_result = btree.load_btree_node(apfs, child.value.oid, StorageType::Physical);
                if node_result.is_err() {
                    println!("Error: {:?}", node_result.as_ref().err());
                }
                assert!(node_result.is_ok(), "Bad b-tree node load");
                let node = node_result.unwrap();
                println!("{} sub B-Tree: {:#?}", name, node);
                dump_btree_records(name, btree, apfs, &node.records);
            }
        },
    };
}

fn dump_btree(name: &str, apfs: &mut APFS<File>, oid: Oid) {
    let btree = load_btree_generic(apfs, oid, StorageType::Physical)
        .expect("Bad b-tree load");
    println!("{} B-Tree: {:#?}", name, &btree);
    match btree {
        BtreeTypes::ExtentRef(body) => dump_btree_records(name, &body, apfs, &body.root.records),
        BtreeTypes::SnapMetadata(body) => dump_btree_records(name, &body, apfs, &body.root.records),
        _ => { unimplemented!("Unsupported generic B-Tree"); },
    }
}

fn dump_omap_apfs_records(btree: &Btree<OmapVal>, apfs: &mut APFS<File>, records: &AnyRecords<OmapVal>) {
    match records {
        AnyRecords::Leaf(_) => {},
        AnyRecords::NonLeaf(children, _) => {
            for child in children {
                let node_result = btree.load_btree_node(apfs, child.value.oid, StorageType::Physical);
                if node_result.is_err() {
                    println!("Error: {:?}", node_result.as_ref().err());
                }
                assert!(node_result.is_ok(), "Bad b-tree node load");
                let node = node_result.unwrap();
                println!("Volume Object Map sub B-Tree: {:#?}", node);
                dump_omap_apfs_records(btree, apfs, &node.records);
            }
        },
    };
}

fn dump_apfs_records(btree: &Btree<ApfsValue>, apfs: &mut APFS<File>, omap_btree: &Btree<OmapVal>, records: &AnyRecords<ApfsValue>) {
    let empty = vec![];
    let file_records = match records {
        AnyRecords::Leaf(ref x) => x,
        AnyRecords::NonLeaf(children, _) => {
            for child in children {
                let root_object = omap_btree.get_record(apfs, &OmapKey::new(child.value.oid.0, u64::MAX))
                    .expect("I/O error")
                    .expect("Failed to find address for Volume root B-tree");
                let node_result = btree.load_btree_node(apfs, Oid(root_object.value.paddr.0 as u64), StorageType::Physical);
                let node = node_result.expect("Bad b-tree node load");
                println!("Volume Root sub B-Tree: {:#?}", &node);
                dump_apfs_records(btree, apfs, omap_btree, &node.records);
            }
            &empty
        },
    };
    let mut sizes = HashMap::<u64, u64>::new();
    for file_record in file_records {
        if let ApfsValue::Inode(ref y) = file_record.value {
            if let Some(&InodeXdata::Dstream(ref z)) = y.xdata.get(&InoExtType::Dstream) {
                sizes.insert(file_record.key.key.obj_id_and_type.id(), z.size);
            }
        } else if let ApfsValue::FileExtent(ref y) = file_record.value {
            if let Some(&length) = sizes.get(&file_record.key.key.obj_id_and_type.id()) {
                println!("Reading block: {} ({} bytes)", y.phys_block_num, length);
                if let Ok(block) = apfs.load_block(Paddr(y.phys_block_num as i64)) {
                    println!("Body: '{}'", String::from_utf8((&block[0..min(length as usize, block.len())]).to_owned()).unwrap_or_else(|_| String::from("(binary)")));
                }
            } else {
                println!("Missing inode for file!");
            }
        }
    }
}

// use der::{Sequence, asn1::Any};
// use der::asn1;
extern crate der_derive;
use der_derive::Sequence;

#[derive(Sequence)]
struct Bag {
    #[asn1(context_specific="0")]
    unknown_80: u8,
    #[asn1(context_specific="1")]
    iter: u64,
    // #[asn1(context_specific="1")]
    // salt: Vec<u8>,
}

#[derive(Debug)]
struct KeyBlob<'a> {
    unk_80: u64,
    hmac: OctetString<'a>,
    salt: OctetString<'a>,
    blob: &'a[u8],
}

use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use hex_literal::hex;

type HmacSha256 = Hmac<Sha256>;

#[test]
fn test_hmac_sha256() {
    let mut mac: HmacSha256 = Mac::new_from_slice(b"my secret and secure key")
        .expect("HMAC can take key of any size");
    mac.update(b"input message");

    // `result` has type `CtOutput` which is a thin wrapper around array of
    // bytes for providing constant time equality check
    let result = mac.finalize();
    // To get underlying array use `into_bytes`, but be careful, since
    // incorrect use of the code value may permit timing attacks which defeats
    // the security provided by the `CtOutput`
    let code_bytes = result.into_bytes();
    let expected = hex!("
        97d2a569059bbcd8ead4444ff99071f4
        c01d005bcefe0d3567e1be628e5fdcd9
    ");
    assert_eq!(code_bytes[..], expected[..]);
}

fn verify_key_blob(key: &KeyBlob) -> bool {
    const BLOB_COOKIE: [u8; 6] = [ 0x01, 0x16, 0x20, 0x17, 0x15, 0x05 ];

    assert_eq!(key.unk_80, 0);
    let mut hmac_key = Sha256::new();
    hmac_key.update(&BLOB_COOKIE);
    hmac_key.update(key.salt);
    let hk = hmac_key.finalize();
    println!("Debug out\n{:02x}\n{:02x}\n{:02x}", hk[0], key.blob[0], key.blob.len());

    let mut mac: HmacSha256 = Mac::new_from_slice(&hk)
        .expect("HMAC can take key of any size");
    mac.update(key.blob);
    let result = mac.finalize();
    let b = result.into_bytes();
    println!("Calc: {:02x?}, Expected: {:02x?}", &b, key.hmac);
    if &b[..] == &key.hmac.as_bytes()[..] {
        true
    } else {
        panic!("Failed HMAC!");
    }
}

fn main() {
    println!("Dumping file");
    let mut apfs = APFS::open(env::args().skip(1).next().unwrap()).unwrap();
    let superblock = match apfs.load_object_addr(Paddr(0)).unwrap() {
        APFSObject::Superblock(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    println!("Container Superblock: {:#?}", superblock);
    assert!(superblock.body.xp_desc_blocks & (1 << 31) == 0);
    assert!(superblock.body.xp_data_blocks & (1 << 31) == 0);
    for idx in 0..superblock.body.xp_desc_blocks {
        let object = apfs.load_object_addr(Paddr(superblock.body.xp_desc_base.0+idx as i64)).unwrap();
        println!("Checkpoint descriptor object: {:#?}", &object);
    }
    for idx in 0..superblock.body.xp_data_blocks {
        let object = apfs.load_object_addr(Paddr(superblock.body.xp_data_base.0+idx as i64));//.unwrap();
        println!("Checkpoint data object: {:#?}", &object);
        if let Ok(APFSObject::Spaceman(body)) = object {
            let subobject_result = apfs.load_object_addr(body.body.ip_base);
            if let Ok(subobject) = subobject_result {
                println!("Internal pool data object: {:#?}", &subobject);
            } else {
                println!("Error reading pool data: {:#?}", subobject_result);
            }
            // for subidx in 0..body.body.ip_block_count {
            //     let subobject = apfs.load_object_addr(Paddr(body.body.ip_base.0 + subidx as i64)).unwrap();
            //     println!("Internal pool data object: {:#?}", &subobject);
            // }
            // let subobject = apfs.load_object_addr(body.body.ip_bm_base).unwrap();
            // println!("Internal pool bitmap data object: {:#?}", &subobject);
        } else if let Ok(APFSObject::Btree(body)) = object {
            if body.header.subtype.r#type() == ObjectType::SpacemanFreeQueue {
                let btree = apfs.load_btree::<SpacemanFreeQueueValue>(Oid(superblock.body.xp_data_base.0 as u64 + idx as u64), StorageType::Physical)
                    .expect("Bad b-tree load");
                println!("Space Manager Free Queue B-Tree: {:#?}", btree);
                match &btree.root.records {
                    AnyRecords::Leaf(ref x) => {
                        for record in x {
                            let subobject_result = apfs.load_object_addr(record.key.paddr);
                            if let Ok(subobject) = subobject_result {
                                println!("SFQ Internal pool data object: {:#?}", &subobject);
                            } else {
                                println!("SFQ Error reading pool data: {:#?}", subobject_result);
                            }
                        }
                    },
                    AnyRecords::NonLeaf(_, _) => {},
                }
            }
        }
    }
    if superblock.body.efi_jumpstart != Paddr(0) {
        println!("Dumping Bootloader");
        let object = apfs.load_object_addr(superblock.body.efi_jumpstart).unwrap();
        let jumpstart = match object {
            APFSObject::EfiJumpstart(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        println!("EFI Jumpstart: {:#?}", &jumpstart);
        assert_eq!(jumpstart.body.magic, NX_EFI_JUMPSTART_MAGIC);
        assert_eq!(jumpstart.body.version, NX_EFI_JUMPSTART_VERSION);
        let mut loader = vec![];
        for range in jumpstart.body.rec_extents {
            for idx in 0..range.block_count {
                loader.push(apfs.load_block(Paddr(range.start_paddr.0 + idx as i64)).expect("Failed to load jumpstart block"));
            }
        }
        let mut loader_bytes = loader.into_iter().flatten().collect::<Vec<u8>>();
        loader_bytes.shrink_to(jumpstart.body.efi_file_len as usize);
        println!("Bootloader: {:?}", loader_bytes);
    }
    if superblock.body.keylocker.start_paddr.0 != 0 &&
       superblock.body.keylocker.block_count != 0 {
        println!("Found keylocker");
        let mut encrypted_bag = apfs.load_block(superblock.body.keylocker.start_paddr).expect("failed to load keybag");
        // println!("{:?}", encrypted_bag);
        let kek = superblock.body.uuid.as_bytes();

        let cipher_1 = Aes128::new(GenericArray::from_slice(kek));
        let cipher_2 = Aes128::new(GenericArray::from_slice(kek));

        let xts = Xts128::new(cipher_1, cipher_2);

        let sector_size = 0x200;
        let first_sector_index = superblock.body.keylocker.start_paddr.0 * (superblock.body.block_size as i64 / sector_size);

        xts.decrypt_area(&mut encrypted_bag, sector_size as usize, first_sector_index as u128, get_tweak_default);
        println!("Decrypt: {:#x?}", &encrypted_bag);
        println!("Decrypt text: {:x?}", String::from_utf8_lossy(&encrypted_bag));
        let mut keybag_cursor = Cursor::new(&encrypted_bag);
        let decoded_header = ObjPhys::import(&mut keybag_cursor).expect("Failed to decode");
        let decoded = MediaKeybag::import(&mut keybag_cursor).expect("Failed to decode");
        println!("Decoded keybag header: {:#x?}", decoded_header);
        println!("Decoded keybag: {:#x?}", decoded);
        for entry in decoded.locker.entries {
            if entry.tag == KbTag::VolumeUnlockRecords {
                let mut unlock_cursor = Cursor::new(&entry.keydata);
                let block_range = Prange::import(&mut unlock_cursor).expect("Invalid Prange for keybag");
                let kek = entry.uuid.as_bytes();

                let cipher_1 = Aes128::new(GenericArray::from_slice(kek));
                let cipher_2 = Aes128::new(GenericArray::from_slice(kek));

                let xts = Xts128::new(cipher_1, cipher_2);

                // let sector_size = 0x200;
                let first_sector_index = block_range.start_paddr.0 * (superblock.body.block_size as i64 / sector_size);
                let mut encrypted_bag = apfs.load_block(block_range.start_paddr).expect("failed to load volume keybag");
                xts.decrypt_area(&mut encrypted_bag, sector_size as usize, first_sector_index as u128, get_tweak_default);
                println!("Volume keybag decrypt: {:#x?}", &encrypted_bag);
                let mut keybag_cursor = Cursor::new(&encrypted_bag);
                let decoded_header = ObjPhys::import(&mut keybag_cursor).expect("Failed to decode");
                let decoded = MediaKeybag::import(&mut keybag_cursor).expect("Failed to decode volume keybag");
                println!("Decoded volume keybag header: {:#x?}", decoded_header);
                println!("Decoded volume keybag: {:#x?}", decoded);
                for entry in decoded.locker.entries {
                    if entry.tag == KbTag::VolumeUnlockRecords {
                        let mut value = Decoder::new(&entry.keydata).expect("Bad DER encoding");
                        value.sequence(|value| {
                            #[derive(Debug)]
                            struct Inner<'a> {
                                unk_80: u64,
                                uuid: OctetString<'a>,
                                unk_82: OctetString<'a>,
                                wrapped_kek: OctetString<'a>,
                                iterations: u64,
                                salt: OctetString<'a>,
                            }
                            impl FixedTag for Inner<'_> {
                                const TAG: der::Tag = der::Tag::ContextSpecific { constructed: true, number: TagNumber::N3 };
                            }
                            impl<'a> DecodeValue<'a> for Inner<'a> {
                                fn decode_value(decoder: &mut Decoder<'a>, _: der::Length) -> der::Result<Self> {
                                    let unk_80: u64 = decoder.context_specific(TagNumber::N0, der::TagMode::Implicit).expect("Invalid field").expect("Value");
                                    let uuid: OctetString = decoder.context_specific(TagNumber::N1, der::TagMode::Implicit).expect("Bad bytes").expect("Value");
                                    let unk_82: OctetString = decoder.context_specific(TagNumber::N2, der::TagMode::Implicit).expect("Bad bytes").expect("Value");
                                    let wrapped_kek: OctetString = decoder.context_specific(TagNumber::N3, der::TagMode::Implicit).expect("Bad bytes").expect("Value");
                                    let iterations: u64 = decoder.context_specific(TagNumber::N4, der::TagMode::Implicit).expect("Bad num").expect("Value");
                                    let salt: OctetString = decoder.context_specific(TagNumber::N5, der::TagMode::Implicit).expect("Bad bytes").expect("Value");
                                    Ok(Self {
                                        unk_80,
                                        uuid,
                                        unk_82,
                                        wrapped_kek,
                                        iterations,
                                        salt,
                                    })
                                }
                            }
                            let key = KeyBlob {
                                unk_80: value.context_specific(TagNumber::N0, der::TagMode::Implicit).expect("Invalid field").expect("Value"),
                                hmac: value.context_specific(TagNumber::N1, der::TagMode::Implicit).expect("Bad bytes").expect("Value"),
                                salt: value.context_specific(TagNumber::N2, der::TagMode::Implicit).expect("bad num").expect("Value"),
                                blob: {
                                    let pos: u32 = value.position().into();
                                    &entry.keydata[pos as usize..]
                                },
                            };
                            verify_key_blob(&key);
                            let inner: Inner = value.context_specific(TagNumber::N3, der::TagMode::Implicit).expect("bad num").expect("Value");
                            // println!("Volume Bag: {} - {:?} - {:?} - {:?} ({})", unk_80, hmac, salt, inner, blob.value().len());
                            println!("Volume Bag: {:?} - {:?} ({})", key, inner, key.blob.len());
                            let passwd = "";
                            let passwd = std::env::var("APFS_PASSWD").or_else(|_: VarError | -> std::io::Result<String> {
                                let mut passwd = String::new();
                                println!("APFS Password: ");
                                // std::io::stdin().flush();
                                std::io::stdin().lock().read_line(&mut passwd)?;
                                println!("Got password: {}", &passwd);
                                Ok(passwd)
                            }).expect("Failed to read password");
                            let hash = pbkdf2_hmac_sha256(passwd, inner.salt, 32, inner.iterations as usize);
                            assert!(hash.len() > 0, "Failed to PBKDF2 password");
                            assert_eq!(hash.len(), 32, "PBKDF2 password hash is short");
                            println!("PW Key  : {:02x?}", hash);
                            println!("KEK Wrpd: {:02x?}", inner.wrapped_kek);
                            Ok(())
                        }).expect("Failed to decode");
                        // let mut dump_file = File::create("keybag-volume.raw").expect("Can't open dump file for keybag");
                        // dump_file.write_all(&mut entry.keydata.clone()).expect("failed to save keybag");
                    }
                }
            } else {
                let mut value = Decoder::new(&entry.keydata).expect("Bad DER encoding");
                value.sequence(|value| {
                    #[derive(Debug)]
                    struct Inner<'a> {
                        unk_80: u64,
                        uuid: OctetString<'a>,
                        unk_82: OctetString<'a>,
                        wrapped_vek: OctetString<'a>,
                    }
                    impl FixedTag for Inner<'_> {
                        const TAG: der::Tag = der::Tag::ContextSpecific { constructed: true, number: TagNumber::N3 };
                    }
                    impl<'a> DecodeValue<'a> for Inner<'a> {
                        fn decode_value(decoder: &mut Decoder<'a>, _: der::Length) -> der::Result<Self> {
                            let unk_80: u64 = decoder.context_specific(TagNumber::N0, der::TagMode::Implicit).expect("Invalid field").expect("Value");
                            let uuid: OctetString = decoder.context_specific(TagNumber::N1, der::TagMode::Implicit).expect("Bad bytes").expect("Value");
                            let unk_82: OctetString = decoder.context_specific(TagNumber::N2, der::TagMode::Implicit).expect("Bad bytes").expect("Value");
                            let wrapped_vek: OctetString = decoder.context_specific(TagNumber::N3, der::TagMode::Implicit).expect("Bad bytes").expect("Value");
                            Ok(Self {
                                unk_80,
                                uuid,
                                unk_82,
                                wrapped_vek,
                            })
                        }
                    }
                    // let unknown_80: bool = value.context_specific(TagNumber::N0, der::TagMode::Implicit).expect("Invalid field").expect("Value");
                    // let body: OctetString = value.context_specific(TagNumber::N1, der::TagMode::Implicit).expect("Bad bytes").expect("Value");
                    // let num = ContextSpecific::<u64>::decode_implicit(value, TagNumber::N2).expect("bad num").expect("Value").value;
                    // let raw = value.clone().any().expect("bad any");
                    // let inner: Inner = value.context_specific(TagNumber::N3, der::TagMode::Implicit).expect("bad inner").expect("Value");
                    // println!("Bag: {} - {:?} - {} - {:?} ({})", unknown_80, body, num, inner, raw.value().len());
                    let key = KeyBlob {
                        unk_80: value.context_specific(TagNumber::N0, der::TagMode::Implicit).expect("Invalid field").expect("Value"),
                        hmac: value.context_specific(TagNumber::N1, der::TagMode::Implicit).expect("Bad bytes").expect("Value"),
                        salt: value.context_specific(TagNumber::N2, der::TagMode::Implicit).expect("bad num").expect("Value"),
                        blob: {
                            let pos: u32 = value.position().into();
                            &entry.keydata[pos as usize..]
                        },
                    };
                    verify_key_blob(&key);
                    let inner: Inner = value.context_specific(TagNumber::N3, der::TagMode::Implicit).expect("bad num").expect("Value");
                    println!("Bag: {:?} - {:?} ({})", key, inner, key.blob.len());
                    Ok(())
                }).expect("Failed to decode");
                // let mut dump_file = File::create("keybag.raw").expect("Can't open dump file for keybag");
                // dump_file.write_all(&mut entry.keydata.clone()).expect("failed to save keybag");
            }
        }
        // let bag = aes_unwrap_key(&kek, &encrypted_bag).unwrap();
        // let test_kek = hex::decode("000102030405060708090A0B0C0D0E0F").unwrap();
        // let test_plain = hex::decode("00112233445566778899AABBCCDDEEFF").unwrap();
        // let test_cipher = hex::decode("1FA68B0A8112B447AEF34BD8FB5A7B829D3E862371D2CFE5").unwrap();
        // assert_eq!(aes_unwrap_key(&test_kek, &test_cipher).unwrap(), test_plain, "Failed to match test key data on old lib");
        // assert_eq!(Aes128KeyWrap::new(&test_kek.try_into().unwrap()).decapsulate(&test_cipher, test_plain.len()).expect("Failed to decrypt test bag"), test_plain);
        // for len in 3900..encrypted_bag.len()+1 {
        // println!("Key: 0x{:x?}", superblock.body.uuid.as_bytes());
        // for len in 0..encrypted_bag.len()+1 {
        //     // println!("Decode keybag attempt {}: {:?}", len, aes_unwrap_key(&kek[..], &encrypted_bag[0..len]))
        //     println!("Decode keybag attempt {}: {:?}", len, aes_unwrap_key(&kek[..], &encrypted_bag[len..]))
        // }
        // match aes_unwrap_key_and_iv(&kek[..], &encrypted_bag[0..4096]) {
        // match aes_unwrap_key(&kek[..], &encrypted_bag) {
        // match Aes128KeyWrap::new(kek).decapsulate(&encrypted_bag, encrypted_bag.len()-8) {
        //     Ok(bag) => {
        //         println!("{:?}", bag);
        //     },
        //     Err(error) => {
        //         println!("Failed to decrypt keybag: {:?}", error);
        //     }
        // }
        // println!("{:?}", apfs.load_object_addr(superblock.body.keylocker.start_paddr));
    }
    let object = apfs.load_object_oid(superblock.body.omap_oid, StorageType::Physical).unwrap();
    let omap = match object {
        APFSObject::ObjectMap(x) => x,
        _ => { panic!("Wrong object type!"); },
    };
    let btree_result = apfs.load_btree::<OmapVal>(omap.body.tree_oid, StorageType::Physical);
    if btree_result.is_err() {
        println!("Error: {:?}", btree_result.as_ref().err());
    }
    assert!(btree_result.is_ok(), "Bad b-tree load");
    let btree = btree_result.unwrap();
    println!("Superblock Object Map B-Tree: {:#?}", btree);
    let records: &Vec<OmapRecord> = match &btree.root.records {
        AnyRecords::Leaf(ref x) => x,
        _ => { panic!("Wrong b-tree record type!"); },
    };
    for record in records {
        let object = apfs.load_object_addr(record.value.paddr).unwrap();
        let volume = match object {
            APFSObject::ApfsSuperblock(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        println!("Volume Superblock: {:#?}", volume);
        let object = apfs.load_object_oid(volume.body.omap_oid, StorageType::Physical).unwrap();
        let omap = match object {
            APFSObject::ObjectMap(x) => x,
            _ => { panic!("Wrong object type!"); },
        };
        let btree = apfs.load_btree::<OmapVal>(omap.body.tree_oid, StorageType::Physical)
            .expect("Bad b-tree load");
        println!("Volume Object Map B-Tree: {:#?}", &btree);
        dump_omap_apfs_records(&btree, &mut apfs, &btree.root.records);
        dump_btree("Volume Extent Reference", &mut apfs, volume.body.extentref_tree_oid);
        dump_btree("Volume Snapshot Metadata", &mut apfs, volume.body.snap_meta_tree_oid);
        if volume.body.snap_meta_ext_oid != Oid(0) {
            let root_object = btree.get_record(&mut apfs, &OmapKey::new(volume.body.snap_meta_ext_oid.0, u64::MAX))
                .expect("I/O error")
                .expect("Failed to find address for Volume Snapshot extended data");
            let object = apfs.load_object_oid(Oid(root_object.value.paddr.0 as u64), StorageType::Physical).unwrap();
            let ext = match object {
                APFSObject::SnapMetaExt(x) => x,
                _ => { panic!("Wrong object type!"); },
            };
            println!("Volume Snapshot extended data: {:#?}", ext);
        }

        let root_object = btree.get_record(&mut apfs, &OmapKey::new(volume.body.root_tree_oid.0, u64::MAX))
            .expect("I/O error")
            .expect("Failed to find address for Volume root B-tree");
        if root_object.value.flags.contains(OvFlags::ENCRYPTED) {
            println!("Encrypted volume found, skipping...");
            continue;
        }
        let root_btree = apfs.load_btree::<ApfsValue>(Oid(root_object.value.paddr.0 as u64), StorageType::Physical)
            .expect("Failed to load volume root B-tree");
        println!("Volume Root B-Tree: {:#?}", &root_btree);
        dump_apfs_records(&root_btree, &mut apfs, &btree, &root_btree.root.records);

        // let btree_result = apfs.load_btree(volume.body.root_tree_oid, StorageType::Physical);
    }
}
