use std::io::{Error, Read};
use leb128::ReadLeb128Ext;

#[derive(Debug, PartialEq)]
pub struct FunctionSection {
    pub count: u32,
    pub types: Vec<u32>,
}

impl FunctionSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<FunctionSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut types = Vec::new();

        for _ in 0..count {
            let (index, _) = reader.leb128_unsigned()?;
            types.push(index as u32);
        }

        Ok(FunctionSection { count: count as u32, types })
    }
}