use std::io;

fn parse_line(line: Option<Result<String, io::Error>>) -> i64 {
    let line = line.unwrap().unwrap();
    let (_, digits) = line.split_once(':').unwrap();
    digits.split_whitespace().collect::<String>().parse().unwrap()
}

fn solve_quadratic(a: i64, b: i64, c: i64) -> (i64, i64) {
    let bb = b.pow(2) as f64;
    let a = a as f64;
    let c = c as f64;
    let b = b as f64;

    let sqrt = (bb - 4. * a * c).sqrt();

    let min = (-b + sqrt) / (2. * a);
    let max = (-b - sqrt) / (2. * a);

    let min = if min.ceil() == min { min + 1. } else { min.ceil() } as i64;
    let max = if max.floor() == max { max - 1. } else { max.floor() } as i64;

    (min, max)
}

fn main() {
    let mut lines = io::stdin().lines();
    let time = parse_line(lines.next());
    let distance = parse_line(lines.next());
    let (min, max) = solve_quadratic(-1, time, -distance);
    let wins = max - min + 1;
    println!("{wins}");
}
