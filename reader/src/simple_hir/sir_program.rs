use crate::simple_hir::{Sir, SirExpression, SirFunction, SirInstruction, SirParameter, SirStruct};
use core::fmt;
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct SirProgram {
    pub name: String,
    pub sirs: Vec<Sir>,
    last_struct: SirStruct,
    last_func: SirFunction,
    last_stmt: SirInstruction,
    last_expr: SirExpression,
}

impl SirProgram {
    pub fn new(name: String) -> Self {
        SirProgram {
            name,
            sirs: vec![],
            last_struct: Default::default(),
            last_func: Default::default(),
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

    pub fn create_class(&mut self, name: String) {
        self.last_struct.name = name;
    }

    pub fn end_class(&mut self) {
        self.sirs.push(Sir::Struct(self.last_struct.clone()));
        self.last_struct.name = "".to_string();
    }

    pub fn end_function(&mut self) {
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
    #[allow(dead_code)]
    fn colon(&self, name: &String) -> String {
        format!("{}{}", name, ":")
    }

    fn signature(&self, returns: &Vec<String>, params: &Vec<SirParameter>) -> String {
        if params.len() == 0 {
            return format!("{}{}", returns.join(""), "");
        }
        let parameters = String::from("");
        for param in params {
            println!("todo: add param {:?}", param);
        }

        format!("{}{}", returns.join(""), parameters)
    }

    /// `module` -> `module:`
    #[allow(unused_must_use)]
    fn write_func(&self, f: &mut Formatter, width: usize) {
        for sir in &self.sirs {
            match sir {
                Sir::Import(_) => {}
                Sir::Function(func) => {
                    let signature = self.signature(&func.returns, &func.params);
                    writeln!(
                        f,
                        "{:w$} {} {}",
                        self.colon(&func.name),
                        "func",
                        signature,
                        w = width
                    );

                    writeln!(f, "{:w$} {}", "", "endfunc", w = width);
                }
                Sir::Struct(stt) => {}
            }
        }
    }
}

impl Display for SirProgram {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let width: usize = 10;
        // <module_name>: module
        let module_name = self.module_name();
        writeln!(f, "{:w$} {}", module_name, "module", w = width);
        // <space> export sieve
        writeln!(f, "{:w$} export {}", "", self.name, w = width);

        self.write_func(f, width);

        // end module
        writeln!(f, "{:w$} {}", "", "endmodule", w = width)
    }
}

#[cfg(test)]
mod tests {
    use crate::simple_hir::{Sir, SirFunction, SirProgram};

    #[test]
    fn should_build_empty_app_modules() {
        let program = SirProgram::new(String::from("app"));
        assert_eq!(
            "m_app:     module
           export app
           endmodule
",
            format!("{}", program)
        );
    }

    #[test]
    fn should_build_main() {
        let mut program = SirProgram::new(String::from("app"));
        let mut function = SirFunction::new(String::from("main"));
        function.returns.push(String::from("int"));

        program.sirs.push(Sir::Function(function));
        println!("{}", program);
        // assert_eq!("", format!("{}", program));
    }
}
