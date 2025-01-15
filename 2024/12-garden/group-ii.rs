use std::collections::HashSet;
use std::io;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);
type Positions = HashSet<Position>;
type State = (u64, Positions);
type Direction = (isize, isize);
type Point = (isize, isize);
type Perimeter = HashSet<(Point, Direction)>;

enum Adjacent {
    Plot(Position),
    Inside(Point, Direction),
    Outside(Point, Direction),
}

use Adjacent::*;

fn directions() -> Vec<Direction> {
    vec![(0, -1), (1, 0), (0, 1), (-1, 0)]
}

fn adjacents(grid: &Grid, (x, y): &Position) -> Vec<Adjacent> {
    let (x, y) = (*x as isize, *y as isize);
    directions()
        .iter()
        .map(|&direction| {
            let (dx, dy) = direction;
            let (xn, yn) = (x + dx, y + dy);

            if xn < 0 || yn < 0 {
                return Outside((xn, yn), direction);
            };

            let (xn, yn) = (xn as usize, yn as usize);

            if yn >= grid.len() || xn >= grid[yn].len() {
                return Outside((xn as isize, yn as isize), direction);
            };

            if grid[yn][xn] != grid[y as usize][x as usize] {
                return Inside((xn as isize, yn as isize), direction);
            }

            Plot((xn, yn))
        })
        .collect()
}

fn sides(perimeter: &mut Perimeter) -> u64 {
    let mut count = 0;

    loop {
        let Some(((x, y), side)) = perimeter.iter().min().cloned() else {
            break;
        };

        let Some(direction) = directions()
            .iter()
            .find(|(dx, dy)| perimeter.contains(&((x + dx, y + dy), side)))
            .cloned()
        else {
            perimeter.remove(&((x, y), side));
            count += 1;
            continue;
        };

        let (mut xn, mut yn) = (x, y);
        let (dx, dy) = direction;

        loop {
            let perimeter_point = ((xn, yn), side);
            if !perimeter.contains(&perimeter_point) {
                break;
            };
            perimeter.remove(&perimeter_point);
            (xn, yn) = (xn + dx, yn + dy);
        }
        count += 1;
    }

    count
}

fn explore_from(grid: &Grid, start: &Position, seen: &mut Positions) -> State {
    let mut queue = vec![*start];
    let mut successors = HashSet::new();
    let mut perimeter = HashSet::new();
    let mut area = 0;

    while let Some(position) = queue.pop() {
        if seen.contains(&position) {
            continue;
        };

        for adjacent in adjacents(grid, &position) {
            match adjacent {
                Plot(next_position) => {
                    queue.push(next_position);
                }
                Inside(point, direction) => {
                    successors.insert((point.0 as usize, point.1 as usize));
                    perimeter.insert((point, direction));
                }
                Outside(point, direction) => {
                    perimeter.insert((point, direction));
                }
            }
        }
        seen.insert(position);
        area += 1;
    }

    (sides(&mut perimeter) * area, successors)
}

fn explore(grid: &Grid) -> u64 {
    let mut queue = vec![(0, 0)];
    let mut seen = HashSet::new();
    let mut total = 0;

    while let Some(position) = queue.pop() {
        if seen.contains(&position) {
            continue;
        };
        let (cost, successors) = explore_from(grid, &position, &mut seen);
        for successor in successors {
            queue.push(successor);
        }
        total += cost;
    }

    total
}

fn main() {
    let grid: Grid = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let cost = explore(&grid);
    println!("{cost}");
}
