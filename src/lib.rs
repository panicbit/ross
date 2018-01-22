extern crate lalrpop_util;
extern crate regex;
extern crate itertools;
#[macro_use] extern crate failure;
#[macro_use] extern crate matches;

mod parser;
pub mod ast;
mod trans;
#[cfg(test)]
mod tests;
mod validate;
mod picasso;

pub use parser::parse_Module as parse_module;

fn main() {

}
