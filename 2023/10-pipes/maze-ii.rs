use std::io;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::HashSet;

type Position = (u32, u32);
type Positions = HashSet<Position>;
type Rotation = (i32, i32);
type Pipe = (Position, Position);
type Map = HashMap<Position, Pipe>;
type Path = Vec<Position>;

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

fn adjacents(&(x, y): &Position) -> Vec<Position> {
    [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)].iter()
        .copied()
        .filter(|(xx, yy)| *xx > 0 && *yy > 0)
        .collect()
}

fn find_loop(map: &Map, start: &Path) -> Path {
    let mut path = start.clone();
    let mut visited: HashSet<Position> = HashSet::from_iter(
        path.iter().copied(),
    );
    let Some(last_position) = path.iter().find(|p| map.contains_key(p))
    else { return vec![] };

    let mut last_position = *last_position;

    loop {
        let Some((a, b)) = map.get(&last_position) else { break };
        let next_position = if !visited.contains(a) { *a } else { *b };

        if visited.contains(&next_position) { break };
        visited.insert(next_position);
        path.push(next_position);

        last_position = next_position;
    }

    path
}

fn rotation(prev: &Position, next: &Position, clockwise: bool) -> Rotation {
    let k = if clockwise { 1 } else { -1 };

    if next.0 > prev.0 {
        return (0, k);
    }

    if next.0 < prev.0 {
        return (0, -k);
    }

    if next.1 > prev.1 {
        return (-k, 0);
    }

    if next.1 < prev.1 {
        return (k, 0);
    }

    (0, 0)
}

fn line(s: &Position, r: &Rotation, path: &Positions, points: &mut Positions) {
    let mut p = *s;
    loop {
        p = ((p.0 as i32 + r.0) as u32, (p.1 as i32 + r.1) as u32);
        if path.contains(&p) { break };
        points.insert(p);
    }
}

fn loop_area(path: &Path) -> u32 {
    let mut loop_id = 0;
    let mut points = HashSet::new();
    let positions = HashSet::from_iter(path.iter().copied());
    let (start_idx, start_pos) = path.iter()
        .copied()
        .enumerate()
        .min_by(|(_, p), (_, q)| p.cmp(q))
        .unwrap();

    let mut prev_pos = start_pos;
    let next_pos = path[start_idx + 1];
    let clockwise = next_pos.1 <= prev_pos.1;
    let mut prev_rotation = rotation(&prev_pos, &next_pos, clockwise);

    for next_pos in path.iter().cycle().skip(start_idx + 1) {
        if loop_id > 1 { break };
        if *next_pos == start_pos { loop_id += 1 };

        let next_rotation = rotation(&prev_pos, next_pos, clockwise);
        if next_rotation != prev_rotation {
            line(&prev_pos, &next_rotation, &positions, &mut points);
        }
        line(next_pos, &next_rotation, &positions, &mut points);

        prev_rotation = next_rotation;
        prev_pos = *next_pos;
    }

    points.len() as u32
}

fn main() {
    let (start, map) = parse_maze();
    let max_loop = adjacents(&start).iter()
        .map(|&position| find_loop(&map, &vec![start, position]))
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap_or(vec![]);


    let area = loop_area(&max_loop);
    println!("{area}");
}
