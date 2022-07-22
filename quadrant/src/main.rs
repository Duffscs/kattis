use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");

    let mut words = input.split_whitespace();
    let x = words.next().expect("x").parse::<i32>().expect("Entier");
    let y = words.next().expect("y").parse::<i32>().expect("Entier");
    let mut quadrant = 1;

    if x < 0 {
        quadrant = quadrant + 1;
    }
    if y < 0 {
        quadrant = quadrant + 1;
        if x > 0 {
            quadrant = quadrant + 2;
        }
    }
    println!("{}", quadrant);
}
