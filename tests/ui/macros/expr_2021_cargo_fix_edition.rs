//@ run-rustfix
//@ check-pass
//@ compile-flags: --edition=2021
#![allow(incomplete_features)]
#![feature(expr_fragment_specifier_2024)]
#![warn(edition_2024_expr_fragment_specifier)]

macro_rules! m {
    ($e:expr) => { //~ WARN: the `expr` fragment specifier will accept more expressions in the 2024 edition
       //~^ WARN: this changes meaning in Rust 2024
        $e
    };
    ($($i:expr)*) => { }; //~ WARN: the `expr` fragment specifier will accept more expressions in the 2024 edition
       //~^ WARN: this changes meaning in Rust 2024
}

macro_rules! test {
    (expr) => {}
}

fn main() {
    m!(());
    test!(expr);
}
