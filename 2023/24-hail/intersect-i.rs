use std::io;
use std::str::FromStr;
use std::fmt::Debug;

type Position = (usize, usize, usize);
type Velocity = (isize, isize, isize);
type Hailstone = (Position, Velocity);
type Point = (f64, f64);

fn parse_numbers<T: FromStr>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.split(',').map(|n| n.trim().parse::<T>().unwrap()).collect()
}

fn parse_line(line: &str) -> (Position, Velocity) {
    let (position, velocity) = line.split_once(" @ ").unwrap();
    let position = parse_numbers::<usize>(position);
    let velocity = parse_numbers::<isize>(velocity);

    let [x, y, z] = position[..] else { panic!("Invalid position") };
    let [vx, vy, vz] = velocity[..] else { panic!("Invalid velocity") };

    ((x, y, z), (vx, vy, vz))
}

fn line((x1, y1): &Point, (x2, y2): &Point) -> (f64, f64) {
    let m = (y2 - y1) / (x2 - x1);
    let b = y1 - m * x1;

    (m, b)
}

fn points(h: &Hailstone) -> (Point, Point) {
    let ((x, y, _), (vx, vy, _)) = h;
    let x1 = *x as f64;
    let y1 = *y as f64;
    let x2 = (*x as isize + vx) as f64;
    let y2 = (*y as isize + vy) as f64;

    ((x1, y1), (x2, y2))
}

fn distance((x1, y1): &Point, (x2, y2): &Point) -> f64 {
    (y2 - y1).abs() + (x2 - x1).abs()
}

fn intersection(h1: &Hailstone, h2: &Hailstone) -> Option<Point> {
    let ((x1, y1), (x2, y2)) = points(h1);
    let ((x3, y3), (x4, y4)) = points(h2);

    let (m1, b1) = line(&(x1, y1), &(x2, y2));
    let (m2, b2) = line(&(x3, y3), &(x4, y4));

    if m1 == m2 { return None };

    let x = (b1 - b2) / (m2 - m1);
    let y = m1 * x + b1;

    if distance(&(x, y), &(x1, y1)) < distance(&(x, y), &(x2, y2)) {
        return None;
    }

    if distance(&(x, y), &(x3, y3)) < distance(&(x, y), &(x4, y4)) {
        return None;
    }

    Some((x, y))
}

fn main() {
    let hailstones: Vec<Hailstone> = io::stdin()
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect();

    let min = 200000000000000.;
    let max = 400000000000000.;
    let mut sum = 0;

    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let a = hailstones[i];
            let b = hailstones[j];
            if let Some((x, y)) = intersection(&a, &b) {
                if x < min || x > max || y < min || y > max { continue };
                sum += 1;
            }
        }
    }

    println!("{sum}");
}
