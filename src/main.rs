#![allow(unused)]
use std::{io, string};

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("{}", fact(6));
}

fn fact(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}
