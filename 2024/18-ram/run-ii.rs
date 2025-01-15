use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::io;
use std::iter::FromIterator;

const SIZE: usize = 71; // 7 in sample

type Position = (usize, usize);
type Grid = HashSet<Position>;

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

            if grid.contains(&(xn, yn)) {
                return None;
            }

            Some(State {
                position: (xn, yn),
                steps: state.steps + 1,
            })
        })
        .collect()
}

fn search(grid: &Grid, initial: State) -> Option<u32> {
    let goal = (SIZE - 1, SIZE - 1);
    let mut queue = VecDeque::from([initial]);
    let mut visited = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if state.position == goal {
            return Some(state.steps);
        }

        if visited.contains(&state.position) {
            continue;
        }
        visited.insert(state.position);

        for successsor in successors(grid, &state) {
            queue.push_back(successsor);
        }
    }

    None
}

fn main() {
    let corrupted: Vec<(usize, Position)> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .enumerate()
        .collect();

    let Err(idx) = corrupted.binary_search_by(|(i, _)| {
        let initial = State {
            position: (0, 0),
            steps: 0,
        };
        let grid = HashSet::from_iter(corrupted[..*i].iter().map(|(_, p)| *p));
        match search(&grid, initial) {
            Some(_) => Ordering::Less,
            None => Ordering::Greater,
        }
    }) else {
        unreachable!();
    };

    let (_, (x, y)) = corrupted[idx - 1];
    println!("{x},{y}");
}
