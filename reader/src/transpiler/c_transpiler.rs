use std::path::PathBuf;

use crate::read_scie_data;
use crate::simple_hir::{Sir, SirExpression, SirProgram};
use crate::transpiler::Transpiler;

pub struct CTranspiler {}

impl Transpiler for CTranspiler {
    fn transpile(path: &mut PathBuf) -> SirProgram {
        let mut sir_program = SirProgram::new("main".to_string());
        let vec = read_scie_data(&*path);
        for token in &vec[0].elements {
            let last_token = token.scopes[token.scopes.len() - 1].as_str();
            let mut next_to_last = "";
            if token.scopes.len() > 2 {
                next_to_last = token.scopes[token.scopes.len() - 2].as_str();
            }

            match last_token {
                "string.quoted.other.lt-gt.include.c" => {
                    sir_program.add_sir(Sir::Import(token.value.to_string()));
                }
                "entity.name.function.c" => match next_to_last {
                    "meta.function.definition.parameters.c" => {
                        sir_program.create_function(token.value.to_string());
                    }
                    "meta.function-call.c" => {
                        sir_program.create_expr(SirExpression::Call {
                            name: token.value.to_string(),
                            args: None,
                        });
                    }
                    _ => {}
                },
                "string.quoted.double.c" => {
                    match next_to_last {
                        "meta.function-call.c" => {
                            sir_program.create_call_argument(token.value.to_string());
                        }
                        _ => {}
                    }
                    println!("string: {:?}", token.value);
                }
                "punctuation.terminator.statement.c" => {
                    sir_program.end_expr();
                }
                "keyword.control.c" => {
                    match token.value.as_str() {
                        "return" => {}
                        _ => {}
                    };
                }
                "punctuation.section.block.end.bracket.curly.c" => {
                    sir_program.done_function();
                }
                _ => {}
            }
        }
        sir_program
    }
}
