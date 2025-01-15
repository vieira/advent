use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io;

type Grid = Vec<Vec<char>>;
type Position = (isize, isize);
type Direction = (isize, isize);

#[derive(Debug, Eq, PartialEq)]
struct State {
    position: Position,
    direction: Direction,
    cost: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl State {
    fn new(position: Position) -> Self {
        Self {
            position,
            direction: (1, 0),
            cost: 0,
        }
    }

    fn go(&self, (xn, yn): Position) -> Self {
        let (x, y) = self.position;
        let (dx, dy) = self.direction;
        let (dxn, dyn) = (xn - x, yn - y);

        let cost = self.cost + if (dxn, dyn) == self.direction {
            1
        } else if dxn != dx && dyn != dy {
            1000 + 1
        } else {
            panic!("going back!");
        };

        Self {
            position: (xn, yn),
            direction: (dxn, dyn),
            cost,
        }
    }
}

fn successors(grid: &Grid, deer: &State) -> Vec<State> {
    let (x, y) = deer.position;

    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .filter(|(dx, dy)| (-dx, -dy) != deer.direction)
        .filter_map(|(dx, dy)| {
            let (xn, yn) = (x + dx, y + dy);

            match grid[yn as usize][xn as usize] {
                '#' => None,
                _ => Some(deer.go((xn, yn))),
            }
        })
        .collect()
}

fn search(grid: &Grid, start: Position, goal: Position) -> u32 {
    let key = |state: &State| (state.position, state.direction);
    let mut queue = BinaryHeap::from([State::new(start)]);
    let mut visited = HashSet::new();

    while let Some(state) = queue.pop() {
        if state.position == goal {
            return state.cost;
        }

        for successor in successors(grid, &state) {
            if visited.contains(&key(&successor)) { continue; }
            queue.push(successor);
        }

        visited.insert(key(&state));
    }

    0
}

fn main() {
    let mut deer = (0, 0);
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
                        deer = (*x as isize, y as isize);
                    }
                    'E' => {
                        goal = (*x as isize, y as isize);
                    }
                    _ => {}
                })
                .map(|(_, c)| c)
                .collect()
        })
        .collect();

    let cost = search(&grid, deer, goal);
    println!("{cost}");
}
