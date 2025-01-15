use std::io;
use std::collections::HashMap;

type Counts = HashMap<String, usize>;

fn arrangements(pattern: &str, stripes: &[&str], counts: &mut Counts) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(count) = counts.get(pattern) {
        return *count;
    }

    let count = stripes
        .iter()
        .filter(|s| pattern.starts_with(*s))
        .map(|s| arrangements(&pattern[s.len()..], stripes, counts))
        .sum();
    counts.insert(pattern.to_string(), count);

    count
}

fn main() {
    let mut counts: Counts = HashMap::new();
    let mut lines = io::stdin().lines();
    let line = lines.next().unwrap().unwrap();
    let stripes: Vec<_> = line.split(", ").collect();
    lines.next();

    let count: usize = lines
        .map(|line| line.unwrap())
        .map(|pattern| arrangements(&pattern, &stripes, &mut counts))
        .sum();

    println!("{count}");
}
