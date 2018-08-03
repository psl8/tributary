#![feature(test)]
extern crate test as rust_test;
//extern crate rayon;

pub mod goal;
#[macro_use]
pub mod macros;
pub mod prelude;
pub mod state;
pub mod stream;
pub mod unify;

#[cfg(test)]
mod test;
