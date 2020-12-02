use crate::location::Location;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct TypoGrammar(pub Vec<GrammarUnit>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GrammarUnit {
    SpecDecl(Box<SpecDecl>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecDecl {
    pub location: Location,
}

impl SpecDecl {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub name: Option<String>,
    pub extensions: Vec<String>
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tokenizer {

}
