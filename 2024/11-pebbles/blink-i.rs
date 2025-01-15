use std::io;

fn count(pebble: u64, blinks: u64) -> u64 {
    if blinks == 0 {
        return 1;
    }

    let next_blink = blinks - 1;
    if pebble == 0 {
        return count(1, next_blink);
    }

    let p = pebble.to_string();
    if p.len() % 2 != 0 {
        return count(pebble * 2024, next_blink);
    }

    let middle = p.len() / 2;
    let left = p[0..middle].parse().unwrap();
    let right = p[middle..].parse().unwrap();
    count(left, next_blink) + count(right, next_blink)
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

    let count: u64 = pebbles.iter().map(|p| count(*p, 25)).sum();
    println!("{count}");
}
