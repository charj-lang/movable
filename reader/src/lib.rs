#[macro_use]
extern crate strum_macros;

pub use scie_code_file::CodeFile;
pub use scie_token_element::TokenElement;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

#[cfg(test)]
mod tests {
    use crate::read_scie_data;
    use crate::simple_hir::{Sir, SirProgram};
    use std::path::PathBuf;

    #[test]
    fn should_build_first_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../_fixtures/c/hello.c.json");

        let mut sir_program = SirProgram::default();
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
                        println!("function: {:?}", token.value);
                    }
                    "meta.function-call.c" => {
                        println!("callee: {:?}", token.value);
                    }
                    _ => {}
                },
                "string.quoted.double.c" => {
                    println!("string: {:?}", token.value);
                }
                _ => {}
            }
        }

        println!("{:?}", sir_program);
    }
}
