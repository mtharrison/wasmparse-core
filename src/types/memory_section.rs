use leb128::ReadLeb128Ext;
use std::io::{Error, Read};

use super::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct MemorySection {
    pub count: u32,
    pub entries: Vec<MemoryType>,
}

impl MemorySection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<MemorySection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut entries = Vec::new();

        for _ in 0..count {
            let limits = ResizableLimits::from_reader(reader)?;
            entries.push(MemoryType { limits });
        }

        Ok(MemorySection {
            count: count as u32,
            entries,
        })
    }
}
