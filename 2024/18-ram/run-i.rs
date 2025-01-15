use std::convert::TryInto;
use std::collections::{HashSet, VecDeque};
use std::io;

const BYTES: usize = 1024; // 12 in sample
const SIZE: usize = 71; // 7 in sample

type Grid = Vec<Vec<bool>>;
type Position = (usize, usize);

struct State {
    position: Position,
    steps: u32,
}

fn successors(grid: &Grid, state: &State) -> Vec<State> {
    let (x, y) = state.position;

    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .filter_map(|(dx, dy)| {
            let xn: usize = (x as isize + dx).try_into().ok()?;
            let yn: usize = (y as isize + dy).try_into().ok()?;

            if xn >= SIZE || yn >= SIZE {
                return None;
            }

            if grid[y][x] {
                return None;
            }

            Some(State { position: (xn, yn), steps: state.steps + 1 })
        })
        .collect()
}

fn search(grid: &Grid, initial: State) -> u32 {
    let goal = (SIZE - 1, SIZE - 1);
    let mut queue = VecDeque::from([initial]);
    let mut visited = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if state.position == goal {
            return state.steps;
        }

        if visited.contains(&state.position) {
            continue;
        }
        visited.insert(state.position);

        for successsor in successors(grid, &state) {
            queue.push_back(successsor);
        }
    }

    0
}

fn main() {
    let mut grid: Grid = vec![vec![false; SIZE]; SIZE];
    let initial = State { position: (0, 0), steps: 0 };

    io::stdin()
        .lines()
        .take(BYTES)
        .map(|line| line.unwrap())
        .for_each(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            grid[y][x] = true;
        });

    let steps = search(&grid, initial);
    println!("{steps}");
}
