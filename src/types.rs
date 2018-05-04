#[derive(Debug, Serialize)]
pub struct WasmModule {
    pub version: u32,
    pub sections: Vec<WasmSection>,
}

#[derive(Debug, Serialize)]
pub struct WasmSection {
    pub payload_len: u32,
    pub name: Option<String>,
    pub body: WasmSectionBody,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum WasmSectionBody {
    Custom(Box<CustomSection>),
    Types(Box<TypeSection>),
    Function(Box<FunctionSection>),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct TypeSection {
    pub count: u32,
    pub entries: Vec<FunctionType>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionSection {
    pub count: u32,
    pub types: Vec<u32>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct CustomSection {
    pub len: usize,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ValueType {
    Integer32,
    Integer64,
    Float32,
    Float64,
    Anyfunc,
    Func,
    EmptyBlockType,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct FunctionType {
    pub form: ValueType,
    pub param_count: u32,
    pub param_types: Vec<ValueType>,
    pub return_count: u32,
    pub return_type: Option<ValueType>,
}
