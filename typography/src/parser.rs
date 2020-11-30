use crate::lexer;

macro_rules! do_lalr_parsing {
    ($input: expr) => {{
        let lex = lexer::Lexer::new($input);
        match charj::CharjParser::new().parse($input, lex) {
            Err(err) => Err(Diagnostic::handle_error(err)),
            Ok(s) => Ok(s),
        }
    }};
}
