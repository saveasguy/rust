// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags:-Znll -Zborrowck=mir

#![allow(warnings)]
#![feature(dyn_trait)]

use std::fmt::Debug;

fn no_region<'a, T>(x: Box<T>) -> Box<Debug + 'a>
where
    T: Debug,
{
    x
    //~^ WARNING not reporting region error due to -Znll
    //~| ERROR failed type test
}

fn correct_region<'a, T>(x: Box<T>) -> Box<Debug + 'a>
where
    T: 'a + Debug,
{
    x
}

fn wrong_region<'a, 'b, T>(x: Box<T>) -> Box<Debug + 'a>
where
    T: 'b + Debug,
{
    x
    //~^ WARNING not reporting region error due to -Znll
    //~| ERROR failed type test
}

fn outlives_region<'a, 'b, T>(x: Box<T>) -> Box<Debug + 'a>
where
    T: 'b + Debug,
    'b: 'a,
{
    x
}

fn main() {}
