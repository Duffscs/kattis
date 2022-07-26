use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();
    lines.take(n).for_each(|e| {
        if e.starts_with("simon says ") {
            println!("{}", e.replace("simon says ", ""));
        } else {
            println!();
        }
    });
}
