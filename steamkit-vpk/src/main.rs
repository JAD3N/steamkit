use std::{fs::File, io::Read};

use nom_derive::Parse;
use steamkit_vpk::*;

fn main() {
    let mut file = File::open("assets/pak01_dir.vpk").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let (_, vpk) = Vpk::parse(&buf).unwrap();
    println!("{vpk:?}");
}
