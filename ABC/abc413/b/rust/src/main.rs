use std::collections::HashSet;
use std::io;

pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

pub fn read_int() -> i32 {
    read_line().parse().unwrap()
}

fn main() {
    let n = read_int() as usize;
    let s: Vec<String> = (0..n).map(|_| read_line()).collect();

    let count = (0..n)
        .flat_map(|i| {
            let s_ref = &s;
            (0..n)
                .filter(move |&j| i != j)
                .map(move |j| s_ref[i].clone() + &s_ref[j])
        })
        .collect::<HashSet<_>>()
        .len();

    println!("{}", count);
}
