use std::io::{Error, Read};
use leb128::ReadLeb128Ext;
use byteorder::ReadBytesExt;

#[derive(Debug, PartialEq)]
pub struct ExportSection {
    pub count: u32,
    pub entries: Vec<ExportEntry>,
}

impl ExportSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<ExportSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut entries = Vec::new();

        for _ in 0..count {
            let entry = ExportEntry::from_reader(reader)?;
            entries.push(entry);
        }

        Ok(ExportSection {
            count: count as u32,
            entries,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ExportEntry {
    pub field_name_len: u32,
    pub field_name: String,
    pub kind: u8,
    pub index: u32,
}

impl ExportEntry {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<ExportEntry, Error> {
        let (field_name_len, _) = reader.leb128_unsigned()?;
        let mut buff = vec![0; field_name_len as usize];
        reader.read(&mut buff)?;
        let field_name = String::from_utf8_lossy(&buff).into_owned();

        let kind = reader.read_u8()?;
        let (index, _) = reader.leb128_unsigned()?;

        Ok(ExportEntry {
            field_name_len: field_name_len as u32,
            field_name,
            kind,
            index: index as u32,
        })
    }
}
