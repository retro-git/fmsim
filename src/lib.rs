#![feature(lazy_cell)]
#![feature(type_changing_struct_update)]
#![feature(test)]

extern crate test;

pub mod data;
pub mod duel;

pub use data::*;
pub use duel::Duel;
