use std::io;

type Button = Vec<usize>;
type Assignment = (usize, i32, usize);
type Bound = (usize, usize);

const EPSILON: f64 = 1e-6;

fn make_matrix(jolts: &[i32], buttons: &[Button]) -> Vec<Vec<f64>> {
    jolts
        .iter()
        .enumerate()
        .map(|(i, &jolt)| {
            let mut row: Vec<f64> = buttons
                .iter()
                .map(|b| if b.contains(&i) { 1f64 } else { 0f64 })
                .collect();
            row.push(jolt.into());
            row
        })
        .collect()
}

fn gaussian_elimination(mut m: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n_rows = m.len();
    let n_cols = m[0].len();
    let n_vars = n_cols - 1;
    let mut n_pivots = 0;

    for i in 0..n_vars {
        if n_pivots >= n_rows {
            break;
        }

        let pivot_row = (n_pivots..n_rows)
            .filter(|&j| m[j][i].abs() > EPSILON)
            .max_by(|&a, &b| {
                m[a][i].abs().partial_cmp(&m[b][i].abs()).unwrap()
            });

        let Some(pivot_row) = pivot_row else { continue };

        if n_pivots != pivot_row {
            m.swap(n_pivots, pivot_row);
        }

        for j in n_pivots + 1..n_rows {
            let f = m[j][i] / m[n_pivots][i];
            m[j] = m[j]
                .iter()
                .enumerate()
                .map(|(k, v)| v - f * m[n_pivots][k])
                .collect();
        }

        n_pivots += 1;
    }

    for i in (0..n_rows).rev() {
        let Some(c) = m[i].iter().position(|&v| v.abs() > EPSILON) else {
            continue;
        };
        let f = m[i][c];
        m[i] = m[i].iter().map(|v| v / f).collect();

        for j in 0..i {
            let f = m[j][c] / m[i][c];
            m[j] = m[j]
                .iter()
                .enumerate()
                .map(|(k, v)| v - f * m[i][k])
                .collect();
        }
    }

    m
}

fn free_variables(m: &[Vec<f64>]) -> Vec<usize> {
    let n_vars = m[0].len() - 1;

    let basic: Vec<_> = m
        .iter()
        .filter_map(|r| r.iter().take(n_vars).position(|v| v.abs() > EPSILON))
        .collect();

    (0..n_vars).filter(|var| !basic.contains(var)).collect()
}

fn try_assignments(vars: &[Assignment], m: &[Vec<f64>]) -> Option<usize> {
    let n_vars = m[0].len() - 1;
    let cost: usize = vars.iter().map(|(_, v, _)| *v as usize).sum();
    let mut values = vec![];

    for row in m {
        let rhs = row[n_vars];
        let is_zeros = row[..n_vars].iter().all(|&c| c.abs() < EPSILON);

        if is_zeros && rhs.abs() >= EPSILON {
            return None;
        }

        if is_zeros {
            continue;
        }

        let lhs: f64 = vars.iter().map(|(j, v, _)| row[*j] * (*v as f64)).sum();

        let value = rhs - lhs;
        if value < -EPSILON || (value - value.round()).abs() > EPSILON {
            return None;
        }

        values.push(value.round() as usize);
    }

    if values.len() + vars.len() == n_vars {
        Some(cost + values.iter().sum::<usize>())
    } else {
        None
    }
}

fn assignments(m: &[Vec<f64>]) -> usize {
    let rhs = m[0].len() - 1;
    m.iter().map(|r| r[rhs].round() as usize).sum()
}

fn upper_bounds(vars: &[usize], m: &[Vec<f64>]) -> Vec<Bound> {
    let rhs = m[0].len() - 1;

    vars.iter()
        .filter_map(|&v| {
            m.iter()
                .filter(|row| row[v] > 0.0)
                .map(|row| row[rhs])
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .map(|bound| (v, bound as usize))
        })
        .collect()
}

fn search(vars: &[Bound], m: &[Vec<f64>]) -> usize {
    let initial: Vec<Assignment> =
        vars.iter().map(|(id, bound)| (*id, 0, *bound)).collect();

    let mut queue = vec![(initial, 0)];
    let mut best_cost = usize::MAX;

    while let Some((state, n_assigns)) = queue.pop() {
        if n_assigns >= state.len() {
            if let Some(c) = try_assignments(&state, m) {
                if c < best_cost {
                    best_cost = c
                }
            }
            continue;
        }

        let (id, _, bound) = state[n_assigns];

        for i in 0..=bound {
            let mut sucessor = state.clone();
            sucessor[n_assigns] = (id, i as i32, bound);
            queue.push((sucessor, n_assigns + 1));
        }
    }

    best_cost
}

fn main() {
    let sum: usize = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_, line) = line.split_once(' ').unwrap();
            let (buttons, jolts) = line.rsplit_once(' ').unwrap();

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

            let jolts = jolts
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>();

            let m = make_matrix(&jolts, &buttons);
            let rref = gaussian_elimination(m.clone());
            let vars = free_variables(&rref);

            if vars.is_empty() {
                assignments(&rref)
            } else {
                search(&upper_bounds(&vars, &m), &rref)
            }
        })
        .sum();

    println!("{sum}");
}
