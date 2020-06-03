//! C language parser and abstract syntax tree
//!
//! ```
//! use lang_cl::driver::{Config, parse};
//!
//! fn main() {
//!     let config = Config::default();
//!     println!("{:?}", parse(&config, "example.c"));
//! }
//! ```

#![allow(deprecated)]

pub mod ast;
pub mod driver;
pub mod span;
pub mod visit;

mod astutil;
mod env;
mod parser;
mod strings;

#[cfg(test)]
mod tests;
