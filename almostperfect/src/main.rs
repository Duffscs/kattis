use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    input.lines().for_each(|line| {
        let nb = line.parse::<i64>().unwrap();
        let sum: i64 = get_dividers(nb).iter().sum();
        if nb == sum {
            println!("{} perfect", nb);
        } else if nb - 2 <= sum && sum <= nb + 2 {
            println!("{} almost perfect", nb);
        } else {
            println!("{} not perfect", nb);
        }
    });
}

fn get_dividers(nb: i64) -> HashSet<i64> {
    let mut dividers = HashSet::new();
    dividers.insert(1);
    let mut i = 2;
    while i * i <= nb {
        if nb % i == 0 {
            dividers.insert(i);
            dividers.insert(nb / i);
        }
        i += 1;
    }
    dividers
}
