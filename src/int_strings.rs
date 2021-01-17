//#[cfg(target_endian = "little")]
//#[macro_export]
//macro_rules! u16_code {
//    ($w:expr) => {
//        ((($w[0] as u16) <<  0) |
//         (($w[1] as u16) <<  8) |
//         ((*$w as [u8; 2])[0] as u16 * 0))
//    }
//}
//#[cfg(target_endian = "big")]
//#[macro_export]
//macro_rules! u16_code {
//    ($w:expr) => {
//        ((($w[1] as u16) <<  0) |
//         (($w[0] as u16) <<  8) |
//         ((*$w as [u8; 2])[0] as u16 * 0))
//    }
//}
//
//#[cfg(target_endian = "little")]
//#[macro_export]
//macro_rules! u32_code {
//    ($w:expr) => {
//        ((($w[0] as u32) <<  0) |
//         (($w[1] as u32) <<  8) |
//         (($w[2] as u32) << 16) |
//         (($w[3] as u32) << 24) |
//         ((*$w as [u8; 4])[0] as u32 * 0))
//    }
//}
//
//#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! u32_code {
    ($w:expr) => {
        ((($w[3] as u32) <<  0) |
         (($w[2] as u32) <<  8) |
         (($w[1] as u32) << 16) |
         (($w[0] as u32) << 24) |
         ((*$w as [u8; 4])[0] as u32 * 0))
    }
}
//
//#[cfg(target_endian = "little")]
//#[macro_export]
//macro_rules! u64_code {
//    ($w:expr) => {
//        ((($w[0] as u64) <<  0) |
//         (($w[1] as u64) <<  8) |
//         (($w[2] as u64) << 16) |
//         (($w[3] as u64) << 24) |
//         (($w[4] as u64) << 32) |
//         (($w[5] as u64) << 40) |
//         (($w[6] as u64) << 48) |
//         (($w[7] as u64) << 56) |
//         ((*$w as [u8; 8])[0] as u64 * 0))
//    }
//}
//#[cfg(target_endian = "big")]
//#[macro_export]
//macro_rules! u64_code {
//    ($w:expr) => {
//        ((($w[7] as u64) <<  0) |
//         (($w[6] as u64) <<  8) |
//         (($w[5] as u64) << 16) |
//         (($w[4] as u64) << 24) |
//         (($w[3] as u64) << 32) |
//         (($w[2] as u64) << 40) |
//         (($w[1] as u64) << 48) |
//         (($w[0] as u64) << 56) |
//         ((*$w as [u8; 8])[0] as u64 * 0))
//    }
//}
