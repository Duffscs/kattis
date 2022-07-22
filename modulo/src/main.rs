use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let hash = input
        .split_whitespace()
        .map(|e| e.parse::<i64>().expect("i64") % 42)
        .collect::<HashSet<i64>>();
    println!("{}", hash.len());
}
