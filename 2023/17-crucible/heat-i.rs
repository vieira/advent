use std::io;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

type Position = (usize, usize);
type Direction = (i32, i32);
type HeatMap = Vec<Vec<u32>>;

#[derive(Clone, Eq, PartialEq)]
struct Trip {
    position: Position,
    direction: Direction,
    loss: u32,
    straight: u32,
    size: (usize, usize),
}

impl Trip {
    fn new(heatmap: &HeatMap) -> Self {
        let width = heatmap[0].len();
        let height = heatmap.len();

        Self {
            position: (0, 0),
            direction: (0, 0),
            straight: 3,
            loss: 0,
            size: (width, height),
        }
    }

    fn heuristic(&self) -> u32 {
        let (x, y) = self.position;
        let (gx, gy) = self.size;

        ((gx - x) + (gy - y)) as u32
    }

    fn cost(&self) -> u32 {
        self.loss + self.heuristic()
    }
}

impl PartialOrd for Trip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Trip {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost().cmp(&self.cost())
    }
}

fn is_goal(trip: &Trip) -> bool {
    let (width, height) = trip.size;

    trip.position == (width - 1, height - 1)
}

fn successors(trip: &Trip, heatmap: &HeatMap) -> Vec<Trip> {
    let (x, y) = trip.position;

    [(1, 0), (0, -1_i32), (0, 1), (-1_i32, 0)].iter()
        .filter_map(|(dx, dy)| {
            let x = x as i32;
            let y = y as i32;
            if x + dx < 0 || y + dy < 0 { return None };

            let nx = (x + dx) as usize;
            let ny = (y + dy) as usize;

            let (width, height) = trip.size;
            let mut straight = trip.straight;

            if nx >= width || ny >= height { return None };

            if trip.direction == (-dx, -dy) { return None };
            if trip.direction == (*dx, *dy) {
                straight -= 1;
                if straight == 0 { return None };
            } else {
                straight = 3;
            }

            let mut t = trip.clone();
            t.position = (nx, ny);
            t.direction = (*dx, *dy);
            t.straight = straight;
            t.loss += heatmap[ny][nx];

            Some(t)
        })
        .collect()
}

fn search(trip: &Trip, heatmap: &HeatMap) -> Option<u32> {
    let mut states = BinaryHeap::from([trip.to_owned()]);
    let mut visited = HashSet::new();

    while let Some(state) = states.pop() {
        if is_goal(&state) {
            return Some(state.loss);
        }

        let status = (state.position, state.direction, state.straight);

        if visited.contains(&status) { continue };
        visited.insert(status);

        for successor in successors(&state, heatmap) {
            states.push(successor);
        }
    }

    None
}

fn main() {
    let mut heatmap = vec![];

    for line in io::stdin().lines() {
        heatmap.push(
            line.unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        );
    }

    let trip = Trip::new(&heatmap);
    let loss = search(&trip, &heatmap).unwrap();
    println!("{loss}");
}
