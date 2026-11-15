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

fn reachable_locations(grid: &Grid) -> Vec<Position> {
    (0..grid.len())
        .flat_map(|y| {
            (0..grid[y].len())
                .filter(|&x| grid[y][x] == '@')
                .filter(|&x| !has_max_rolls(grid, (x as i32, y as i32)))
                .map(|x| (x as i32, y as i32))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn remove_locations(grid: &mut Grid, locations: &[Position]) {
    locations.iter().for_each(|&(x, y)| grid[y as usize][x as usize] = '.');
}

fn remove_rolls(grid: &mut Grid) -> usize {
    let mut count = 0;
    loop {
        let locations = reachable_locations(grid);
        if locations.is_empty() { break }
        count += locations.len();
        remove_locations(grid, &locations);
    }

    count
}

fn main() {
    let mut grid: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    println!("{}", remove_rolls(&mut grid));
}
