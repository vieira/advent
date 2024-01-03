use std::io;

type Position = (i64, i64);

fn parse_line(line: &str) -> (char, u32) {
    let (_, rest) = line.split_once(' ').unwrap();
    let (_, instruction) = rest.split_once(' ').unwrap();
    let instruction = &instruction[2..instruction.len() - 1];
    let direction = match instruction.chars().last() {
        Some('1') => 'D',
        Some('2') => 'L',
        Some('3') => 'U',
        _ => 'R',
    };
    let len = u32::from_str_radix(&instruction[..instruction.len() - 1], 16)
        .unwrap();

    (direction, len)
}

fn next_position(position: &Position, direction: char, len: u32) -> Position {
    let (x, y) = position;
    let len = len as i64;

    match direction {
        'U' => (*x, y - len),
        'D' => (*x, y + len),
        'L' => (x - len, *y),
        _ => (x + len, *y),
    }
}

fn area(positions: &[Position]) -> u64 {
    let mut total_area: i64 = 0;
    let mut border: i64 = 0;

    for i in 1..positions.len() {
        let (ax, ay) = positions[i - 1];
        let (bx, by) = positions[i];

        total_area += ax * by - bx * ay;
        border += (by - ay).abs() + (bx - ax).abs();
    }

    let total_area = total_area.abs() / 2;
    let interior = total_area - border / 2 + 1;

    (interior + border) as u64
}

fn main() {
    let mut current_position = (0, 0);
    let mut positions = vec![current_position];

    for line in io::stdin().lines() {
        let (direction, len) = parse_line(&line.unwrap());
        let position = next_position(&current_position, direction, len);
        current_position = position;
        positions.push(position);
    }

    let a = area(&positions);
    println!("{a}");
}
