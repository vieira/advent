use std::io;

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<_> = (1..report.len())
        .map(|i| report[i] - report[i - 1])
        .collect();

    diffs.iter().all(|d| {
        diffs[0].signum() == d.signum() && d.abs() >= 1 && d.abs() <= 3
    })
}

fn main() {
    let count = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| is_safe(report))
        .count();

    println!("{count}");
}
