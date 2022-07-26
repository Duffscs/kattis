use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
        .split_whitespace()
        .map(|e| e.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|a|  (a[0] - a[1]).abs())
        .for_each(|e| println!("{}", e));
}
