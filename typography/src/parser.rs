use crate::error::Diagnostic;
use crate::lexer;
use crate::parse_tree::TypoGrammar;
use crate::typography;

macro_rules! do_lalr_parsing {
    ($input: expr) => {{
        let lex = lexer::Lexer::new($input);
        match typography::TypographyParser::new().parse($input, lex) {
            Err(err) => Err(Diagnostic::handle_error(err)),
            Ok(s) => Ok(s),
        }
    }};
}

pub fn parse_program(source: &str) -> Result<TypoGrammar, Diagnostic> {
    do_lalr_parsing!(source)
}

#[cfg(test)]
mod test {
    use crate::parser::parse_program;

    #[test]
    #[rustfmt::skip]
    fn should_parse_empty_options() {
        let parse_ast = parse_program("options {}");
        println!("{:?}", parse_ast);
        // assert!(parse_ast.is_ok());
    }
}
