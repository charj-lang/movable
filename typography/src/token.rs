use core::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),
    Semicolon,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenParenthesis,
    CloseParenthesis,
    Assign,
    ArrowAssign,
    Arrow,
    Question,
    Colon,
    Add,
    Subtract,
    Mul,
    Divide,
    Modulo,
    Not,
    OpenBracket,
    CloseBracket,
    Member,
    Comma,
    Complement,
    More,
    Less,

    Options,
    Spec,
    Typo,
    Default,
    Tokenizer,
    Node,
    Ast,
    Template,
    Impl,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::OpenParenthesis => write!(f, "("),
            Token::CloseParenthesis => write!(f, ")"),
            Token::OpenCurlyBrace => write!(f, "{{"),
            Token::CloseCurlyBrace => write!(f, "}}"),
            Token::Add => write!(f, "+"),
            Token::Subtract => write!(f, "-"),
            Token::Mul => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::Assign => write!(f, "="),
            Token::Not => write!(f, "!"),
            Token::More => write!(f, ">"),
            Token::Member => write!(f, "."),
            Token::Colon => write!(f, ":"),
            Token::OpenBracket => write!(f, "["),
            Token::CloseBracket => write!(f, "]"),
            Token::Complement => write!(f, "~"),
            Token::Question => write!(f, "?"),
            Token::Less => write!(f, "<"),

            Identifier(id) => write!(f, "{}", id),
            Options => write!(f, "options"),
            Spec => write!(f, "spec"),
            Typo => write!(f, "typo"),
            Default => write!(f, "default"),
            Tokenizer => write!(f, "tokenizer"),
            Node => write!(f, "node"),
            Ast => write!(f, "ast"),
            Template => write!(f, "template"),
            Impl => write!(f, "impl"),
        }
    }
}
