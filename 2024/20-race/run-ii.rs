use std::io;
use std::collections::HashMap;

type Grid = Vec<Vec<char>>;
type Position = (usize, usize);
type Costs = HashMap<Position, u32>;

fn calc_costs(grid: &Grid, start: Position, goal: Position) -> Costs {
    let mut costs = HashMap::from([(start, 0)]);
    let mut position = start;
    let mut cost = 0;

    while position != goal {
        let (x, y) = position;

        position = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .iter()
            .copied()
            .find(|(xn, yn)| {
                grid[*yn][*xn] != '#' && !costs.contains_key(&(*xn, *yn))
            })
            .unwrap();
        cost += 1;
        costs.insert(position.clone(), cost);
    }

    costs
}

fn find_shortcuts(grid: &Grid, costs: Costs, radius: u32) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut shortcuts = HashMap::new();

    for (position, cost) in costs.iter() {
        let (x, y) = position;
        for dx in 0..=radius {
            for dy in 0..=radius - dx {
                let jump_cost = dx + dy;
                let (x, y) = (*x as isize, *y as isize);
                let (dx, dy) = (dx as isize, dy as isize);
                let destinations = [
                    (x + dx, y + dy),
                    (x - dx, y + dy),
                    (x + dx, y - dy),
                    (x - dx, y - dy),
                ];

                for (xn, yn) in destinations {
                    if xn < 0 || yn < 0 { continue; }

                    let (xn, yn) = (xn as usize, yn as usize);
                    if yn >= cols || xn >= rows { continue; }

                    if grid[yn][xn] == '#' { continue; }
                    let Some(next_cost) = costs.get(&(xn, yn))
                    else { continue };
                    if next_cost <= cost { continue; }
                    if next_cost - cost > jump_cost {
                        let cost_saved = (next_cost - cost) - jump_cost;
                        shortcuts.insert(((x, y), (xn, yn)), cost_saved);
                    }
                }
            }
        }
    }

    shortcuts.values().filter(|&c| *c >= 100).count()
}

fn main() {
    let mut start = (0, 0);
    let mut goal = (0, 0);
    let grid: Grid = io::stdin()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .inspect(|(x, c)| match c {
                    'S' => {
                        start = (*x, y);
                    }
                    'E' => {
                        goal = (*x, y);
                    }
                    _ => {}
                })
                .map(|(_, c)| c)
                .collect()
        })
        .collect();

    let costs = calc_costs(&grid, start, goal);
    println!("{}", find_shortcuts(&grid, costs, 20));
}
