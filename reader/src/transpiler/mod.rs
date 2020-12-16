use crate::simple_hir::SirProgram;
use std::path::PathBuf;

pub use c_transpiler::*;

pub mod c_transpiler;

trait Transpiler {
    fn transpile(path: &mut PathBuf) -> SirProgram;
}
