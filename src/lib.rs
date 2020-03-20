pub mod ast;
pub mod check;
pub mod dna;
pub mod genetic;
pub mod genetic_config_parser;
pub mod genetic_plot;
pub mod inventory;
pub mod parser;
pub mod score;
pub mod simulate;
pub mod solver;
pub mod utils;

#[cfg(test)]
mod check_tests;

#[cfg(test)]
mod solver_test;

#[cfg(test)]
#[macro_use]
extern crate maplit;
