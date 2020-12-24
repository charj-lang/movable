use crate::simple_hir::SirProgram;
use crate::transpiler::Transpiler;
use crate::CodeFile;

pub struct JavaTranspiler {}

impl Transpiler for JavaTranspiler {
    fn transpile(&self, code_files: Vec<CodeFile>) -> SirProgram {
        let mut sir_program = SirProgram::new("main".to_string());
        for token in &code_files[0].elements {
            let last_token = token.scopes[token.scopes.len() - 1].as_str();
            let mut next_to_last = "";
            if token.scopes.len() > 2 {
                next_to_last = token.scopes[token.scopes.len() - 2].as_str();
            }

            match last_token {
                "entity.name.type.class.java" => {
                    sir_program.create_class(token.value.to_string());
                }
                "entity.name.function.java" => {
                    sir_program.create_function(token.value.to_string());
                }
                "punctuation.section.method.end.bracket.curly.java" => {
                    sir_program.end_function();
                }
                "variable.other.object.java" => {
                    // todo: make proc name
                }
                "variable.other.object.property.java" => {
                    // todo: make proc name
                }
                "punctuation.section.class.end.bracket.curly.java" => {
                    sir_program.end_class();
                }
                _ => {
                    // println!("{:?}", last_token);
                }
            }
        }
        sir_program
    }
}
