use std::io;
use std::collections::HashSet;

type Position = (usize, usize);
type Garden = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    position: Position,
    steps: u32,
}

fn parse_input() -> (Garden, Position) {
    let mut start: (usize, usize) = (0, 0);
    let garden = io::stdin().lines()
        .enumerate()
        .map(|(y, line)| {
            let line = line.unwrap();

            line.chars()
                .enumerate()
                .map(|(x, ch)| {
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
    let width = garden[0].len() as isize;
    let height = garden.len() as isize;
    let (x, y) = position;
    let x = *x as isize;
    let y = *y as isize;

    [(0, 1), (0, -1_isize), (1, 0), (-1_isize, 0)]
        .iter()
        .filter_map(|(dx, dy)| {
            if x + dx < 0 || y + dy < 0 { return None };
            if x + dx >= width || y + dy >= height { return None };
            let nx = (x + dx) as usize;
            let ny = (y + dy) as usize;

            if garden[ny][nx] == '#' { return None }

            Some((nx, ny))
        })
        .collect()
}

fn search(start: State, garden: &Garden) -> usize {
    let mut reached = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = vec![start];

    while let Some(state) = queue.pop() {
        let State { position, steps } = state;

        if steps == 0 {
            reached.insert(position);
            continue;
        }

        if visited.contains(&state) { continue };
        visited.insert(state);

        for next_position in successors(&position, garden) {
            let next_state = State {
                position: next_position,
                steps: steps - 1,
            };
            queue.push(next_state);
        }
    }

    reached.len()
}

fn main() {
    let (garden, start) = parse_input();

    let initial = State { position: start, steps: 64 };
    let num_plots = search(initial, &garden);
    println!("{num_plots}");
}
