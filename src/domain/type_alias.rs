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

impl TypeAlias {
    pub fn new() -> Self {
        TypeAlias {
            int: "int".to_string(),
            int8: "int8".to_string(),
            int16: "int16".to_string(),
            int32: "int32".to_string(),
            uint: "uint".to_string(),
            uint8: "uint8".to_string(),
            uint16: "uint16".to_string(),
            uint32: "uint32".to_string(),
            float32: "float32".to_string(),
            float64: "float64".to_string(),
            string: "string".to_string(),
            bool: "bool".to_string(),
            byte: "byte".to_string(),
        }
    }
}

impl TypeAlias {}
