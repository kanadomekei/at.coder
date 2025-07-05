use std::io;

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

fn main() {
    let data = read_ints();
    let _n = data[0];
    let m = data[1];

    let a = read_ints();

    let sum_a: i32 = a.iter().sum();

    if sum_a <= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
