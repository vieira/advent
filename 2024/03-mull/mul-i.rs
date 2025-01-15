use std::io;

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

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let sum: u32 = input
        .match_indices("mul(")
        .filter_map(|(idx, m)| match_op(&input[idx + m.len()..]))
        .map(|ops| ops.iter().product::<u32>())
        .sum();

    println!("{sum}");
}
