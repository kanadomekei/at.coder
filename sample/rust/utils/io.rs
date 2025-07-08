
use std::io::{self, BufRead};

pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

pub fn read_int() -> i32 {
    read_line().parse().unwrap()
}

pub fn read_ints() -> Vec<i32> {
    read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn read_long() -> i64 {
    read_line().parse().unwrap()
}

pub fn read_longs() -> Vec<i64> {
    read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn read_string() -> String {
    read_line()
}

pub fn read_strings() -> Vec<String> {
    read_line()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub fn read_chars() -> Vec<char> {
    read_line().chars().collect()
}

pub fn read_2d_ints(n: usize) -> Vec<Vec<i32>> {
    (0..n).map(|_| read_ints()).collect()
}

pub fn read_2d_strings(n: usize) -> Vec<Vec<String>> {
    (0..n).map(|_| read_strings()).collect()
}

pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
    for (i, item) in arr.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", item);
    }
    println!();
}

pub fn print_vec_lines<T: std::fmt::Display>(arr: &[T]) {
    for item in arr {
        println!("{}", item);
    }
} 