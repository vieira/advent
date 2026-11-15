use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

type Button = Vec<usize>;

fn min_presses(goal: &[bool], buttons: &[Button]) -> usize {
    let start: Vec<bool> = goal.iter().map(|_| false).collect();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(0, start)]);

    while let Some((i, state)) = queue.pop_front() {
        if state == goal {
            return i;
        }

        if seen.contains(&state) {
            continue;
        }

        seen.insert(state.clone());

        buttons.iter().for_each(|b| {
            let mut sucessor = state.clone();
            b.iter().for_each(|&j| sucessor[j] = !sucessor[j]);
            queue.push_back((i + 1, sucessor));
        });
    }

    0
}

fn main() {
    let sum: usize = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (lights, rest) = line.split_once(' ').unwrap();
            let (buttons, _) = rest.rsplit_once(' ').unwrap();

            let lights = lights
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(false),
                    '#' => Some(true),
                    _ => None,
                })
                .collect::<Vec<_>>();

            let buttons = buttons
                .split_whitespace()
                .map(|button| {
                    button
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect::<Button>()
                })
                .collect::<Vec<_>>();

            min_presses(&lights, &buttons)
        })
        .sum();

    println!("{sum}");
}
