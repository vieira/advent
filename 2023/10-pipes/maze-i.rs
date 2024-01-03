use std::io;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::HashSet;

type Position = (u32, u32);
type Pipe = (Position, Position);
type Path = Vec<Position>;
type Map = HashMap<Position, Pipe>;

fn parse_maze() -> (Position, Map) {
    let mut map = HashMap::new();
    let mut start = (0, 0);

    for (y, line) in io::stdin().lines().enumerate() {
        let y = (y + 1) as u32;
        let line = line.unwrap();

        for (x, ch) in line.chars().enumerate() {
            let x = (x + 1) as u32;
            let (a, b) = match ch {
                '|' => ((x, y - 1), (x, y + 1)),
                '-' => ((x - 1, y), (x + 1, y)),
                'L' => ((x + 1, y), (x, y - 1)),
                'J' => ((x, y - 1), (x - 1, y)),
                '7' => ((x - 1, y), (x, y + 1)),
                'F' => ((x + 1, y), (x, y + 1)),
                'S' => {
                    start = (x, y);
                    continue;
                },
                _ => { continue },
            };
            if a.0 == 0 || a.1 == 0 || b.0 == 0 || b.1 == 0 { continue };
            map.insert((x, y), (a, b));
        }
    }

    (start, map)
}

fn find_loop(map: &Map, start: &Path) -> u32 {
    let mut visited: HashSet<Position> = HashSet::from_iter(
        start.iter().copied(),
    );
    let Some(last_position) = start.iter().find(|p| map.contains_key(p))
    else { return 0 };

    let mut last_position = *last_position;

    loop {
        let Some((a, b)) = map.get(&last_position) else { break };
        let next_position = if !visited.contains(a) { *a } else { *b };

        if visited.contains(&next_position) { break };
        visited.insert(next_position);
        last_position = next_position;
    }
    visited.len() as u32
}

fn adjacents(&(x, y): &Position) -> Vec<Position> {
    [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)].iter()
        .copied()
        .filter(|(nx, ny)| *nx > 0 && *ny > 0)
        .collect()
}

fn main() {
    let (start, map) = parse_maze();
    let max_steps = adjacents(&start).iter()
        .map(|&position| find_loop(&map, &vec![start, position]))
        .max()
        .unwrap_or(0);

    let max_dist = max_steps / 2;

    println!("{max_dist}");
}
