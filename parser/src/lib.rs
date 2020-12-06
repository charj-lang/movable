#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub movable
);

pub mod error;
pub mod lexer;
pub mod location;
pub mod parse_tree;
pub mod parser;
pub mod token;
