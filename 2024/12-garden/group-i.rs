use std::collections::HashSet;
use std::io;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);
type Positions = HashSet<Position>;
type State = (u64, Positions);
type Direction = (isize, isize);

enum Adjacent {
    Plot(Position),
    Inside(Position),
    Outside,
}

use Adjacent::*;

fn directions() -> Vec<Direction> {
    vec![(0, -1), (1, 0), (0, 1), (-1, 0)]
}

fn adjacents(grid: &Grid, (x, y): &Position) -> Vec<Adjacent> {
    directions()
        .iter()
        .map(|(dx, dy)| {
            let (xn, yn) = (*x as isize + dx, *y as isize + dy);
            if xn < 0 || yn < 0 {
                return Outside;
            };

            let (xn, yn) = (xn as usize, yn as usize);
            if yn >= grid.len() || xn >= grid[yn].len() {
                return Outside;
            };

            if grid[yn][xn] != grid[*y][*x] {
                return Inside((xn, yn));
            }

            Plot((xn, yn))
        })
        .collect()
}

fn explore_from(grid: &Grid, start: &Position, seen: &mut Positions) -> State {
    let mut queue = vec![*start];
    let mut successors = HashSet::new();
    let mut perimeter = 0;
    let mut area = 0;

    while let Some(position) = queue.pop() {
        if seen.contains(&position) {
            continue;
        };

        for adjacent in adjacents(grid, &position) {
            match adjacent {
                Plot(next_position) => {
                    queue.push(next_position);
                },
                Inside(next_position) => {
                    successors.insert(next_position);
                    perimeter += 1;
                },
                Outside => {
                    perimeter += 1;
                }
            }
        }
        seen.insert(position);
        area += 1;
    }

    (perimeter * area, successors)
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
