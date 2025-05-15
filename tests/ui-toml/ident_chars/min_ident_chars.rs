//@aux-build:extern_types.rs
#![allow(nonstandard_style, unused)]
#![warn(clippy::ident_chars)]

extern crate extern_types;
use extern_types::{Aaa, LONGER, M, N as W};
//~^ ident_chars

pub const N: u32 = 0;
//~^ ident_chars
pub const LONG: u32 = 32;

struct Owo {
    Uwu: u128,
    aaa: Aaa,
    //~^ ident_chars
}

fn main() {
    let wha = 1;
    let vvv = 1;
    //~^ ident_chars
    let uuu = 1;
    //~^ ident_chars
    let (mut a, mut b) = (1, 2);
    //~^ ident_chars
    //~| ident_chars
    for i in 0..1000 {}
    //~^ ident_chars
}
