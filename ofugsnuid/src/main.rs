use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut rev = input.lines().skip(1).collect::<Vec<&str>>();
    rev.reverse();
    println!("{}", rev.join("\n"));
}