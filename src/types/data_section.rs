use std::io::{Error, Read};
use leb128::ReadLeb128Ext;

use super::*;

#[derive(Debug, PartialEq)]
pub struct DataSection {
    pub count: u32,
    pub entries: Vec<DataSegment>,
}

impl DataSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<DataSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut entries = Vec::new();

        for _ in 0..count {
            let entry = DataSegment::from_reader(reader)?;
            entries.push(entry);
        }

        Ok(DataSection {
            count: count as u32,
            entries,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct DataSegment {
    pub index: u32,
    pub offset: Expression,
    pub size: u32,
    pub data: Vec<u8>,
}

impl DataSegment {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<DataSegment, Error> {
        let (index, _) = reader.leb128_unsigned()?;

        let offset = Expression::from_reader(reader)?;

        let (size, _) = reader.leb128_unsigned()?;

        let mut data = vec![0; size as usize];
        reader.read_exact(&mut data)?;

        Ok(DataSegment {
            index: index as u32,
            offset,
            size: size as u32,
            data
        })
    }
}