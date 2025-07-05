use std::collections::VecDeque;
use std::io::{self};

pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

pub fn read_ints() -> Vec<i64> {
    read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let q: i32 = read_line().parse().unwrap();
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();

    for _ in 0..q {
        let query = read_ints();
        match query[0] {
            1 => {
                let x = query[2];
                let c = query[1];
                queue.push_back((x, c));
            }
            2 => {
                let mut k = query[1];
                let mut total_sum = 0;

                while k > 0 {
                    if let Some(mut front) = queue.pop_front() {
                        let (val, count) = &mut front;

                        if *count <= k {
                            total_sum += *val * *count;
                            k -= *count;
                        } else {
                            total_sum += *val * k;
                            *count -= k;
                            k = 0;
                            queue.push_front(front);
                        }
                    } else {
                        break;
                    }
                }
                println!("{}", total_sum);
            }
            _ => unreachable!(),
        }
    }
}
