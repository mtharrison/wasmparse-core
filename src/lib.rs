extern crate byteorder;

mod leb128;
mod types;

use std::io::{Error, Read};
use byteorder::{LittleEndian, ReadBytesExt};

use leb128::ReadLeb128Ext;
use types::*;
use types::custom_section::*;
use types::function_section::*;
use types::import_section::*;
use types::type_section::*;

static WASM_MAGIC_NUMBER: u32 = 0x6d736100;
static WASM_VERSION_KNOWN: u32 = 0x01;


fn parse_section<T: Read>(reader: &mut T) -> Option<WasmSection> {
    let code = match reader.read_u8() {
        Ok(code) => code,
        Err(_) => return None,
    };

    let mut payload_len = read_leb128_unsigned_value(reader);
    let mut name = None;

    if code == 0 {
        let (name_len, name_len_bytes) = reader.leb128_unsigned().expect("Parse error");
        let mut n = vec![0; name_len as usize];
        reader.read(&mut n).unwrap();
        let nam = String::from_utf8_lossy(&n).into_owned();
        println!("{} {} {:?}", name_len, name_len_bytes, n);
        name = Some(nam);

        payload_len -= name_len as u32;
        payload_len -= name_len_bytes as u32;
    }

    println!("Found code {}", code);

    let body = match code {
        1 => WasmSectionBody::Types(Box::new(TypeSection::from_reader(reader).expect("Parse error"))),
        2 => WasmSectionBody::Import(Box::new(ImportSection::from_reader(reader).expect("Parse error"))),
        3 => WasmSectionBody::Function(Box::new(FunctionSection::from_reader(reader).expect("Parse error"))),
        _ => WasmSectionBody::Custom(Box::new(CustomSection::from_reader(reader, payload_len as usize).expect("Parse error"))),
    };

    Some(WasmSection {
        payload_len,
        name,
        body,
    })
}

pub fn parse<T: Read>(mut rdr: T) -> Result<WasmModule, Error> {
    let magic = rdr.read_u32::<LittleEndian>()?;

    if magic != WASM_MAGIC_NUMBER {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Magic number 0x{:x} is not the expected value 0x{:x}",
                magic, WASM_MAGIC_NUMBER
            ),
        ));
    }

    let version = rdr.read_u32::<LittleEndian>()?;

    if version != WASM_VERSION_KNOWN {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Unknown WASM version {}", version),
        ));
    }

    let mut module = WasmModule {
        version,
        sections: Vec::new(),
    };

    loop {
        let section = parse_section(&mut rdr);
        match section {
            Some(section) => module.sections.push(section),
            None => break,
        }
    }

    return Ok(module);
}
