use std::io;

type Matrix = Vec<Vec<u64>>;

fn calculate(matrix: &Matrix, ops: &[char]) -> u64 {
    matrix
        .iter()
        .zip(ops.iter())
        .map(|(nums, op)| match op {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => 0,
        })
        .sum()
}

fn main() {
    let mut matrix: Matrix = vec![];
    let mut ops = vec![];

    for (i, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();
        if !line.split_whitespace().any(|el| el.parse::<u64>().is_ok()) {
            ops = line
                .split_whitespace()
                .map(|op| op.chars().next().unwrap())
                .collect();
            break;
        }

        let els = line.split_whitespace();

        if i == 0 {
            matrix = els.map(|el| vec![el.parse().unwrap()]).collect();
            continue;
        }

        els.enumerate().for_each(|(i, num)| {
            matrix[i].push(num.parse().unwrap());
        });
    }

    println!("{}", calculate(&matrix, &ops));
}
