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

fn solve() {
    input! {
        s: String,
    }

    // Calculate the answer
    let conditions1 = s.starts_with("A");
    let conditions2 = s.chars().skip(2).take(s.len() - 2).any(|c| c == 'C');
    let conditions3 = s.chars().skip(s.len() - 1).all(|c| c.is_lowercase());
    let ans = conditions1 && conditions2 && conditions3;

    println!("{}", if ans { "AC" } else { "WA" });
}

// Macro for debugging
#[macro_export]
macro_rules! dbg {
    ($($a:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}
