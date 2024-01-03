use std::io;
use std::collections::HashSet;
use std::collections::HashMap;

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split_whitespace().map(|n| n.parse().unwrap()).collect::<HashSet<u32>>()
}

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (_, all_numbers) = line.split_once(':').unwrap();
    let (winning, card) = all_numbers.split_once('|').unwrap();
    (parse_numbers(winning), parse_numbers(card))
}

fn main() {
    let mut cards_count = HashMap::new();

    for (i, line) in io::stdin().lines().enumerate() {
        let id = i + 1;
        let copies = 1 + *cards_count.get(&id).unwrap_or(&0);
        cards_count.insert(id, copies);

        let line = line.unwrap();
        let (winning, card) = parse_line(&line);
        let matches = winning.intersection(&card).count();

        for j in id + 1..=id + matches {
            let count = copies + cards_count.get(&j).unwrap_or(&0);
            cards_count.insert(j, count);
        }
    }

    let sum = cards_count.values().sum::<u32>();
    println!("{sum}");
}
