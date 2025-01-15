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

fn antinodes(antennas: &[Position], limit: &Position) -> HashSet<Position> {
    let (max_x, max_y) = limit;
    pairs(antennas)
        .iter()
        .flat_map(|(a1, a2)| {
            let (x, y) = a1;
            let (dx, dy) = distance(a2, a1);
            let mut i = 0;
            let mut antis = vec![];
            loop {
                let (xn, yn) = (x + i * dx, y + i * dy);
                if xn < 0 || xn > *max_x || yn < 0 && yn > *max_y { break };
                antis.push((xn, yn));
                i += 1;
            }
            antis
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
        .flat_map(|(_, ants_freq)| antinodes(ants_freq, &(max_x, max_y)))
        .filter(|(x, y)| 0 <= *x && *x <= max_x && 0 <= *y && *y <= max_y)
        .collect::<HashSet<_>>()
        .len();
    println!("{count}");
}
