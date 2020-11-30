#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub typography
);

pub mod lexer;
pub mod parser;
