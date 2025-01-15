use std::io;

fn is_safe(report: &[i32]) -> bool {
    let diffs: Vec<_> = (1..report.len())
        .map(|i| report[i] - report[i - 1])
        .collect();

    diffs.iter().all(|d| {
        diffs[0].signum() == d.signum() && d.abs() >= 1 && d.abs() <= 3
    })
}

fn is_tolerable(report: &[i32]) -> bool {
    (-1_i32..report.len() as i32).any(|i| {
        let report: Vec<_> = report
            .iter()
            .enumerate()
            .filter(|(j, _)| *j as i32 != i)
            .map(|(_, v)| *v)
            .collect();
        is_safe(&report)
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
        .filter(|report| is_tolerable(report))
        .count();

    println!("{count}");
}
