use alloc::Vec;

use opcode::Opcode;
use bincode;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Module {
    pub types: Vec<Type>,
    pub functions: Vec<Function>,
    pub data_segments: Vec<DataSegment>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Type {
    Func(Vec<ValType>, Vec<ValType>) // (args...) -> (ret)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Function {
    pub typeidx: usize,
    pub locals: Vec<ValType>,
    pub body: FunctionBody
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionBody {
    pub opcodes: Vec<Opcode>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataSegment {
    pub offset: u32,
    pub data: Vec<u8>
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64
}

impl Module {
    pub fn std_serialize(&self) -> Option<Vec<u8>> {
        match bincode::serialize(self) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }

    pub fn std_deserialize(data: &[u8]) -> Option<Module> {
        match bincode::deserialize(data) {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }
}