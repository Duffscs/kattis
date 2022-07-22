use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let lines = input.lines();
    let mut max = 0;
    let mut nb_max = 0;
    let mut index = 1;
    lines.for_each(|line| {
        let mut somme = 0;
        line.split_whitespace()
            .map(|e| e.parse::<i32>().expect("Entier"))
            .for_each(|e| {
                somme += e;
            });
        if somme > max {
            max = somme;
            nb_max = index;
        }
        index += 1;
    });
    println!("{} {}", nb_max, max);
}
