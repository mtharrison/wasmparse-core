use std::io::{Error, Read};
use leb128::ReadLeb128Ext;

use super::*;

#[derive(Debug, PartialEq)]
pub struct ElementSection {
    pub count: u32,
    pub entries: Vec<ElementSegment>,
}

impl ElementSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<ElementSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut entries = Vec::new();

        for _ in 0..count {
            let entry = ElementSegment::from_reader(reader)?;
            entries.push(entry);
        }

        Ok(ElementSection {
            count: count as u32,
            entries,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ElementSegment {
    pub index: u32,
    pub offset: Expression,
    pub num_elem: u32,
    pub elems: Vec<u32>,
}

impl ElementSegment {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<ElementSegment, Error> {
        let (index, _) = reader.leb128_unsigned()?;
        let offset = Expression::from_reader(reader)?;
        let (num_elem, _) = reader.leb128_unsigned()?;

        let mut elems = Vec::with_capacity(num_elem as usize);

        for _ in 0..num_elem {
            let (index, _) = reader.leb128_unsigned()?;
            elems.push(index as u32);
        }

        Ok(ElementSegment {
            index: index as u32,
            offset,
            num_elem: num_elem as u32,
            elems
        })
    }
}