pub mod custom_section;
pub mod function_section;
pub mod import_section;
pub mod memory_section;
pub mod table_section;
pub mod type_section;
pub mod export_section;
pub mod code_section;
pub mod start_section;
pub mod data_section;
pub mod global_section;

pub use custom_section::CustomSection;
pub use function_section::FunctionSection;
pub use import_section::ImportSection;
pub use memory_section::MemorySection;
pub use table_section::TableSection;
pub use type_section::TypeSection;
pub use export_section::ExportSection;
pub use code_section::CodeSection;
pub use start_section::StartSection;
pub use data_section::DataSection;
pub use global_section::GlobalSection;

use std::io::{Error, ErrorKind, Read};
use leb128::ReadLeb128Ext;
use byteorder::ReadBytesExt;

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
    Custom(Box<CustomSection>),
    Function(Box<FunctionSection>),
    Import(Box<ImportSection>),
    Memory(Box<MemorySection>),
    Table(Box<TableSection>),
    Types(Box<TypeSection>),
    Export(Box<ExportSection>),
    Global(Box<GlobalSection>),
    Start(Box<StartSection>),
    Code(Box<CodeSection>),
    Data(Box<DataSection>),
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

#[derive(Debug, PartialEq)]
pub enum ExternalKind {
    Function(u32),
    Table,
    Memory(MemoryType),
    Global,
}

impl ExternalKind {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<ExternalKind, Error> {
        let external_type_code = reader.read_u8()?;

        match external_type_code {
            0 => {
                let (fn_idx, _) = reader.leb128_unsigned()?;
                Ok(ExternalKind::Function(fn_idx as u32))
            }
            1 => unimplemented!("Table imports not implemented"),
            2 => unimplemented!("Memory imports not implemented"),
            3 => unimplemented!("Global imports not implemented"),
            _ => Err(Error::new(ErrorKind::Other, "Unknown External Kind")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MemoryType {
    pub limits: ResizableLimits,
}

#[derive(Debug, PartialEq)]
pub struct Expression(Vec<u8>);

impl Expression {
    pub fn from_reader<T: Read>(reader: &mut T) -> Result<Expression, Error> {

        let mut bytes = Vec::new();
        let mut buff = [0];

        while buff[0] != 0x0b {
            reader.read_exact(&mut buff)?;
            bytes.push(buff[0]);
        }

        Ok(Expression(bytes))
    }
}

