use std::io;
use std::cmp;
use std::collections::HashSet;

type Trails = Vec<Vec<char>>;
type Position = (usize, usize);

#[derive(Clone)]
struct Hike {
    position: Position,
    visited: HashSet<Position>,
}

fn is_goal(hike: &Hike, trails: &Trails) -> bool {
    let (_, y) = hike.position;

    y == trails.len() - 1
}

fn successors(hike: &Hike, trails: &Trails) -> Vec<Hike> {
    let (x, y) = hike.position;
    let (nx, ny) = match trails[y][x] {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        'v' => (x, y + 1),
        '^' => (x, y - 1),
        _ => (x, y),
    };

    if nx != x || ny != y {
        if hike.visited.contains(&(nx, ny)) {
            return vec![];
        }

        let mut next_hike = hike.clone();
        next_hike.visited.insert((nx, ny));
        next_hike.position = (nx, ny);

        return vec![next_hike];
    }

    let x = x as isize;
    let y = y as isize;

    [(1, 0), (-1_isize, 0), (0, 1), (0, -1)]
        .iter()
        .filter_map(|(dx, dy)| {
            if x + dx < 0 || y + dy < 0 { return None };
            let nx = (x + dx) as usize;
            let ny = (y + dy) as usize;

            if trails[ny][nx] == '#' { return None };
            if hike.visited.contains(&(nx, ny)) { return None };

            let mut next_hike = hike.clone();
            next_hike.visited.insert((nx, ny));
            next_hike.position = (nx, ny);

            Some(next_hike)
        })
        .collect()
}

fn search(start: &Position, trails: &Trails) -> usize {
    let initial = Hike {
        position: *start,
        visited: HashSet::from([*start]),
    };
    let mut queue = vec![initial];
    let mut max_visited = 0;

    while let Some(hike) = queue.pop() {
        if is_goal(&hike, trails) {
            max_visited = cmp::max(hike.visited.len(), max_visited);
            continue;
        }

        for next_hike in successors(&hike, trails) {
            queue.push(next_hike);
        }
    }

    max_visited - 1
}

fn main() {
    let trails: Trails = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect();

    let x = trails[0].iter().position(|&c| c == '.').unwrap();
    let start = (x, 0);

    let max_steps = search(&start, &trails);
    println!("{max_steps}");
}
