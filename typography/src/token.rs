use core::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),
    OpenCurlyBrace,
    CloseCurlyBrace,
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
            Identifier(id) => write!(f, "{}", id),
            OpenCurlyBrace => write!(f, "{{"),
            CloseCurlyBrace => write!(f, "}}"),
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
