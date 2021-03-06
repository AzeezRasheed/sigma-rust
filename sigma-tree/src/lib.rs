//! ErgoTree IR

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]

mod ast;
mod constants;
mod data;
mod ecpoint;
mod eval;
mod serialization;
mod types;

pub mod chain;
pub mod ergo_tree;
