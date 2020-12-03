use crate::location::Location;
use serde::{Deserialize, Serialize};
use string_cache::DefaultAtom as Atom;

#[derive(Debug, PartialEq)]
pub struct TypoGrammar(pub Vec<GrammarUnit>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GrammarUnit {
    SpecDecl(Box<SpecDecl>),
    OptionsDecl(Box<OptionsDecl>),
    NamespaceDecl(Box<NamespaceDecl>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpecDecl {
    pub location: Location,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionsDecl {
    pub location: Location,
    pub properties: Vec<Box<PropertyDecl>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    PropertyDecl,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyDecl {
    pub location: Location,
    pub name: Atom,
    pub pattern: PatternKind,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceDecl {
    pub location: Location,
    pub scope: Atom,
    pub name: Atom,
    pub properties: Vec<Box<PropertyDecl>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalScopeDecl {
    pub scope: Atom,
    pub properties: Vec<Box<PropertyDecl>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PatternKind {
    CharLiteral(Atom),
    StringLiteral(Atom),
    Pattern(String),
    LocalScope(LocalScopeDecl),
}

impl SpecDecl {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Options {
    pub name: Option<String>,
    pub extensions: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tokenizer {}
