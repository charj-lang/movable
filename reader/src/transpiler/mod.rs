pub use c_transpiler::*;

use crate::simple_hir::SirProgram;
use crate::CodeFile;

pub mod c_transpiler;

pub trait Transpiler {
    fn transpile(self, code_files: Vec<CodeFile>) -> SirProgram;
}
