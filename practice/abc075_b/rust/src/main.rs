#![recursion_limit = "256"]
use proconio::input;
use proconio::marker::Chars;
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
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    // Calculate the answer
    let mut result = vec![vec!['0'; w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                result[i][j] = '#';
                continue;
            }

            let mut count = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
                        continue;
                    }

                    if s[ni as usize][nj as usize] == '#' {
                        count += 1;
                    }
                }
            }
            result[i][j] = std::char::from_digit(count, 10).unwrap();
        }
    }

    for row in result {
        println!("{}", row.into_iter().collect::<String>());
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
