use std::io;

fn divide(a: i128, b: i128) -> Option<i128> {
    match a % b {
        0 => Some(a / b),
        _ => None,
    }
}

fn is_valid(value: i128, numbers: &[i128]) -> bool {
    if value < 0 { return false };
    if numbers.is_empty() { return value == 0 };
    let [head @ .., last] = numbers else { unreachable!() };
    divide(value, *last).is_some_and(|x| is_valid(x, head))
        || is_valid(value - last, head)
}

fn main() {
    let sum: i128 = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (value, numbers) = line.split_once(": ").unwrap();
            let value: i128 = value.parse().unwrap();
            let numbers: Vec<i128> =
                numbers.split(' ').map(|x| x.parse().unwrap()).collect();
            (value, numbers)
        })
        .filter(|(value, numbers)| is_valid(*value, numbers))
        .map(|(value, _)| value)
        .sum();
    println!("{sum}");
}
