use apfs::{APFS, Paddr};

use std::env;

fn main() {
    println!("Dumping file");
    let mut file = APFS::open(env::args().skip(1).next().unwrap()).unwrap();
    let superblock = file.load_object_addr(Paddr(0)).unwrap();
    println!("Superblock: {:?}", superblock);
}
