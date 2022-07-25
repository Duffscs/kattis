use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    loop {
        let max = words.next().unwrap().parse::<u64>().unwrap();
        if max == 0 {
            break;
        }
        println!("{}", phi(max));
    }
}

fn phi(mut n: u64) -> u64 {
    let mut result: u64 = n;
    let mut i: u64 = 2;
    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
        i += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    return result;
}
