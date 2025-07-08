#![recursion_limit = "256"]
use core::iter::Iterator;

use proconio::input;
// use itertools::Itertools;
// use std::cmp::{max, min};
// use std::collections::{HashMap, HashSet, VecDeque};

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
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    // Calculate the answer
    let ans = (0..=a)
        .flat_map(|i| (0..=b).flat_map(move |j| (0..=c).map(move |k| i * 500 + j * 100 + k * 50)))
        .filter(|&l| l == x)
        .count();

    println!("{}", ans);
}

// Macro for debugging
#[macro_export]
macro_rules! dbg {
    ($($a:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}
