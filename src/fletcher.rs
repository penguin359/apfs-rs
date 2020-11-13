#[cfg(test)]
mod test {
    use super::*;

    use std::fs::File;
    use std::io::Cursor;
    use std::io::prelude::*;
    use std::path::PathBuf;

    use byteorder::{LittleEndian, ReadBytesExt};

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

fn fletcher64(_buffer: &[u8]) -> u64 {
    return 0;
}
