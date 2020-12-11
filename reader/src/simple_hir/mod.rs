#[derive(Debug, PartialEq, Clone)]
pub struct SirProgram {
    pub sirs: Vec<Sir>,
    last_func: SirFunction,
    last_stmt: SirStatement,
    last_expr: SirExpression,
}

impl Default for SirProgram {
    fn default() -> Self {
        SirProgram {
            sirs: vec![],
            last_func: SirFunction::new(),
            last_stmt: SirStatement::None,
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

    pub fn create_stmt(&mut self, name: String) {
        self.last_func.name = name;
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
    pub body: Vec<SirStatement>,
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
pub enum SirStatement {
    Return(String),
    Expression(SirExpression),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SirExpression {
    Builtin {
        name: String,
        parameters: Vec<SirArgument>,
    },
    None,
}
