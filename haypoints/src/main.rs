use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut words = input.split_whitespace();

    let nb = words.next().unwrap().parse::<usize>().unwrap();
    let test_case = words.next().unwrap().parse::<usize>().unwrap();
    let mut score_map: HashMap<&str, i64> = HashMap::new();

    for _ in 0..nb {
        let (key, value) = (
            words.next().unwrap(),
            words.next().unwrap().parse::<i64>().unwrap(),
        );
        score_map.insert(key, value);
    }   
    let vec = words.collect::<Vec<&str>>().join(" ");
    let mut new_word = vec.split(".");
    for _ in 0..test_case {
        let e = new_word.next().unwrap();

        if e == "" {
            return;
        }
        let mut sum = 0;
        e.split_whitespace().for_each(|word| {
            sum += score_map.get(word).unwrap_or(&0);
        });
        println!("{}", sum);
    }
}
