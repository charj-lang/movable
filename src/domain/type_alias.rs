use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeAlias {
    pub int: String,
    pub int8: String,
    pub int16: String,
    pub int32: String,

    pub uint: String,
    pub uint8: String,
    pub uint16: String,
    pub uint32: String,

    pub float32: String,
    pub float64: String,

    pub string: String,
    pub bool: String,
    pub byte: String,
}
