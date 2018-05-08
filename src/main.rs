extern crate wasmparse;

use std::fs::File;
use std::io::{Cursor, Read};

fn main() {
    let mut buff = Vec::new();
    let mut f = File::open("files/test-start.wasm").expect("file not found");
    f.read_to_end(&mut buff).unwrap();

    let module = wasmparse::parse(Cursor::new(buff));
    println!("{:#?}", module);
}
