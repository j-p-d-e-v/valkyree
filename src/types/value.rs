use crate::types::SimpleErrorKind;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Array(Vec<Value>),
    String(String),
    Error(SimpleErrorKind, String),
    Boolean(bool),
    Double(f64),
    Integer(i64),
    BigNumber(BigInt),
    Null,
    Infinity,
    NegativeInfinity,
    Nan,
}
impl Value {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_, _))
    }
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }
    pub fn is_big_number(&self) -> bool {
        matches!(self, Self::BigNumber(_))
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    pub fn is_infinity(&self) -> bool {
        matches!(self, Self::Infinity)
    }
    pub fn is_negative_infinity(&self) -> bool {
        matches!(self, Self::NegativeInfinity)
    }
    pub fn is_nan(&self) -> bool {
        matches!(self, Self::Nan)
    }
}
