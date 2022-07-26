use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(2)
        .for_each(|e| println!("{} ", e[0]));
}
