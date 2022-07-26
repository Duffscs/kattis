use std::{
    collections::{HashMap, HashSet},
    io::{self, Read}, time::Instant,
};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Lecture de stdin");
    let mut word = input.split_whitespace();
    let n = word.next().unwrap().parse::<usize>().unwrap();
    let mut max = 0;
    let set: HashSet<usize> = word
        .clone()
        .map(|e| {
            let i = e.parse::<usize>().unwrap();
            if i > max {
                max = i;
            }
            return i;
        })
        .collect();

    let mut prev_ai = 1;
    let mut prev_si = 1;
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut sod_vec: Vec<i64> = Vec::new();
    sod_vec.push(0);
    sod_vec.push(1);
    for i in 2..=max {
        sod_vec.push(sod(prev_ai));
        prev_ai = (prev_ai + sod_vec.last().unwrap()) % ((10 as i64).pow(9) + 7);
        prev_si = (prev_ai + prev_si) % ((10 as i64).pow(9) + 7);
        if set.contains(&i) {
            map.insert(i, prev_si as usize);
        }
    }

    for _ in 0..n {
        let i = &word.next().unwrap().parse::<usize>().unwrap();
        print!("{} ", map.get(i).unwrap());
    }
}

fn sod(num: i64) -> i64 {
    // let mut sum: i64 = 0;
    // if num > 0 {
    //     sum += num % 10;
    //     sum += sod(num / 10);
    // }
    return DigitIter(num as usize).sum::<usize>() as i64;
}

struct DigitIter(usize);

impl Iterator for DigitIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let ret = self.0 % 10;
            self.0 /= 10;
            Some(ret)
        }
    }
}
