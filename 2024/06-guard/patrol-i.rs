use std::collections::HashSet;
use std::io;

type Grid = Vec<Vec<bool>>;
type Position = (usize, usize);
type Orientation = (isize, isize);
type Guard = (Position, Orientation);

fn rotate(guard: &Guard) -> Guard {
    let (position, (x, y)) = guard;
    (*position, (-y, *x))
}

fn patrol(grid: &Grid, guard: &Guard) -> usize {
    let (mut position, mut orientation) = guard;
    let mut visited = HashSet::from([position]);
    loop {
        let (x, y) = position;
        let (dx, dy) = orientation;
        let xn = x as isize + dx;
        let yn = y as isize + dy;

        if xn < 0 || yn < 0 {
            break;
        }

        let xn = xn as usize;
        let yn = yn as usize;

        if yn >= grid.len() || xn >= grid[yn].len() {
            break;
        }

        if grid[yn][xn] {
            (position, orientation) = rotate(&((x, y), (dx, dy)));
            continue;
        }

        visited.insert((xn, yn));
        position = (xn, yn);
    }

    visited.len()
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

    let count = patrol(&grid, &guard);
    println!("{count}");
}
