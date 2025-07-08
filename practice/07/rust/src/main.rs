#![recursion_limit = "256"]
use itertools::Itertools;
use proconio::input;
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
        n: usize,
        a: [i64; n],
    }

    // Calculate the answer
    let alice = a.iter().sorted().rev().step_by(2).sum::<i64>();
    let bob = a.iter().sorted().rev().skip(1).step_by(2).sum::<i64>();

    // dbg!(&a.iter().sorted().rev().step_by(2), &a[1..].iter().sorted().rev().step_by(2));
    println!("{}", alice - bob);
}

// Macro for debugging
#[macro_export]
macro_rules! dbg {
    ($($a:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}
