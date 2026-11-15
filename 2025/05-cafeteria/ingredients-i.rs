use std::io;

fn main() {
    let mut lines = io::stdin().lines();

    let ranges: Vec<_> = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            let start = start.parse::<u64>().ok()?;
            let end = end.parse::<u64>().ok()?;
            Some(start..=end)
        })
        .collect();

    let count = lines
        .map(|line| line.unwrap())
        .filter(|line| {
            let id = line.parse().unwrap();
            ranges.iter().any(|range| range.contains(&id))
        })
        .count();

    println!("{count}");
}
