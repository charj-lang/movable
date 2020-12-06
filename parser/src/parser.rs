use crate::error::Diagnostic;
use crate::lexer;
use crate::movable;
use crate::parse_tree::SourceUnit;

macro_rules! do_lalr_parsing {
    ($input: expr) => {{
        let lex = lexer::Lexer::new($input);
        match movable::MovableParser::new().parse($input, lex) {
            Err(err) => Err(Diagnostic::handle_error(err)),
            Ok(s) => Ok(s),
        }
    }};
}

pub fn parse_program(source: &str) -> Result<SourceUnit, Diagnostic> {
    do_lalr_parsing!(source)
}

#[cfg(test)]
mod test {
    use crate::parser::parse_program;

    #[test]
    #[rustfmt::skip]
    fn test_parse_empty() {
        let parse_ast = parse_program("typo {

}");

        assert!(parse_ast.is_ok());
    }
}
