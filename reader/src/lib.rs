#[macro_use]
extern crate strum_macros;

use crate::simple_hir::{Sir, SirExpression, SirProgram};
pub use scie_code_file::CodeFile;
pub use scie_token_element::TokenElement;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub mod domain;
pub mod retoken;
pub mod scie_code_file;
pub mod scie_token_element;
pub mod simple_hir;

pub fn read_scie_data(path: &Path) -> Vec<CodeFile> {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).expect("error file");
}

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

#[cfg(test)]
mod tests {
    use crate::transpile;
    use std::path::PathBuf;

    #[test]
    fn should_build_first_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../_fixtures/c/hello.c.json");

        let sir_program = transpile(&mut path);

        println!("{:?}", sir_program);
    }
}
