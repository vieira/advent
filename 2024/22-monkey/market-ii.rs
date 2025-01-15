use std::collections::{HashMap, HashSet};
use std::io;

type Counts = HashMap<Vec<i8>, u32>;

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(number: usize) -> usize {
    number % 16777216
}

fn prices(number: usize, nth: u32) -> Vec<i8> {
    let mut ps = vec![(number % 10) as i8];
    let mut secret = number;
    for _ in 0..nth {
        secret = prune(mix(secret * 64, secret));
        secret = prune(mix(secret / 32, secret));
        secret = prune(mix(secret * 2048, secret));
        ps.push((secret % 10) as i8);
    }
    ps
}

fn deltas(ps: &[i8]) -> Vec<i8> {
    ps.iter().zip(&ps[1..]).map(|(a, b)| b - a).collect()
}

fn sequences(buyer: &[i8], mut seqs: Counts) -> Counts {
    let ds = deltas(buyer);
    let mut seen = HashSet::new();
    for i in 0..ds.len() - 4 {
        let seq = &ds[i..i + 4];
        if seen.insert(seq.to_owned()) {
            *seqs.entry(seq.to_owned()).or_insert(0) += buyer[i + 4] as u32;
        }
    }

    seqs
}

fn main() {
    let mut seqs = HashMap::new();
    let buyers: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|number| number.parse::<usize>().unwrap())
        .map(|secret| prices(secret, 2000))
        .collect();

    for buyer in buyers {
        seqs = sequences(&buyer, seqs);
    }

    let (_, bananas) = seqs.iter().max_by_key(|(_, v)| **v).unwrap();
    println!("{bananas}");
}
