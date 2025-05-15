//@aux-build:proc_macros.rs
#![allow(irrefutable_let_patterns, nonstandard_style, unused)]
#![allow(clippy::struct_field_names)]
#![warn(clippy::ident_chars)]

extern crate proc_macros;
use proc_macros::{external, with_span};

struct A {
    //~^ ident_chars
    a: u32,
    //~^ ident_chars
    i: u32,
    A: u32,
    //~^ ident_chars
    I: u32,
    //~^ ident_chars
}

struct B(u32);
//~^ ident_chars

struct O {
    //~^ ident_chars
    o: u32,
    //~^ ident_chars
}

struct i;

enum C {
    //~^ ident_chars
    D,
    //~^ ident_chars
    E,
    //~^ ident_chars
    F,
    //~^ ident_chars
    j,
}

struct Vec4 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

struct AA<T, E>(T, E);

trait Trait {
    const A: u32 = 0;
    //~^ ident_chars
    type A;
    //~^ ident_chars
    fn a() {}
    //~^ ident_chars
}

fn main() {
    // Allowed idents
    let w = 1;
    // Ok, not this one
    // let i = 1;
    let j = 1;
    let n = 1;
    let z = 1;
    let y = 1;
    let z = 1;
    // Implicitly disallowed idents
    let h = 1;
    //~^ ident_chars
    let e = 2;
    //~^ ident_chars
    let l = 3;
    //~^ ident_chars
    let l = 4;
    //~^ ident_chars
    let o = 6;
    //~^ ident_chars
    // 2 len does not lint
    let hi = 0;
    // Lint
    let (h, o, w) = (1, 2, 3);
    //~^ ident_chars
    //~| ident_chars
    for (a, (r, e)) in (0..1000).enumerate().enumerate() {}
    //~^ ident_chars
    //~| ident_chars
    //~| ident_chars
    let you = Vec4 { x: 1, y: 2, z: 3, w: 4 };
    while let (d, o, _i, n, g) = (true, true, false, false, true) {}
    //~^ ident_chars
    //~| ident_chars
    //~| ident_chars
    let today = true;
    // Ideally this wouldn't lint, but this would (likely) require global analysis, outta scope
    // of this lint regardless
    let o = 1;
    //~^ ident_chars
    let o = O { o };
    //~^ ident_chars

    for j in 0..1000 {}
    for _ in 0..10 {}

    // Do not lint code from external macros
    external! { for j in 0..1000 {} }
    // Do not lint code from procedural macros
    with_span! {
        span
        for j in 0..1000 {}
    }
}

fn b() {}
//~^ ident_chars
fn wrong_pythagoras(a: f32, b: f32) -> f32 {
    //~^ ident_chars
    //~| ident_chars
    a * a + a * b
}

mod issue_11163 {
    struct Array<T, const N: usize>([T; N]);
}

struct Issue13396;

impl core::fmt::Display for Issue13396 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Issue13396")
    }
}

impl core::fmt::Debug for Issue13396 {
    fn fmt(&self, g: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        //~^ ident_chars
        write!(g, "Issue13396")
    }
}

fn issue13396() {
    let a = |f: i8| f;
    //~^ ident_chars
    //~| ident_chars
}

trait D {
    //~^ ident_chars
    fn f(g: i32);
    //~^ ident_chars
    //~| ident_chars
    fn long(long: i32);

    fn g(arg: i8) {
        //~^ ident_chars
        fn c(d: u8) {}
        //~^ ident_chars
        //~| ident_chars
    }
}

impl D for Issue13396 {
    fn f(g: i32) {
        fn h() {}
        //~^ ident_chars
        fn inner(a: i32) {}
        //~^ ident_chars
        let a = |f: String| f;
        //~^ ident_chars
        //~| ident_chars
    }
    fn long(long: i32) {}
}
