use std::io;

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(number: usize) -> usize {
    number % 16777216
}

fn secret_number(number: usize, nth: u32) -> usize {
    let mut secret = number;
    for _ in 0..nth {
        secret = prune(mix(secret * 64, secret));
        secret = prune(mix(secret / 32, secret));
        secret = prune(mix(secret * 2048, secret));
    }
    secret
}

fn main() {
    let sum: usize = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|number| number.parse::<usize>().unwrap())
        .map(|number| secret_number(number, 2000))
        .sum();

    println!("{sum}");
}
