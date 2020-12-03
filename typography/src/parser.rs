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
    use crate::parse_tree::GrammarUnit;
    use crate::parser::parse_program;

    #[test]
    #[rustfmt::skip]
    fn should_parse_empty_options() {
        let parse_ast = parse_program("options {}");
        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_parse_options_property() {
        let parse_ast = parse_program("options{ name -> 'C' ; }");
        let mut name_equal_name = false ;
        let grammar_units = parse_ast.unwrap().0;
        if let GrammarUnit::OptionsDecl(decl) = grammar_units.get(0).unwrap() {
            for prop in &decl.properties {
                if prop.name.to_string() == "name" {
                    name_equal_name = true;
                }
            }
        }
        assert!(name_equal_name);
    }

    #[test]
    #[rustfmt::skip]
    fn should_parse_multiple_options() {
        let parse_ast = parse_program("options{ \
    name -> 'C';
    name2 -> 'C';
//    extensions -> \"c\";
}");
        println!("{:?}", parse_ast);
        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_parse_define() {
        let parse_ast = parse_program("define default$tokenizer {
    identifier: [a-zA-Z_];
}");
        println!("{:?}", parse_ast);
        assert!(parse_ast.is_ok());
    }
}
