use std::io;
use std::str::FromStr;
use std::fmt::Debug;

type Position = (f64, f64, f64);
type Velocity = (f64, f64, f64);
type Hailstone = (Position, Velocity);
type Vector = (f64, f64, f64, f64);

fn parse_numbers<T: FromStr>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.split(',').map(|n| n.trim().parse::<T>().unwrap()).collect()
}

fn parse_line(line: &str) -> (Position, Velocity) {
    let (position, velocity) = line.split_once(" @ ").unwrap();
    let position = parse_numbers::<f64>(position);
    let velocity = parse_numbers::<f64>(velocity);

    let [x, y, z] = position[..] else { panic!("Invalid position") };
    let [vx, vy, vz] = velocity[..] else { panic!("Invalid velocity") };

    ((x, y, z), (vx, vy, vz))
}

fn gaussian_elimination(m: &mut Vec<Vec<f64>>) -> Vec<f64> {
    let size = m.len();

    for i in 0..size {
        let pivot_row = (i..size).max_by(|&a, &b| {
            m[a][i].abs().partial_cmp(&m[b][i].abs()).unwrap()
        });

        if pivot_row.is_some_and(|r| i != r) {
            let pivot_row = pivot_row.unwrap();
            let t = m[i].clone();
            m[i] = m[pivot_row].clone();
            m[pivot_row] = t;
        }

        for j in i + 1..size {
            let f = m[j][i] / m[i][i];
            m[j] = m[j]
                .iter()
                .enumerate()
                .map(|(k, v)| v - f * m[i][k])
                .collect();
        }
    }

    for i in (0..size).rev() {
        let f = m[i][i];
        m[i] = m[i].iter().map(|v| v / f).collect();

        for j in 0..i {
            let f = m[j][i] / m[i][i];
            m[j] = m[j]
                .iter()
                .enumerate()
                .map(|(k, v)| v - f * m[i][k])
                .collect();
        }
    }

    m.iter_mut().map(|r| r.pop().unwrap()).collect()
}

fn make_matrix(vs: &[Vector]) -> Vec<Vec<f64>> {
    (1..vs.len())
        .map(|i| {
            let (x0, a0, vx0, va0) = vs[i - 1];
            let (x1, a1, vx1, va1) = vs[i];
            vec![
                (va0 - va1),
                (vx1 - vx0),
                (a1 - a0),
                (x0 - x1),
                (x0 * va0 - a0 * vx0 + a1 * vx1 - x1 * va1),
            ]
        })
        .collect()
}

fn main() {
    let hailstones: Vec<Hailstone> = io::stdin()
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect();

    let xy = hailstones
        .iter()
        .take(5)
        .copied()
        .map(|hailstone| {
            let ((x0, y0, _), (vx0, vy0, _)) = hailstone;
            (x0, y0, vx0, vy0)
        })
        .collect::<Vec<Vector>>();

    let xz = hailstones
        .iter()
        .take(5)
        .copied()
        .map(|hailstone| {
            let ((x0, _, z0), (vx0, _, vz0)) = hailstone;
            (x0, z0, vx0, vz0)
        })
        .collect::<Vec<Vector>>();

    let mut mxy = make_matrix(&xy);
    let mut mxz = make_matrix(&xz);

    let [x, y, _vx, _vy] = gaussian_elimination(&mut mxy)[..]
    else { panic!("Failed to find solution for xy") };

    let [_, z, _, _vz] = gaussian_elimination(&mut mxz)[..]
    else { panic!("Failed to find solution for xz") };

    let sum = (x + y + z).round() as i64;
    println!("{sum}");
}
