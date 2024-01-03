use std::io;

type Position = (i32, i32);

fn parse_line(line: &str) -> (char, u32, String) {
    let (direction, rest) = line.split_once(' ').unwrap();
    let (len, color) = rest.split_once(' ').unwrap();
    let color = &color[2..color.len() - 1];

    (direction.chars().next().unwrap(), len.parse().unwrap(), color.to_owned())
}

fn next_position(position: &Position, direction: char, len: u32) -> Position {
    let (x, y) = position;
    let len = len as i32;

    match direction {
        'U' => (*x, y - len),
        'D' => (*x, y + len),
        'L' => (x - len, *y),
        _ => (x + len, *y),
    }
}

fn area(positions: &[Position]) -> u32 {
    let mut total_area: i32 = 0;
    let mut border: i32 = 0;

    for i in 1..positions.len() {
        let (ax, ay) = positions[i - 1];
        let (bx, by) = positions[i];

        total_area += ax * by - bx * ay;
        border += (by - ay).abs() + (bx - ax).abs();
    }

    let total_area = total_area.abs() / 2;
    let interior = total_area - border / 2 + 1;

    (interior + border) as u32
}

fn main() {
    let mut current_position = (0, 0);
    let mut positions = vec![current_position];

    for line in io::stdin().lines() {
        let (direction, len, _) = parse_line(&line.unwrap());
        let position = next_position(&current_position, direction, len);
        current_position = position;
        positions.push(position);
    }

    let a = area(&positions);
    println!("{a}");
}
