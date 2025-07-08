#![recursion_limit = "256"]
use proconio::{input, marker::Chars};
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

    let mut result = vec![vec![' '; w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                result[i][j] = '#';
                continue;
            }

            let mut bomb_count = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }

                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if ni >= 0 && ni < h as i32 && nj >= 0 && nj < w as i32 {
                        if s[ni as usize][nj as usize] == '#' {
                            bomb_count += 1;
                        }
                    }
                }
            }
            result[i][j] = std::char::from_digit(bomb_count, 10).unwrap();
        }
    }

    for row in result {
        println!("{}", row.iter().collect::<String>());
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
