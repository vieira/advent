use std::collections::HashMap;
use std::io;

type Cache = HashMap<(u64, u64), u64>;

fn count(pebble: u64, blinks: u64, cache: &mut Cache) -> u64 {
    if blinks == 0 {
        return 1;
    }

    let next_blink = blinks - 1;
    let key = (pebble, blinks);

    if let Some(c) = cache.get(&key) {
        return *c;
    }

    let num = if pebble == 0 {
        count(1, next_blink, cache)
    } else {
        let digits = pebble.to_string();
        match digits.len() % 2 {
            0 => {
                let middle = digits.len() / 2;
                let left = digits[0..middle].parse().unwrap();
                let right = digits[middle..].parse().unwrap();
                count(left, next_blink, cache) + count(right, next_blink, cache)
            }
            _ => count(pebble * 2024, next_blink, cache),
        }
    };

    cache.insert(key, num);
    num
}

fn main() {
    let pebbles: Vec<u64> = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();

    let mut cache = HashMap::new();
    let count: u64 = pebbles.iter().map(|p| count(*p, 75, &mut cache)).sum();
    println!("{count}");
}
