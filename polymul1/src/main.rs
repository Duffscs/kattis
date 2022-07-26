use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let nb_cases = lines.next().unwrap().trim().parse::<i64>().unwrap();
    for _ in 0..nb_cases {
        let _ = lines.next();
        let eq1 = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let _ = lines.next();
        let eq2 = lines
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let result = polynomial_multiplication(eq1.clone(), eq2.clone());

        println!("{}", result.len() - 1);
        result.iter().for_each(|e| print!("{} ", e));
        println!();
    }
}

fn polynomial_multiplication(eq1: Vec<i64>, eq2: Vec<i64>) -> Vec<i64> {
    let degree_out = eq1.len() + eq2.len() - 1;
    let mut result = vec![0; degree_out];

    for i in 0..eq1.len() {
        for j in 0..eq2.len() {
            result[i + j] += eq1[i] * eq2[j];
        }
    }

    return result;
}