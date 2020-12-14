use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub enum Sir {
    Import(String),
    Function(SirFunction),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SirFunction {
    pub name: String,
    pub params: Vec<SirParameter>,
    pub returns: Vec<String>,
    pub body: Vec<SirInstruction>,
}

impl Default for SirFunction {
    fn default() -> Self {
        SirFunction {
            name: "".to_string(),
            params: vec![],
            returns: vec![],
            body: vec![],
        }
    }
}

impl SirFunction {
    pub fn new(name: String) -> Self {
        SirFunction {
            name,
            params: vec![],
            returns: vec![],
            body: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SirParameter {
    pub ty: String,
    pub name: String,
    pub is_array: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SirArgument {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SirInstruction {
    Expression(SirExpression),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SirExpression {
    Call { name: String, args: Option<String> },
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CallType {
    Positional(usize),
    Keyword(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SirInstr {
    Import {
        name: Option<String>,
        symbols: Vec<String>,
        level: usize,
    },
    CallFunction {
        typ: CallType,
    },
    ReturnValue,
}
