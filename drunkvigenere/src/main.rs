use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");

    let mut words = input.split_whitespace();
    let ciphered_word = words.next().unwrap();
    let key = words.next().unwrap();
    let A = 'A' as u32;
    ciphered_word
        .chars()
        .zip(key.chars())
        .map(|(c1, c2)| (c1 as i32, c2 as i32))
        .enumerate()
        .map(|(i, (c1, c2))| {
            (c1 + c2 * (-1 as i32).pow(((i & 1) + 1) as u32)).rem_euclid(26) as u32 + A
        })
        .for_each(|c| print!("{}", char::from_u32(c).unwrap()));
}
