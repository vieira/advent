use std::io;
use std::collections::HashSet;

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split_whitespace().map(|n| n.parse().unwrap()).collect::<HashSet<u32>>()
}

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (_, all_numbers) = line.split_once(':').unwrap();
    let (winning, card) = all_numbers.split_once('|').unwrap();
    (parse_numbers(winning), parse_numbers(card))
}

fn main() {
    let sum: u32 = io::stdin().lines()
        .map(|line| {
            let line = line.unwrap();
            let (winning, card) = parse_line(&line);
            let matches = winning.intersection(&card).count() as u32;

            if matches > 0 { 2_u32.pow(matches - 1) } else { 0 }
        })
        .sum();

        println!("{sum}");
}
