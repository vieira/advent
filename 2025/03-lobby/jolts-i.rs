use std::io;

fn largest_joltage(bank: &[u32]) -> Option<u32> {
    let (idx, msd) = bank[..bank.len() - 1]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, &val)| val)?;
    let lsd = bank[idx + 1..].iter().max()?;

    Some(msd * 10 + lsd)
}

fn main() {
    let sum: u32 = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .filter_map(|bank| largest_joltage(&bank))
        .sum();

    println!("{sum}");
}
