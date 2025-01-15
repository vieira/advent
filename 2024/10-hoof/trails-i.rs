use std::io;
use std::collections::HashSet;

type Grid = Vec<Vec<u32>>;
type Position = (usize, usize);

fn next_positions(grid: &Grid, p: &Position) -> Vec<Position> {
    let (x, y) = p;

    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .filter_map(|(dx, dy)| {
            let (xn, yn) = (*x as isize + dx, *y as isize + dy);
            if xn < 0 || yn < 0 { return None };

            let (xn, yn) = (xn as usize, yn as usize);
            if yn >= grid.len() || xn >= grid[yn].len() { return None };

            if grid[yn][xn] != grid[*y][*x] + 1 { return None };

            Some((xn, yn))
        })
        .collect()
}

fn search_from(grid: &Grid, start: &Position) -> usize {
    let mut positions = vec![*start];
    let mut peaks = HashSet::new();

    while let Some(position) = positions.pop() {
        let (x, y) = position;
        if grid[y][x] == 9 {
            peaks.insert((x, y));
            continue;
        }

        for position in next_positions(grid, &position) {
            positions.push(position);
        }
    }

    peaks.len()
}

fn search(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                .map(|(x, _)| search_from(grid, &(x, y)))
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let grid: Grid = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()
        })
        .collect();

    let sum = search(&grid);
    println!("{sum}");
}
