use leb128::ReadLeb128Ext;
use std::io::{Error, Read};

#[derive(Debug, PartialEq, Serialize)]
pub struct StartSection {
    pub index: u32,
}

impl StartSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<StartSection, Error> {
        let (index, _) = reader.leb128_unsigned()?;
        Ok(StartSection {
            index: index as u32,
        })
    }
}
