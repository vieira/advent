use std::io;

type Grid = Vec<Vec<char>>;
type Position = (i32, i32);

fn adjacents((x, y): Position) -> impl Iterator<Item = Position> {
    (-1..=1).flat_map(move |dx| {
        (-1..=1)
            .filter(move |&dy| dx != 0 || dy != 0)
            .map(move |dy| (x + dx, y + dy))
    })
}

fn has_max_rolls(grid: &Grid, (x, y): Position) -> bool {
    let count = adjacents((x, y))
        .filter_map(|(xn, yn)| grid.get(yn as usize)?.get(xn as usize))
        .filter(|&c| *c == '@')
        .take(4)
        .count();

    count >= 4
}

fn count_locations(grid: &Grid) -> usize {
    (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .filter(|&x| grid[y][x] == '@')
                .filter(|&x| !has_max_rolls(grid, (x as i32, y as i32)))
                .count()
        })
        .sum()
}

fn main() {
    let grid: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    println!("{}", count_locations(&grid));
}
