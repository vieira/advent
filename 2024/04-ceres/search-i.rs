use std::io;

type Grid = Vec<Vec<char>>;
type Position = (isize, isize);
type Direction = (isize, isize);

fn matches_here(grid: &Grid, pat: &str, p: &Position, dir: &Direction) -> bool {
    let (x, y) = p;
    let (dx, dy) = dir;

    for (i, c) in pat.chars().enumerate() {
        let nx = x + (i as isize * dx);
        let ny = y + (i as isize * dy);

        if ny < 0 || ny >= grid.len() as isize {
            return false;
        }

        let ny = ny as usize;
        if nx < 0 || nx >= grid[ny].len() as isize {
            return false;
        }

        let nx = nx as usize;
        if grid[ny][nx] != c {
            return false;
        }
    }

    true
}

fn matches(grid: &Grid, pat: &str) -> usize {
    let directions: Vec<Direction> = (-1..=1)
        .flat_map(|x| {
            (-1..=1)
                .filter(|&y| x != 0 || y != 0)
                .map(|y| (x, y))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut count = 0;
    for y in 0..grid.len() as isize {
        for x in 0..grid[y as usize].len() as isize {
            for dir in &directions {
                if matches_here(grid, pat, &(x, y), dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let grid: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect();

    let count = matches(&grid, "XMAS");
    println!("{count}");
}
