extern crate byteorder;

mod leb128;
mod types;

use std::io::{Cursor, Read};
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
        let nam = unsafe { String::from_utf8_unchecked(n) };
        name = Some(nam);
        payload_len -= name_len as u32;
        payload_len -= name_len_bytes as u32;
    }

    let mut payload_bytes = vec![0u8; (payload_len) as usize];
    reader.read_exact(&mut payload_bytes).expect("Parse error");

    let body = match code {
        1 => WasmSectionBody::Types(Box::new(parse_type_section(payload_bytes))),
        _ => WasmSectionBody::Custom(payload_bytes),
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

fn parse_type_section(data: Vec<u8>) -> TypeSection {
    let mut c = Cursor::new(data);
    let count = read_leb128_unsigned_value(&mut c);

    let mut entries = Vec::new();

    for _ in 0..count {
        let form = read_value_type(&mut c);
        let param_count = read_leb128_unsigned_value(&mut c);
        let mut param_types = Vec::new();

        for _ in 0..param_count {
            param_types.push(read_value_type(&mut c));
        }

        let return_count = read_leb128_unsigned_value(&mut c);
        let return_type = match return_count {
            1 => Some(read_value_type(&mut c)),
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

    // Parse first section

    loop {
        let section = parse_section(&mut rdr);
        match section {
            Some(section) => module.sections.push(section),
            None => break,
        }
    }

    return Ok(module);
}
