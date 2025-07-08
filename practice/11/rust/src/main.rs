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
        n: usize,
        y: usize,
    }

    let result = (0..=n)
        .flat_map(|i| (0..=n.saturating_sub(i)).map(move |j| (i, j)))
        .find(|(i, j)| {
            let k = n - i - j;
            10000 * i + 5000 * j + 1000 * k == y
        });

    if let Some((i, j)) = result {
        let k = n - i - j;
        println!("{} {} {}", i, j, k);
    } else {
        println!("-1 -1 -1");
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
