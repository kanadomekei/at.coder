use std::collections::HashSet;
use std::io;

pub fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let s = read_line();
    let t = read_line();

    let s_chars: Vec<char> = s.chars().collect();
    let t_set: HashSet<char> = t.chars().collect();

    let violation_found = s_chars.windows(2).any(|window| {
        let prev_char = window[0];
        let curr_char = window[1];
        curr_char.is_uppercase() && !t_set.contains(&prev_char)
    });

    if violation_found {
        println!("No");
    } else {
        println!("Yes");
    }
}
