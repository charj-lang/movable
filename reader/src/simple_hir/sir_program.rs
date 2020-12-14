use crate::simple_hir::{Sir, SirExpression, SirFunction, SirInstruction};
use core::fmt;
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct SirProgram {
    pub name: String,
    pub sirs: Vec<Sir>,
    last_func: SirFunction,
    last_stmt: SirInstruction,
    last_expr: SirExpression,
}

impl SirProgram {
    pub fn new(name: String) -> Self {
        SirProgram {
            name,
            sirs: vec![],
            last_func: SirFunction::new(),
            last_stmt: SirInstruction::None,
            last_expr: SirExpression::None,
        }
    }

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

    /// convert sir code to mir string in beautify way
    pub fn codify(&self) {
        //
    }

    /// `module` -> `module:`
    fn module_name(&self) -> String {
        format!("m_{}{}", self.name, ":")
    }

    /// `module` -> `module:`
    fn colon(&self, name: &String) -> String {
        format!("{}{}", name, ":")
    }

    /// `module` -> `module:`
    fn p_func(&self, name: &String) -> String {
        // self.sirs
    }
}

impl Display for SirProgram {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let width = 10;
        // <module_name>: module
        let module_name = self.module_name();
        writeln!(f, "{:w$} {}", module_name, "module", w = width);
        // <space> export sieve
        writeln!(f, "{:w$} export {}", "", self.name, w = width);

        // func
        writeln!(
            f,
            "{:w$} {}",
            self.colon(&self.name),
            self.p_func(),
            w = width
        );

        // end module
        writeln!(f, "{:w$} {}", "", "endmodule", w = width)
    }
}

#[cfg(test)]
mod tests {
    use crate::simple_hir::SirProgram;

    #[test]
    fn should_println() {
        let program = SirProgram::new(String::from("println"));
        println!("{}", program);
    }
}
