use core::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token<'input> {
    Identifier(&'input str),
    OpenCurlyBrace,
    CloseCurlyBrace,
    Typo,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Identifier(id) => write!(f, "{}", id),
            OpenCurlyBrace => write!(f, "{{"),
            CloseCurlyBrace => write!(f, "}}"),
            Typo => write!(f, "typo"),
        }
    }
}
