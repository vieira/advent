use std::io;

fn parse_numbers(s: &str) -> Vec<u32> {
    s.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_line(line: &str) -> (Vec<char>, Vec<u32>) {
    let (springs, counts) = line.split_once(' ').unwrap();
    let counts = parse_numbers(counts);

    (springs.chars().collect(), counts)
}

fn contiguous_groups(springs: &[char]) -> Vec<u32> {
    let mut current_count = 0;
    let mut counts = vec![];

    for s in springs {
        match s {
            '#' => { current_count += 1 },
            '?' => {
                current_count = 0;
                break;
            }
            _ => {
                if current_count == 0 { continue };
                counts.push(current_count);
                current_count = 0;
            }
        }
    }

    if current_count > 0 {
        counts.push(current_count);
    }

    counts
}

fn is_valid(s: &[char], counts: &[u32]) -> bool {
    let actual_counts = contiguous_groups(s);
    if actual_counts.len() > counts.len() { return false };

    counts.iter().zip(actual_counts.iter()).all(|(a, b)| { a == b })
}

fn successors(s: &[char], counts: &[u32]) -> Vec<Vec<char>> {
    let mut ss = vec![];

    let Some((i, _)) = s.iter().enumerate().find(|(_, &c)| c == '?')
    else { return ss };

    for c in ['.', '#'] {
        let mut ns = s.to_owned();
        ns[i] = c;

        if !is_valid(&ns, counts) { continue };

        ss.push(ns);
    }

    ss
}

fn is_goal(s: &[char], counts: &[u32]) -> bool {
    if s.iter().any(|c| *c == '?') { return false };

    let actual_counts = contiguous_groups(s);
    if actual_counts.len() != counts.len() { return false };

    counts.iter().zip(actual_counts.iter()).all(|(a, b)| { a == b })
}

fn count_combinations(springs: &[char], counts: &[u32]) -> u32 {
    let mut num_combinations = 0;
    let mut states = vec![springs.to_owned()];

    while let Some(s) = states.pop() {
        if is_goal(&s, counts) {
            num_combinations += 1;
            continue;
        }

        for ns in successors(&s, counts) {
            states.push(ns);
        }
    }

    num_combinations
}

fn main() {
    let sum: u32 = io::stdin().lines()
        .map(|line| parse_line(&line.unwrap()))
        .map(|(springs, counts)| count_combinations(&springs, &counts))
        .sum();

    println!("{sum}");
}
