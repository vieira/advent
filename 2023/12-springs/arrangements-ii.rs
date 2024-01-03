use std::io;
use std::collections::HashMap;

type Springs = Vec<char>;
type Cache = HashMap<String, usize>;

fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_line(line: &str) -> (Springs, Vec<usize>) {
    let (springs, counts) = line.split_once(' ').unwrap();
    let counts = parse_numbers(counts);

    let mut all_springs = vec!['.'];
    let mut all_counts = vec![];

    for _ in 0..5 {
        for n in &counts { all_counts.push(*n) }
        for c in springs.chars() { all_springs.push(c) }
        all_springs.push('?');
    }

    all_springs.pop();
    all_springs.push('.');

    (all_springs, all_counts)
}

fn key(s: &Springs, counts: &[usize]) -> String {
    let k = s.iter().collect::<String>();
    format!("{k}:{:?}", counts)
}

fn can_fit(fragment: &[char], space: usize) -> bool {
    if fragment[0] == '#' || fragment[space - 1] == '#' {
        return false;
    }

    fragment[1..space - 1].iter().all(|&c| c != '.')
}

fn combinations(s: &Springs, size: usize) -> Vec<Springs> {
    let mut suffixes = vec![];
    let space = size + 2;

    let mut i = 0;

    while i + space <= s.len() {
        let fragment = &s[i..i + space];

        if can_fit(fragment, space) {
            let mut suffix = vec!['.'];
            suffix.extend_from_slice(&s[i + space..]);
            suffixes.push(suffix);
        }

        let first = fragment.iter().enumerate().find(|(_, &c)| c == '#');
        if first.is_some_and(|(idx, _)| idx <= 1) { break }

        i += 1;
    }

    suffixes
}

fn count_combinations(s: &Springs, sizes: &[usize], cache: &mut Cache) -> usize {
    let k = key(s, sizes);

    if let Some(v) = cache.get(&k) {
        return *v;
    }

    let size = sizes[0];
    if sizes.len() == 1 {
        return combinations(s, size)
            .iter()
            .filter(|s| s.iter().all(|&c| c != '#'))
            .count();
    }

    let mut sum = 0;
    for c in combinations(s, size) {
        sum += count_combinations(&c, &sizes[1..], cache);
    }

    cache.insert(k, sum);

    sum
}

fn main() {
    let mut cache: HashMap<String, usize> = HashMap::new();
    let sum: usize = io::stdin()
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .map(|(springs, counts)| {
            count_combinations(&springs, &counts, &mut cache)
        })
        .sum();

    println!("{sum}");
}
