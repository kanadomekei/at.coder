use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_int() -> i32 {
    read_line().parse().unwrap()
}

fn read_ints() -> Vec<i32> {
    read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_2d_ints(n: usize) -> Vec<Vec<i32>> {
    (0..n).map(|_| read_ints()).collect()
}

fn main() {
    let n = read_int();
    let data = read_2d_ints(n as usize);
    let count = data.iter().filter(|values| values[1] > values[0]).count();
    println!("{}", count);
}
