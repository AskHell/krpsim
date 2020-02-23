pub mod parser;
pub mod ast;
pub mod inventory;
pub mod genetic;
pub mod solver;
pub mod check;
pub mod genetic_config_parser;
pub mod genetic_plot;
pub mod utils;

#[cfg(test)]
mod check_tests;

#[cfg(test)]
#[macro_use] extern crate maplit;
