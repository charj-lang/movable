use crate::location::Location;

#[derive(Debug, PartialEq)]
pub struct TypoGrammar(pub Vec<GrammarUnit>);

#[derive(Debug, PartialEq)]
pub enum GrammarUnit {
    SpecDecl(Box<SpecDecl>),
}

#[derive(Debug, PartialEq)]
pub struct SpecDecl {
    pub location: Location,
}

impl SpecDecl {}
