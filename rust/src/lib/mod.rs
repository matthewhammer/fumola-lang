#[macro_use]
extern crate lalrpop_util;

pub mod error;
pub mod ast;

lalrpop_mod!(pub parser); // syntesized by LALRPOP
