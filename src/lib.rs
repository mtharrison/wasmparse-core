extern crate byteorder;
#[macro_use]
extern crate serde_derive;

mod leb128;
mod types;

use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use leb128::ReadLeb128Ext;
use types::*;

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
        name = Some(nam);
        payload_len -= name_len as u32;
        payload_len -= name_len_bytes as u32;
    }

    println!("Found code {}", code);

    let body = match code {
        1 => WasmSectionBody::Types(Box::new(parse_type_section(reader))),
        3 => WasmSectionBody::Function(Box::new(parse_function_section(reader))),
        _ => WasmSectionBody::Custom(Box::new(parse_custom_section(reader, payload_len as usize))),
    };

    Some(WasmSection {
        payload_len,
        name,
        body,
    })
}

fn number_to_value_type(number: i64) -> ValueType {
    match number {
        -0x01 => ValueType::Integer32,
        -0x02 => ValueType::Integer64,
        -0x03 => ValueType::Float32,
        -0x04 => ValueType::Float64,
        -0x10 => ValueType::Anyfunc,
        -0x20 => ValueType::Func,
        -0x40 => ValueType::EmptyBlockType,
        t @ _ => panic!("Uknown value type {}", t),
    }
}

fn read_leb128_unsigned_value<T: Read>(reader: &mut T) -> u32 {
    let value = reader.leb128_unsigned().expect("Parse error").0;
    value as u32
}

fn read_value_type<T: Read>(reader: &mut T) -> ValueType {
    let (form_num, _) = reader.leb128_signed().expect("Parse error");
    number_to_value_type(form_num)
}

fn parse_type_section<T: Read>(reader: &mut T) -> TypeSection {
    let count = read_leb128_unsigned_value(reader);

    let mut entries = Vec::new();

    for _ in 0..count {
        let form = read_value_type(reader);
        let param_count = read_leb128_unsigned_value(reader);
        let mut param_types = Vec::new();

        for _ in 0..param_count {
            param_types.push(read_value_type(reader));
        }

        let return_count = read_leb128_unsigned_value(reader);
        let return_type = match return_count {
            1 => Some(read_value_type(reader)),
            _ => None,
        };

        let entry = FunctionType {
            form,
            param_count,
            param_types,
            return_count,
            return_type,
        };

        entries.push(entry);
    }

    TypeSection { count, entries }
}
fn parse_function_section<T: Read>(reader: &mut T) -> FunctionSection {
    let count = read_leb128_unsigned_value(reader);

    let mut types = Vec::new();

    for _ in 0..count {
        types.push(read_leb128_unsigned_value(reader));
    }

    FunctionSection { count, types }
}

fn parse_custom_section<T: Read>(reader: &mut T, len: usize) -> CustomSection {
    let mut data = vec![0; len];
    reader
        .read_exact(&mut data)
        .expect("Cannot read data from custom section");
    CustomSection { len, data }
}

pub fn parse<T: Read>(mut rdr: T) -> Result<WasmModule, String> {
    let magic = rdr.read_u32::<LittleEndian>().unwrap();

    if magic != WASM_MAGIC_NUMBER {
        return Err(format!(
            "Magic number 0x{:x} is not the expected value 0x{:x}",
            magic, WASM_MAGIC_NUMBER
        ));
    }

    let version = rdr.read_u32::<LittleEndian>().unwrap();

    if version != WASM_VERSION_KNOWN {
        return Err(format!("Unknown WASM version {}", version));
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
