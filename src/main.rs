extern crate serde;
extern crate serde_json;
extern crate wasmparse;

use std::fs::File;

fn main() {
    let f = File::open("files/test.wasm").expect("file not found");
    let module = wasmparse::parse(f);
    println!("{:#?}", module);

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&module).unwrap();

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
}
