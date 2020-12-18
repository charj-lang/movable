pub use c_transpiler::*;

use crate::simple_hir::SirProgram;
use crate::transpiler::java_transpiler::JavaTranspiler;
use crate::CodeFile;

pub mod c_transpiler;
pub mod java_transpiler;

pub trait Transpiler {
    fn transpile(&self, code_files: Vec<CodeFile>) -> SirProgram;
}

pub fn transpiler_dispatcher(lang: &str) -> Box<dyn Transpiler> {
    match lang {
        "c" => Box::from(CTranspiler {}),
        "java" => Box::from(JavaTranspiler {}),
        _ => Box::from(CTranspiler {}),
    }
}
