use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub struct SirProgram {
    pub sirs: Vec<Sir>,
    last_func: SirFunction,
    last_stmt: SirInstruction,
    last_expr: SirExpression,
}

impl Default for SirProgram {
    fn default() -> Self {
        SirProgram {
            sirs: vec![],
            last_func: SirFunction::new(),
            last_stmt: SirInstruction::None,
            last_expr: SirExpression::None,
        }
    }
}

impl SirProgram {
    pub fn add_sir(&mut self, sir: Sir) {
        self.sirs.push(sir);
    }

    pub fn create_function(&mut self, name: String) {
        self.last_func.name = name;
    }

    pub fn done_function(&mut self) {
        self.sirs.push(Sir::Function(self.last_func.clone()));
        self.last_func.name = "".to_string();
    }

    pub fn create_call_argument(&mut self, arg: String) {
        match &self.last_expr {
            SirExpression::Call { name, args: _ } => {
                self.last_expr = SirExpression::Call {
                    name: name.clone(),
                    args: Some(arg),
                }
            }
            SirExpression::None => {}
        };
    }

    pub fn create_expr(&mut self, expr: SirExpression) {
        self.last_expr = expr;
    }

    pub fn end_expr(&mut self) {
        match &self.last_expr {
            SirExpression::Call { .. } => {
                self.last_func
                    .body
                    .push(SirInstruction::Expression(self.last_expr.clone()));
            }
            SirExpression::None => {}
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Sir {
    Import(String),
    Function(SirFunction),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SirFunction {
    pub name: String,
    pub params: Vec<SirParameter>,
    pub returns: Vec<SirParameter>,
    pub body: Vec<SirInstruction>,
}

impl SirFunction {
    pub fn new() -> Self {
        SirFunction {
            name: "".to_string(),
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
