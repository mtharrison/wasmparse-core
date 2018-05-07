use std::io::{Error, Read};

#[derive(Debug, PartialEq)]
pub struct CustomSection {
    pub len: usize,
    pub data: Vec<u8>,
}

impl CustomSection {
    pub fn from_reader<T: Read>(reader: &mut T, len: usize) -> Result<CustomSection, Error> {
        let mut data = vec![0; len];
        reader.read_exact(&mut data)?;
        Ok(CustomSection { len, data })
    }
}