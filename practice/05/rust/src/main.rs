#![recursion_limit = "256"]
use core::iter::Sum;

use proconio::input;
// use itertools::Itertools;
// use std::cmp::{max, min};
// use std::collections::{HashMap, HashSet, VecDeque};

fn digit_sum(mut n: i64) -> i64 {
    n = n.abs();
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}
fn main() {
    // For recursive functions, prevent stack overflow.
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}

fn solve() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }

    let sum = (1..=n)
        .filter(|x| a <= digit_sum(*x as i64) && digit_sum(*x as i64) <= b)
        .sum::<i64>();

    println!("{}", sum);
}

// Macro for debugging
#[macro_export]
macro_rules! dbg {
    ($($a:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}
