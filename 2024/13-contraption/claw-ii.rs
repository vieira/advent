use std::io;

type Position = (i64, i64);
type Machine = (Position, Position, Position);

fn parse_coord(c: &str) -> i64 {
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

fn min_cost(machine: &Machine) -> i64 {
    let ((ax, ay), (bx, by), (x, y)) = machine;
    let b = (ax * y - ay * x) / (ax * by - ay * bx);
    let a = (x - bx * b) / ax;

    if (ax * a + bx * b, ay * a + by * b) != (*x, *y) {
        return 0;
    }

    3 * a + b
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut total_cost = 0;

    while let Some(line) = lines.next() {
        let button_a = parse_line(&line.unwrap());
        let button_b = parse_line(&lines.next().unwrap().unwrap());
        let (x, y) = parse_line(&lines.next().unwrap().unwrap());
        let prize = (x + 10000000000000, y + 10000000000000);
        total_cost += min_cost(&(button_a, button_b, prize));
        lines.next();
    }

    println!("{total_cost}");
}
