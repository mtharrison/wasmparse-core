pub mod custom_section;
pub mod function_section;
pub mod import_section;
pub mod table_section;
pub mod type_section;

use std::io::{Error, ErrorKind};

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
