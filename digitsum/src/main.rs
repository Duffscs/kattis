use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let mut map: HashMap<String, i64> = HashMap::new();
    stdin().lock().lines().skip(1).for_each(|e| {
        let str = e.unwrap();
        let mut words = str
            .split_whitespace()
            .map(|e| e.parse::<u64>().expect("u64"));
        let i = words.next().unwrap();
        let end = words.next().unwrap();
        let sum : i64 = (i..end+1)
            .map(|e| format!("{}", e))
            .map(|e| {
                if !map.contains_key(&e) {
                    map.insert(
                        e.clone(),
                        e.clone().chars().fold(0, |acc, e| acc + e.to_digit(10).unwrap()) as i64,
                    );
                }
                return *map.get_mut(&e.clone()).unwrap();
            })
            .sum();
        println!("{}", sum);
    });
}
