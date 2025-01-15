use std::io;
use std::ops::Range;

fn match_op(input: &str) -> Option<Vec<u32>> {
    let pos = input.find(')')?;
    let (lhs, rhs) = input[..pos].split_once(',')?;
    let ops: Vec<u32> = [lhs, rhs]
        .iter()
        .filter(|op| !op.is_empty() && op.len() <= 3)
        .filter_map(|op| op.parse().ok())
        .collect();

    match ops.len() {
        2 => Some(ops),
        _ => None,
    }
}

fn disabled_ranges(input: &str) -> Vec<Range<usize>> {
    let on: Vec<_> = input.match_indices("do()").map(|(i, _)| i).collect();
    input
        .match_indices("don't()")
        .map(|(i, _)| i..*on.iter().find(|&el| *el > i).unwrap_or(&input.len()))
        .collect()
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let disabled = disabled_ranges(&input);
    let sum: u32 = input
        .match_indices("mul(")
        .filter(|(idx, _)| !disabled.iter().any(|r| r.contains(idx)))
        .filter_map(|(idx, m)| match_op(&input[idx + m.len()..]))
        .map(|ops| ops.iter().product::<u32>())
        .sum();

    println!("{sum}");
}
