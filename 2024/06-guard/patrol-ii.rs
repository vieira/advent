use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

type Grid = Vec<Vec<bool>>;
type Position = (usize, usize);
type Positions = Vec<Position>;
type Orientation = (isize, isize);
type Guard = (Position, Orientation);
type Obstacle = Option<Position>;

fn rotate(guard: &Guard) -> Guard {
    let (position, (x, y)) = guard;
    (*position, (-y, *x))
}

fn advance(grid: &Grid, ((x, y), (dx, dy)): &Guard) -> Option<Guard> {
    let (xn, yn) = (*x as isize + dx, *y as isize + dy);
    if xn < 0 || yn < 0 {
        return None;
    }

    let (xn, yn) = (xn as usize, yn as usize);
    if yn >= grid.len() || xn >= grid[yn].len() {
        return None;
    }

    Some(((xn, yn), (*dx, *dy)))
}

fn patrol(grid: &Grid, guard: &Guard, obstacle: Obstacle) -> Option<Positions> {
    let mut path = vec![];
    let mut states = HashSet::new();
    let mut guard = *guard;

    loop {
        let Some(((xn, yn), (dx, dy))) = advance(grid, &guard) else { break };

        if grid[yn][xn] || obstacle.is_some_and(|o| o == (xn, yn)) {
            guard = rotate(&guard);
            if states.contains(&guard) {
                return None;
            }
            states.insert(guard);
            continue;
        }

        guard = ((xn, yn), (dx, dy));
        path.push((xn, yn));
    }

    Some(path)
}

fn count_loops(grid: &Grid, guard: &Guard) -> usize {
    let positions = patrol(grid, guard, None).unwrap();
    let path: HashSet<Position> = HashSet::from_iter(positions);
    path.iter()
        .filter(|&position| patrol(grid, guard, Some(*position)).is_none())
        .count()
}

fn main() {
    let mut guard = ((0, 0), (0, 0));
    let grid: Grid = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    guard = match c {
                        '^' => ((x, y), (0, -1)),
                        '>' => ((x, y), (1, 0)),
                        'v' => ((x, y), (0, 1)),
                        '<' => ((x, y), (-1, 0)),
                        _ => guard,
                    };

                    c == '#'
                })
                .collect()
        })
        .collect();

    let count = count_loops(&grid, &guard);
    println!("{count}");
}
