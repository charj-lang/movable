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
    fn should_parse_empty_options() {
        assert!(parse_program("options {}").is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_parse_string() {
        let parse_ast = parse_program("options{
    name -> \"C Syntax\"
}");
        println!("{:?}", parse_ast);
        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_parse_options_property() {
        let parse_ast = parse_program("options{name -> 'C';}");
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
        let parse_ast = parse_program("options{
    name -> 'C';
    name2 -> 'C';
    extensions -> \".c\";
}");

        println!("{:?}", parse_ast);
        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_parse_define() {
        let parse_ast = parse_program("define default$tokenizer {
    identifier: [a-zA-Z_];
    string: \"[a-zA-Z_]\";
}");

        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_support_local_scope() {
        let parse_ast = parse_program("define default$tokenizer {
  type {
    int -> \"int\";
    string -> \"string\";
  }
}");

        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_support_rule_define() {
        let parse_ast = parse_program("rule import {\

}");

        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_support_use_simple_variables() {
        let parse_ast = parse_program("define default$tokenizer {
  variables -> @identifier*;
}");

        assert!(parse_ast.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_support_basic_bnf_suffix() {
        let normal = parse_program("define default$tokenizer {
  variables: (@identifier)*;
}");

        assert!(normal.is_ok());

        let char = parse_program("define default$tokenizer {
  variables: (\",\" @identifier)?;
}");
        assert!(char.is_ok());

        let char_single_quote = parse_program("define default$tokenizer {
  variables: (',' @identifier)*;
}");
        assert!(char_single_quote.is_ok());
    }

    #[test]
    #[rustfmt::skip]
    fn should_support_use_variables() {
        let parse_ast = parse_program("define default$tokenizer {
  // variables -> @identifier (',' @identifier)?;
}");

        assert!(parse_ast.is_ok());
    }
}
