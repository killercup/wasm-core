use executor::{ExecuteResult, ExecuteError};
use fp_ops;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    Undef,
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64)
}

impl Default for Value {
    fn default() -> Value {
        Value::Undef
    }
}

impl Value {
    pub fn get_i32(&self) -> ExecuteResult<i32> {
        match *self {
            Value::Undef => Ok(0),
            Value::I32(v) => Ok(v),
            _ => {
                //panic!();
                Err(ExecuteError::ValueTypeMismatch)
            }
        }
    }

    pub fn get_i64(&self) -> ExecuteResult<i64> {
        match *self {
            Value::Undef => Ok(0),
            Value::I64(v) => Ok(v),
            _ => {
                //panic!();
                Err(ExecuteError::ValueTypeMismatch)
            }
        }
    }

    pub fn cast_to_i64(&self) -> i64 {
        match *self {
            Value::Undef => 0,
            Value::I32(v) => v as i64,
            Value::I64(v) => v,
            Value::F32(v) => fp_ops::f32_convert_i64_s(v).unwrap_or(0),
            Value::F64(v) => fp_ops::f64_convert_i64_s(v).unwrap_or(0)
        }
    }

    pub fn reinterpret_as_i64(&self) -> i64 {
        unsafe {
            match *self {
                Value::Undef => 0,
                Value::I32(v) => ::prelude::mem::transmute(v as u32 as u64),
                Value::I64(v) => v,
                Value::F32(v) => ::prelude::mem::transmute(v as f64),
                Value::F64(v) => ::prelude::mem::transmute(v)
            }
        }
    }
}
