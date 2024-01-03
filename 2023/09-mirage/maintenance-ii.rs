use std::io;

fn parse_numbers(s: &str) -> Vec<i32> {
    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn extrapolate(arr: &mut Vec<i32>) -> i32 {
    let mut i = arr.len();

    loop {
        for j in 1..i {
            arr[j-1] = arr[j] - arr[j-1];
        }

        i -= 1;

        if arr.iter().take(i).all(|x| *x == 0) {
            break;
        }
    }

    arr.iter().sum()
}

fn main() {
    let sum: i32 = io::stdin().lines()
        .map(|line| parse_numbers(&line.unwrap()))
        .map(|readings| readings.iter().copied().rev().collect::<Vec<i32>>())
        .map(|mut readings| extrapolate(&mut readings))
        .sum();

    println!("{sum}");
}
