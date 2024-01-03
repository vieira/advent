use std::io;
use std::collections::HashMap;
use std::cmp::Ordering;

type Hand = Vec<u8>;
type Counts = Vec<u8>;
type HandCounts = (Hand, Counts);

fn parse_line(line: &str) -> (Hand, u32) {
    let (cards, bid) = line.split_once(' ').unwrap();
    let bid: u32 = bid.parse().unwrap();
    let cards = cards.chars().map(|c| {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as u8,
        }
    }).collect::<Hand>();

    (cards, bid)
}

fn count_cards(hand: &Hand) -> Counts {
    let mut cards_count = HashMap::new();

    for card in hand {
        let count = cards_count.get(card).unwrap_or(&0);
        cards_count.insert(card, count + 1);
    }

    let mut counts = cards_count.values().copied().collect::<Vec<u8>>();
    counts.sort();
    counts
}

fn hand_type(counts: &Counts) -> u8 {
    match counts[..] {
        [5] => 6,
        [1, 4] => 5,
        [2, 3] => 4,
        [1, 1, 3] => 3,
        [1, 2, 2] => 2,
        [1, 1, 1, 2] => 1,
        _ => 0,
    }
}

fn compare_hands((h1, c1): &HandCounts, (h2, c2): &HandCounts) -> Ordering {
    let h1t = hand_type(c1);
    let h2t = hand_type(c2);

    if h1t != h2t { return h1t.cmp(&h2t) }

    for i in 0..h1.len() {
        if h1[i] != h2[i] { return h1[i].cmp(&h2[i]) }
    }

    Ordering::Equal
}

fn main() {
    let mut hands = vec![];

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (hand, bid) = parse_line(&line);
        let counts = count_cards(&hand);
        hands.push(((hand, counts), bid));
    }

    hands.sort_by(|(h, _), (j, _)| compare_hands(h, j));

    let winnings: u32 = hands.iter()
        .enumerate()
        .map(|(r, (_, bid))| (r as u32 + 1) * *bid)
        .sum();

    println!("{winnings}");
}
