use leb128::ReadLeb128Ext;
use std::io::{Error, Read};

use super::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct GlobalSection {
    pub count: u32,
    pub globals: Vec<GlobalEntry>,
}

impl GlobalSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<GlobalSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut globals = Vec::new();

        for _ in 0..count {
            let entry = GlobalEntry::from_reader(reader)?;
            globals.push(entry);
        }

        Ok(GlobalSection {
            count: count as u32,
            globals,
        })
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct GlobalEntry {
    t: GlobalType,
    init: Expression,
}

impl GlobalEntry {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<GlobalEntry, Error> {
        let t = GlobalType::from_reader(reader)?;
        let init = Expression::from_reader(reader)?;

        Ok(GlobalEntry { t, init })
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct GlobalType {
    pub content_type: ValueType,
    pub mutability: u8,
}

impl GlobalType {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<GlobalType, Error> {
        let (content_type_num, _) = reader.leb128_signed()?;
        let content_type = ValueType::from_i64(content_type_num)?;
        let mutability = reader.read_u8()?;

        Ok(GlobalType {
            content_type,
            mutability,
        })
    }
}
