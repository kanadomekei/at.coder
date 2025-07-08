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

    let cond1 = s.starts_with('A');

    let cond2 = if s.len() >= 3 {
        s.chars()
            .skip(2)
            .take(s.len() - 3)
            .filter(|&c| c == 'C')
            .count()
            == 1
    } else {
        false
    };

    let cond3 = s.chars().filter(|c| c.is_uppercase()).count() == 2;

    if cond1 && cond2 && cond3 {
        println!("AC");
    } else {
        println!("WA");
    }
}

// Macro for debugging
#[macro_export]
macro_rules! dbg {
    ($($a:expr),* $(,)?) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}
