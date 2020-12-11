#[derive(Debug, PartialEq, Clone)]
pub struct SirProgram {
    pub sirs: Vec<Sir>,
}

impl Default for SirProgram {
    fn default() -> Self {
        SirProgram { sirs: vec![] }
    }
}

impl SirProgram {
    pub fn add_sir(&mut self, sir: Sir) {
        self.sirs.push(sir);
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
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Builtin {
        name: String,
        parameters: Vec<SirArgument>,
    },
}
