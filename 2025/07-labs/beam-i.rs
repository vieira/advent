use std::io;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

fn is_inside(grid: &Grid, (x, y): (isize, isize)) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    if y as usize >= grid.len() || x as usize >= grid[0].len() {
        return false;
    }

    true
}

fn count_splits(grid: &Grid, start: (isize, isize)) -> usize {
    let mut count = 0;
    let mut visited = HashSet::new();
    let mut heads = vec![start];

    while let Some((x, y)) = heads.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        let ny = y + 1;
        if !is_inside(grid, (x, ny)) {
            continue;
        }


        match grid[ny as usize][x as usize] {
            '^' => {
                heads.push((x - 1, ny));
                heads.push((x + 1, ny));
                count += 1;
            }
            _ => heads.push((x, ny)),
        }
    }

    count
}

fn main() {
    let grid: Grid = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (x, y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, r)| r.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .unwrap();

    println!("{}", count_splits(&grid, (x as isize, y as isize)));
}
