use crate::simple_hir::{Sir, SirExpression, SirFunction, SirInstruction};

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
