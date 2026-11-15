use std::io;

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    if !id_str.len().is_multiple_of(2) {
        return false;
    }

    let mid = id_str.len() / 2;
    let (left, right) = id_str.split_at(mid);

    left == right
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let input = input.trim();
    let sum: u64 = input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .map(|range| range.filter(|&id| is_invalid_id(id)).sum::<u64>())
        .sum();

    println!("{sum}");
}
