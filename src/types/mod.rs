pub mod custom_section;
pub mod function_section;
pub mod import_section;
pub mod table_section;
pub mod type_section;

use std::io::{Error, ErrorKind, Read};
use leb128::ReadLeb128Ext;

#[derive(Debug)]
pub struct WasmModule {
    pub version: u32,
    pub sections: Vec<WasmSection>,
}

#[derive(Debug)]
pub struct WasmSection {
    pub payload_len: u32,
    pub name: Option<String>,
    pub body: WasmSectionBody,
}

#[derive(Debug, PartialEq)]
pub enum WasmSectionBody {
    Custom(Box<custom_section::CustomSection>),
    Function(Box<function_section::FunctionSection>),
    Import(Box<import_section::ImportSection>),
    Table(Box<table_section::TableSection>),
    Types(Box<type_section::TypeSection>),
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    Integer32,
    Integer64,
    Float32,
    Float64,
    Anyfunc,
    Func,
    EmptyBlockType,
}

impl ValueType {
    pub fn from_i64(num: i64) -> Result<ValueType, Error> {
        match num {
            -0x01 => Ok(ValueType::Integer32),
            -0x02 => Ok(ValueType::Integer64),
            -0x03 => Ok(ValueType::Float32),
            -0x04 => Ok(ValueType::Float64),
            -0x10 => Ok(ValueType::Anyfunc),
            -0x20 => Ok(ValueType::Func),
            -0x40 => Ok(ValueType::EmptyBlockType),
            _ => Err(Error::new(ErrorKind::Other, "Unknown Value Type")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionType {
    pub form: ValueType,
    pub param_count: u32,
    pub param_types: Vec<ValueType>,
    pub return_count: u32,
    pub return_type: Option<ValueType>,
}

#[derive(Debug, PartialEq)]
pub struct ResizableLimits {
    flags: u8,
    initial: u32,
    maximum: Option<u32>,
}

impl ResizableLimits {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<ResizableLimits, Error> {
        let (flags, _) = reader.leb128_unsigned()?;
        let (initial, _) = reader.leb128_unsigned()?;
        let mut maximum = None;
        if flags == 1 {
            let (max, _) = reader.leb128_unsigned()?;
            maximum = Some(max as u32);
        }

        Ok(ResizableLimits {
            flags: flags as u8,
            initial: initial as u32,
            maximum,
        })
    }
}
