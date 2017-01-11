use std::io;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);

    let mut lines = buffer.lines().collect::<Vec<_>>();

    lines.sort_by(|&a, &b| clean_line(a).cmp(&clean_line(b)));

    for line in lines {
        println!("{}", line);
    }
}

fn clean_line(line: &str) -> String {
    line.replace("* as", "")
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_digit(10))
        .map(|c| c.to_lowercase().collect::<String>())
        .collect()
}
