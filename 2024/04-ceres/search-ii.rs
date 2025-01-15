use std::io;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);

fn matches_here(grid: &Grid, (x, y): &Position) -> bool {
    let diags = [
        [grid[y - 1][x - 1], grid[*y][*x], grid[y + 1][x + 1]],
        [grid[y - 1][x + 1], grid[*y][*x], grid[y + 1][x - 1]],
    ];

    diags.iter().all(|d| {
        let text = d.iter().collect::<String>();
        text == "MAS" || text == "SAM"
    })
}

fn matches(grid: &Grid) -> usize {
    let mut count = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if matches_here(grid, &(x, y)) {
                count += 1;
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

    let count = matches(&grid);
    println!("{count}");
}
