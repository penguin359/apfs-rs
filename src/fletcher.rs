#[cfg(test)]
mod test {
    use super::*;

    use std::fs::File;
    use std::io::Cursor;
    use std::io::prelude::*;
    use std::path::PathBuf;

    use byteorder::{LittleEndian, ReadBytesExt};

    #[test]
    fn test_simple_fletcher64() {
        let buffer = include_bytes!("../testdata/volume_superblock.1");
        let mut cursor = Cursor::new(buffer);
        let checksum = cursor.read_u64::<LittleEndian>().unwrap();
        assert_eq!(fletcher64(&buffer[8..]), checksum);
    }

    fn test_dir() -> PathBuf {
        let root = ::std::env::var_os("CARGO_MANIFEST_DIR").map(|x| PathBuf::from(x))
            .unwrap_or_else(|| ::std::env::current_dir().unwrap());
        root.join("testdata")
    }

    #[test]
    fn test_read_checksum() {
        let mut buffer = [0u8; 16];
        let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
        file.read_exact(&mut buffer).unwrap();
        let mut cursor = Cursor::new(&buffer);
        let checksum = cursor.read_u64::<LittleEndian>().unwrap();
        assert_eq!(checksum, 0x109452a1ced0d551);
    }

    #[test]
    fn test_fletcher64() {
        let mut buffer = [0u8; 4096];
        let mut file = File::open(test_dir().join("test-apfs.img")).unwrap();
        file.read_exact(&mut buffer).unwrap();
        assert_eq!(fletcher64(&buffer[8..]), 0x109452a1ced0d551);
    }
}

fn fletcher64(buffer: &[u8]) -> u64 {
    let initial_value = 0u64;

    if buffer.len() % 4 != 0 {
        panic!("Bad size");  // TODO turn into error?
    }
    let mut lower_32bit = initial_value & 0xffffffff;
    let mut upper_32bit = (initial_value >> 32) & 0xffffffff;

    for buffer_offset in (0..buffer.len()).step_by(4) {
        let value_32bit = ((buffer[buffer_offset+0] as u64) <<  0) |
                          ((buffer[buffer_offset+1] as u64) <<  8) |
                          ((buffer[buffer_offset+2] as u64) << 16) |
                          ((buffer[buffer_offset+3] as u64) << 24);

        lower_32bit += value_32bit;
        upper_32bit += lower_32bit;
    }
    lower_32bit %= 0xffffffff;
    upper_32bit %= 0xffffffff;

    let value_32bit = 0xffffffff - ((lower_32bit + upper_32bit) % 0xffffffff);
    upper_32bit = 0xffffffff - ((lower_32bit + value_32bit) % 0xffffffff);

    return (upper_32bit << 32) | value_32bit;
}
