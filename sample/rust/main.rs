
mod utils;

use utils::io::{read_int, read_ints, print_vec};
use utils::array::{max_vec, min_vec, sum_vec};

fn main() {
    // 入力例
    let n = read_int();
    let arr = read_ints();
    
    println!("Input: n={}, arr={:?}", n, arr);
    
    let arr_i64: Vec<i64> = arr.iter().map(|&x| x as i64).collect();
    println!("Max: {}, Min: {}, Sum: {}", max_vec(&arr_i64), min_vec(&arr_i64), sum_vec(&arr_i64));

    // その他の関数の使用例（必要に応じてコメントアウトを解除）
    // use utils::math::{gcd, is_prime};
    // println!("GCD of 12 and 18 is {}", gcd(12, 18));
    // println!("Is 17 prime? {}", is_prime(17));

    // use utils::string::reverse_string;
    // let s = "hello";
    // println!("Reverse of '{}' is '{}'", s, reverse_string(s));
} 