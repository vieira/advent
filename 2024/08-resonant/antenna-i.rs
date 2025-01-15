use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

type Position = (isize, isize);

fn distance(p: &Position, q: &Position) -> (isize, isize) {
    (p.0 - q.0, p.1 - q.1)
}

fn pairs(antennas: &[Position]) -> Vec<(Position, Position)> {
    antennas
        .iter()
        .enumerate()
        .flat_map(|(i, a1)| {
            antennas[i + 1..]
                .iter()
                .flat_map(|a2| [(*a1, *a2), (*a2, *a1)])
                .collect::<Vec<_>>()
        })
        .collect()
}

fn antinodes(antennas: &[Position]) -> HashSet<Position> {
    pairs(antennas)
        .iter()
        .map(|(a1, a2)| {
            let (x, y) = a1;
            let (dx, dy) = distance(a1, a2);
            (x + dx, y + dy)
        })
        .collect()
}

fn main() {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let (mut max_x, mut max_y) = (0, 0);

    for (y, line) in io::stdin().lines().map(|line| line.unwrap()).enumerate() {
        max_y = y as isize;
        for (x, c) in line.chars().enumerate() {
            max_x = x as isize;
            if c == '.' { continue };
            antennas
                .entry(c)
                .or_default()
                .push((x as isize, y as isize));
        }
    }

    let count = antennas
        .iter()
        .flat_map(|(_, antennas_freq)| antinodes(antennas_freq))
        .filter(|(x, y)| 0 <= *x && *x <= max_x && 0 <= *y && *y <= max_y)
        .collect::<HashSet<_>>()
        .len();
    println!("{count}");
}
