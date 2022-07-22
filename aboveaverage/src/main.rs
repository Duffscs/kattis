use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");

    input.lines().skip(1).collect::<Vec<&str>>().iter().for_each(|e| {
        let list = e.split_ascii_whitespace().skip(1);
        let nb_elem = list.clone().count();
        let sum :i64 = list.clone().map(|e| e.parse::<i64>().expect("parse")).sum();
        let avg = sum as f64/ nb_elem as f64;
        let above_avg = list.clone().filter(|e| e.parse::<f64>().expect("parse") > avg).count();
        println!("{:.3}%", above_avg as f64/ nb_elem as f64 * 100.0);
    });
}
