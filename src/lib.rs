extern crate byteorder;

mod leb128;
mod types;

use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

use leb128::ReadLeb128Ext;
use types::*;

static WASM_MAGIC_NUMBER: u32 = 0x6d736100;
static WASM_VERSION_KNOWN: u32 = 0x01;

fn parse_section<T: Read>(reader: &mut T) -> Result<Option<WasmSection>, Error> {
    let code = match reader.read_u8() {
        Ok(code) => code,
        Err(_) => return Ok(None),
    };

    let (mut payload_len, _) = reader.leb128_unsigned()?;
    let mut name = None;

    if code == 0 {
        let (name_len, name_len_bytes) = reader.leb128_unsigned()?;
        let mut n = vec![0; name_len as usize];
        reader.read(&mut n)?;
        let nam = String::from_utf8_lossy(&n).into_owned();
        name = Some(nam);

        payload_len -= name_len;
        payload_len -= name_len_bytes as i64;
    }

    println!("Got code {}", code);

    let body = match code {
        1 => WasmSectionBody::Types(Box::new(TypeSection::from_reader(reader)?)),
        2 => WasmSectionBody::Import(Box::new(ImportSection::from_reader(reader)?)),
        3 => WasmSectionBody::Function(Box::new(FunctionSection::from_reader(reader)?)),
        4 => WasmSectionBody::Table(Box::new(TableSection::from_reader(reader)?)),
        5 => WasmSectionBody::Memory(Box::new(MemorySection::from_reader(reader)?)),
        6 => WasmSectionBody::Global(Box::new(GlobalSection::from_reader(reader)?)),
        // Implement global section
        7 => WasmSectionBody::Export(Box::new(ExportSection::from_reader(reader)?)),
        8 => WasmSectionBody::Start(Box::new(StartSection::from_reader(reader)?)),
        // Implement element section
        10 => WasmSectionBody::Code(Box::new(CodeSection::from_reader(reader)?)),
        11 => WasmSectionBody::Data(Box::new(DataSection::from_reader(reader)?)),
        _ => WasmSectionBody::Custom(Box::new(CustomSection::from_reader(
            reader,
            payload_len as usize,
        )?)),
    };

    Ok(Some(WasmSection {
        payload_len: payload_len as u32,
        name,
        body,
    }))
}

pub fn parse<T: Read + Seek>(mut rdr: T) -> Result<WasmModule, Error> {
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
        let section = parse_section(&mut rdr).unwrap_or_else(|err| {
            let position = rdr.seek(SeekFrom::Current(0)).unwrap();
            panic!(format!("PARSE ERROR AT 0x{:012X}: {}", position, err));
        });
        match section {
            Some(section) => module.sections.push(section),
            None => break,
        }
    }

    return Ok(module);
}
