use std::cmp;
use std::io;

type Position = (u64, u64);
type Machine = (Position, Position, Position);

fn parse_coord(c: &str) -> u64 {
    let [_, value] = c.split(['+', '=']).collect::<Vec<_>>()[..] else {
        unreachable!()
    };
    value.parse().unwrap()
}

fn parse_line(line: &str) -> Position {
    let (_, coords) = line.split_once(": ").unwrap();
    let (x_coord, y_coord) = coords.split_once(", ").unwrap();
    (parse_coord(x_coord), parse_coord(y_coord))
}

fn min_cost(machine: &Machine) -> u64 {
    let mut min_cost = u64::MAX;
    let ((ax, ay), (bx, by), (x, y)) = machine;

    for a in 0..100 {
        for b in 0..100 {
            if a * ax + b * bx == *x && a * ay + b * by == *y {
                min_cost = cmp::min(min_cost, a * 3 + b);
            }
        }
    }

    match min_cost { u64::MAX => 0, _ => min_cost }
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut total_cost = 0;

    while let Some(line) = lines.next() {
        let button_a = parse_line(&line.unwrap());
        let button_b = parse_line(&lines.next().unwrap().unwrap());
        let prize = parse_line(&lines.next().unwrap().unwrap());
        total_cost += min_cost(&(button_a, button_b, prize));
        lines.next();
    }

    println!("{total_cost}");
}
