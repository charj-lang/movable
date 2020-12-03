#[macro_use]
extern crate lalrpop_util;

extern crate string_cache;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub typography
);

pub mod error;
pub mod lexer;
pub mod location;
pub mod parse_tree;
pub mod parser;
pub mod token;

#[cfg(test)]
mod test {
    use crate::parser::parse_program;

    #[test]
    #[rustfmt::skip]
    fn test_parse_empty() {
        let parse_ast = parse_program("spec {

}");
        println!("{:?}", parse_ast);
        assert!(parse_ast.is_ok());
    }
}
