use crate::location::Location;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub struct TypoGrammar(pub Vec<GrammarUnit>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GrammarUnit {
    SpecDecl(Box<SpecDecl>),
    OptionsDecl(Box<OptionsDecl>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecDecl {
    pub location: Location,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionsDecl {
    pub location: Location,
    // pub properties: Vec<PropertyDecl>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    PropertyDecl,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    StringLiteral { value: StringLiteral },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StringLiteral {
    pub loc: Location,
    pub string: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyDecl {
    pub location: Location,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub loc: Location,
    pub name: String,
}

impl SpecDecl {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub name: Option<String>,
    pub extensions: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tokenizer {}
