use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

// 入力処理
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

// 数学関数
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

pub fn pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        1
    } else if exp % 2 == 0 {
        let half = pow(base, exp / 2);
        half * half
    } else {
        base * pow(base, exp - 1)
    }
}

pub fn mod_pow(base: i64, exp: i64, modulo: i64) -> i64 {
    if exp == 0 {
        1
    } else if exp % 2 == 0 {
        let half = mod_pow(base, exp / 2, modulo);
        (half * half) % modulo
    } else {
        (base * mod_pow(base, exp - 1, modulo)) % modulo
    }
}

pub fn abs(x: i64) -> i64 {
    if x < 0 { -x } else { x }
}

// 配列操作
pub fn sum_vec(arr: &[i64]) -> i64 {
    arr.iter().sum()
}

pub fn max_vec(arr: &[i64]) -> i64 {
    *arr.iter().max().unwrap()
}

pub fn min_vec(arr: &[i64]) -> i64 {
    *arr.iter().min().unwrap()
}

pub fn reverse_vec<T>(arr: &mut [T]) {
    arr.reverse()
}

// 素数判定
pub fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for i in (3..=((n as f64).sqrt() as i64)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// エラトステネスの篩
pub fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

// 順列・組み合わせ
pub fn factorial(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub fn permutation(n: i64, r: i64) -> i64 {
    if r > n || r < 0 {
        0
    } else {
        (n - r + 1..=n).product()
    }
}

pub fn combination(n: i64, r: i64) -> i64 {
    if r > n || r < 0 {
        0
    } else if r > n - r {
        combination(n, n - r)
    } else {
        let mut result = 1;
        for i in 0..r {
            result = result * (n - i) / (i + 1);
        }
        result
    }
}

// 文字列操作
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

// ソート
pub fn sort_vec<T: Ord>(arr: &mut [T]) {
    arr.sort();
}

pub fn sort_vec_desc<T: Ord>(arr: &mut [T]) {
    arr.sort_by(|a, b| b.cmp(a));
}

// 二分探索
pub fn binary_search(arr: &[i64], target: i64) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

// 出力処理
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

// 使用例
fn main() {
    // 入力例
    let n = read_int();
    let arr = read_ints();
    
    println!("Input: n={}, arr={:?}", n, arr);
    
    let arr_i64: Vec<i64> = arr.iter().map(|&x| x as i64).collect();
    println!("Max: {}, Min: {}, Sum: {}", max_vec(&arr_i64), min_vec(&arr_i64), sum_vec(&arr_i64));
    println!("GCD of first two: {}", gcd(arr_i64[0], arr_i64[1]));
    println!("Is {} prime? {}", n, is_prime(n as i64));
}