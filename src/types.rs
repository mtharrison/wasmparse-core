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
    Custom(Vec<u8>),
    Types(Box<TypeSection>),
}

#[derive(Debug, PartialEq)]
pub struct TypeSection {
    pub count: u32,
    pub entries: Vec<FunctionType>,
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

#[derive(Debug, PartialEq)]
pub struct FunctionType {
    pub form: ValueType,
    pub param_count: u32,
    pub param_types: Vec<ValueType>,
    pub return_count: u32,
    pub return_type: Option<ValueType>,
}
