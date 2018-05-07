use std::io::{Error, Read};
use leb128::ReadLeb128Ext;
use byteorder::ReadBytesExt;

use super::*;

#[derive(Debug, PartialEq)]
pub struct ImportSection {
    pub count: u32,
    pub entries: Vec<ImportEntry>,
}

impl ImportSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<ImportSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut entries = Vec::new();

        for _ in 0..count {
            let entry = ImportEntry::from_reader(reader)?;
            entries.push(entry);
        }

        Ok(ImportSection { count: count as u32, entries })
    }
}

#[derive(Debug, PartialEq)]
pub struct ImportEntry {
    pub module_name_len: u32,
    pub module_name: String,
    pub field_name_len: u32,
    pub field_name: String,
    pub kind: ExternalKind,
}

impl ImportEntry {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<ImportEntry, Error> {
        let (module_name_len, _) = reader.leb128_unsigned()?;

        let mut buff = vec![0; module_name_len as usize];
        reader.read(&mut buff)?;
        let module_name = String::from_utf8_lossy(&buff).into_owned();

        let (field_name_len, _) = reader.leb128_unsigned()?;
        let mut buff = vec![0; field_name_len as usize];
        reader.read(&mut buff)?;
        let field_name = String::from_utf8_lossy(&buff).into_owned();

        let external_type_code = reader.read_u8().unwrap();
        let kind = ExternalKind::from_u8(external_type_code)?;

        let _ = reader.leb128_unsigned()?;

        Ok(ImportEntry {
            module_name_len: module_name_len as u32,
            module_name,
            field_name_len: field_name_len as u32,
            field_name,
            kind,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum ExternalKind {
    Function,
    Table,
    Memory,
    Global,
}

impl ExternalKind {
    pub fn from_u8(num: u8) -> Result<ExternalKind, Error> {
        match num {
            0 => Ok(ExternalKind::Function),
            1 => Ok(ExternalKind::Table),
            2 => Ok(ExternalKind::Memory),
            3 => Ok(ExternalKind::Global),
            _ => Err(Error::new(
                ErrorKind::Other,
                "Unknown External Kind",
            )),
        }
    }
}