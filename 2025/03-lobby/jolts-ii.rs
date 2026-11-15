use std::io;

fn largest_joltage(bank: &[u32], digit: usize) -> Option<u64> {
    if digit == 0 { return Some(0) }
    let digit = digit - 1;
    let (idx, msd) = bank[..bank.len() - digit]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, &val)| val)?;

    largest_joltage(&bank[idx + 1..], digit)
        .map(|r| r + *msd as u64 * 10_u64.pow(digit as u32))
}

fn main() {
    let sum: u64 = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .filter_map(|bank| largest_joltage(&bank, 12))
        .sum();

    println!("{sum}");
}
