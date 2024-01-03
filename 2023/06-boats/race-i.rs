use std::io;

fn parse_numbers(s: &str) -> Vec<i32> {
    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn parse_line(line: Option<Result<String, io::Error>>) -> Vec<i32> {
    let line = line.unwrap().unwrap();
    let (_, numbers) = line.split_once(':').unwrap();
    parse_numbers(numbers)
}

fn solve_quadratic(a: i32, b: i32, c: i32) -> (i32, i32) {
    let bb = b.pow(2) as f32;
    let a = a as f32;
    let c = c as f32;
    let b = b as f32;

    let sqrt = (bb - 4. * a * c).sqrt();

    let min = (-b + sqrt) / (2. * a);
    let max = (-b - sqrt) / (2. * a);

    let min = if min.ceil() == min { min + 1. } else { min.ceil() } as i32;
    let max = if max.floor() == max { max - 1. } else { max.floor() } as i32;

    (min, max)
}

fn main() {
    let mut lines = io::stdin().lines();
    let times = parse_line(lines.next());
    let distances = parse_line(lines.next());
    let product: i32 = times.iter()
        .enumerate()
        .map(|(i, &t)| solve_quadratic(-1, t, -distances[i]))
        .map(|(min, max)| max - min + 1)
        .product();
    println!("{product}");
}
