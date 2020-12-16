pub use c_transpiler::*;

use crate::simple_hir::SirProgram;
use crate::CodeFile;

pub mod c_transpiler;

pub trait Transpiler {
    fn transpile(&self, code_files: Vec<CodeFile>) -> SirProgram;
}

pub fn transpiler_dispatcher(lang: &str) -> Box<dyn Transpiler> {
    match lang {
        "c" => Box::from(CTranspiler {}),
        _ => Box::from(CTranspiler {}),
    }
}
