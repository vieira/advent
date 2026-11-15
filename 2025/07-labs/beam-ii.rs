use std::collections::HashMap;
use std::io;

type Grid = Vec<Vec<char>>;
type Position = (isize, isize);
type Cache = HashMap<Position, usize>;

fn is_inside(grid: &Grid, (x, y): (isize, isize)) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    if y as usize >= grid.len() || x as usize >= grid[0].len() {
        return false;
    }

    true
}

fn count_timelines(grid: &Grid, (x, y): Position, cache: &mut Cache) -> usize {
    let ny = y + 1;
    if !is_inside(grid, (x, ny)) {
        return 1;
    }

    if let Some(count) = cache.get(&(x, ny)) {
        return *count;
    }

    let count = match grid[ny as usize][x as usize] {
        '^' => {
            count_timelines(grid, (x - 1, ny), cache)
                + count_timelines(grid, (x + 1, ny), cache)
        }
        _ => count_timelines(grid, (x, ny), cache),
    };

    cache.insert((x, ny), count);
    count
}

fn main() {
    let mut cache = HashMap::new();
    let grid: Grid = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let (x, y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, r)| r.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .unwrap();

    let count = count_timelines(&grid, (x as isize, y as isize), &mut cache);
    println!("{count}");
}
