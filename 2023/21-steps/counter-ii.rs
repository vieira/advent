use std::io;
use std::collections::VecDeque;
use std::collections::HashSet;

type Position = (isize, isize);
type Garden = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    position: Position,
    steps: u32,
}

fn parse_input() -> (Garden, Position) {
    let mut start: (isize, isize) = (0, 0);
    let garden = io::stdin().lines()
        .enumerate()
        .map(|(y, line)| {
            let line = line.unwrap();

            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    let x = x as isize;
                    let y = y as isize;

                    match ch {
                        'S' => { start = (x, y); '.' },
                        _ => ch
                    }
                })
                .collect()
        })
        .collect();

    (garden, start)
}

fn successors(position: &Position, garden: &Garden) -> Vec<Position> {
    let size = garden.len() as isize;
    let (x, y) = position;

    [(0, 1), (0, -1_isize), (1, 0), (-1_isize, 0)]
        .iter()
        .filter_map(|(dx, dy)| {
            let nx = (x + dx).rem_euclid(size) as usize;
            let ny = (y + dy).rem_euclid(size) as usize;

            if garden[ny][nx] == '#' { return None }

            Some((x + dx, y + dy))
        })
        .collect()
}

fn search(start: State, garden: &Garden) -> usize {
    let max_steps = start.steps as usize;
    let mut reached = [0, 0];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start]);

    while let Some(state) = queue.pop_back() {
        let State { position, steps } = state;
        let curr_steps = max_steps - steps as usize;

        if steps + 1 == 0 || visited.contains(&position) { continue }
        reached[curr_steps % 2] += 1;

        for next_position in successors(&position, garden) {
            let next_state = State {
                position: next_position,
                steps: steps - 1,
            };
            queue.push_front(next_state);
        }

        visited.insert(position);
    }

    reached[max_steps % 2]
}

fn polyfit(y: &[usize], x: usize) -> usize {
    let a = (y[2] + y[0]) / 2 - y[1];
    let b = y[1] - y[0] - a;
    let c = y[0];

    a * x.pow(2) + b * x + c
}

fn main() {
    let target = 26501365;
    let (garden, start) = parse_input();
    let x = start.0 as usize;
    let size = garden.len();

    let ys: Vec<usize> = (0..3)
        .map(|i| {
            let steps = (x + size * i) as u32;
            let initial = State { position: start, steps };

            search(initial, &garden)
        })
        .collect();

    let plots = polyfit(&ys, (target - x) / size);
    println!("{plots}");
}
