extern crate itertools;
#[macro_use] extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate failure;
#[macro_use] extern crate matches;

mod parser;
pub mod ast;
mod trans;
#[cfg(test)]
mod tests;
mod validate;
mod picasso;

pub use parser::parse_module;

fn main() {

}
