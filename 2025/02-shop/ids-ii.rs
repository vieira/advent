use std::io;

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    if len < 2 {
        return false;
    }
    let chars = id_str.as_bytes();
    let mid = len / 2;

    (1..=mid).filter(|&idx| len.is_multiple_of(idx)).any(|idx| {
        let part = &chars[..idx];
        chars.chunks(idx).all(|chunk| part == chunk)
    })
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
