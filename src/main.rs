extern crate wasmparse;

use std::fs::File;

fn main() {
    let f = File::open("files/test-import.wasm").expect("file not found");
    let module = wasmparse::parse(f);
    println!("{:#?}", module);
}
