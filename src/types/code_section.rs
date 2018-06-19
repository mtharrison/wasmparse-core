use leb128::ReadLeb128Ext;
use std::io::{Error, Read};

use super::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct CodeSection {
    pub count: u32,
    pub bodies: Vec<FunctionBody>,
}

impl CodeSection {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<CodeSection, Error> {
        let (count, _) = reader.leb128_unsigned()?;

        let mut bodies = Vec::new();

        for _ in 0..count {
            let body = FunctionBody::from_reader(reader)?;
            bodies.push(body);
        }

        Ok(CodeSection {
            count: count as u32,
            bodies,
        })
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionBody {
    pub body_size: u32,
    pub local_count: u32,
    pub locals: Vec<LocalEntry>,
    pub code: Vec<u8>,
}

impl FunctionBody {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<FunctionBody, Error> {
        let (mut body_size, _) = reader.leb128_unsigned()?;
        let (local_count, local_count_bytes) = reader.leb128_unsigned()?;

        body_size -= local_count_bytes as i64;

        let mut locals = Vec::with_capacity(local_count as usize);

        for _ in 0..local_count {
            let (local, bytes_read) = LocalEntry::from_reader(reader)?;
            locals.push(local);
            body_size -= bytes_read as i64;
        }

        let mut code = vec![0; body_size as usize];
        reader.read_exact(&mut code)?;

        Ok(FunctionBody {
            body_size: body_size as u32,
            local_count: local_count as u32,
            locals,
            code,
        })
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct LocalEntry {
    pub count: u32,
    pub t: ValueType,
}

impl LocalEntry {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<(LocalEntry, usize), Error> {
        let mut read = 0;
        let (count, read1) = reader.leb128_unsigned()?;
        let (num, read2) = reader.leb128_signed()?;
        let t = ValueType::from_i64(num)?;

        read += read1;
        read += read2;

        Ok((
            LocalEntry {
                count: count as u32,
                t,
            },
            read,
        ))
    }
}
