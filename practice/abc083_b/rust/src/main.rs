#![recursion_limit = "256"]
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

fn digit_sum(mut n: i64) -> i64 {
    n = n.abs();
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn solve() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    // Calculate the answer
    let ans = (1..=n)
        .filter(|&x| a as i64 <= digit_sum(x as i64) && digit_sum(x as i64) <= b as i64)
        .sum::<usize>();

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
