
use std::io::{self, BufRead};

// 標準入力から1行読み取り、前後の空白を削除して返す
pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

// 標準入力から1つの整数(i32)を読み取る
pub fn read_int() -> i32 {
    read_line().parse().unwrap()
}

// 標準入力から空白区切りの整数(i32)を読み取ってベクタで返す
pub fn read_ints() -> Vec<i32> {
    read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// 標準入力から1つの整数(i64)を読み取る
pub fn read_long() -> i64 {
    read_line().parse().unwrap()
}

// 標準入力から空白区切りの整数(i64)を読み取ってベクタで返す
pub fn read_longs() -> Vec<i64> {
    read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// 標準入力から文字列を読み取る
pub fn read_string() -> String {
    read_line()
}

// 標準入力から空白区切りの文字列を読み取ってベクタで返す
pub fn read_strings() -> Vec<String> {
    read_line()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

// 標準入力から文字列を読み取り、文字のベクタで返す
pub fn read_chars() -> Vec<char> {
    read_line().chars().collect()
}

// n行の整数(i32)を2次元ベクタで読み取る
pub fn read_2d_ints(n: usize) -> Vec<Vec<i32>> {
    (0..n).map(|_| read_ints()).collect()
}

// n行の文字列を2次元ベクタで読み取る
pub fn read_2d_strings(n: usize) -> Vec<Vec<String>> {
    (0..n).map(|_| read_strings()).collect()
}

// ベクタの要素を空白区切りで1行に出力する
pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
    for (i, item) in arr.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", item);
    }
    println!();
}

// ベクタの各要素を改行区切りで出力する
pub fn print_vec_lines<T: std::fmt::Display>(arr: &[T]) {
    for item in arr {
        println!("{}", item);
    }
} 