use std::io::{Error, Read};
use leb128::ReadLeb128Ext;

use super::*;

#[derive(Debug, PartialEq)]
pub struct TableSection {
    pub count: u32,
    pub entries: Vec<TableType>,
}

impl TableSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<TableSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut entries = Vec::new();

        for _ in 0..count {
            let (element_type_num, _) = reader.leb128_signed()?;
            let element_type = ElementType::from_i64(element_type_num)?;

            let limits = ResizableLimits::from_reader(reader)?;

            let entry = TableType {
                element_type,
                limits,
            };

            entries.push(entry);
        }

        Ok(TableSection {
            count: count as u32,
            entries,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct TableType {
    pub element_type: ElementType,
    pub limits: ResizableLimits,
}

#[derive(Debug, PartialEq)]
pub enum ElementType {
    Anyfunc,
}

impl ElementType {
    pub fn from_i64(num: i64) -> Result<ElementType, Error> {
        match num {
            -0x10 => Ok(ElementType::Anyfunc),
            _ => Err(Error::new(ErrorKind::Other, "Unknown Element Type")),
        }
    }
}
