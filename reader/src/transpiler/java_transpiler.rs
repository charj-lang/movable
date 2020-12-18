use crate::simple_hir::SirProgram;
use crate::transpiler::Transpiler;
use crate::CodeFile;

pub struct JavaTranspiler {}

impl Transpiler for JavaTranspiler {
    fn transpile(&self, _code_files: Vec<CodeFile>) -> SirProgram {
        let mut sir_program = SirProgram::new("main".to_string());
        sir_program
    }
}
