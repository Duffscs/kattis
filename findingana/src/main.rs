use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input.chars().skip_while(|&c| c != 'a').for_each(|c| print!("{}", c));    
}