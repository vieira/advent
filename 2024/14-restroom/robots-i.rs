use std::io;

type Position = (i32, i32);
type Velocity = (i32, i32);
type Robot = (Position, Velocity);

const MAX_X: i32 = 101;
const MAX_Y: i32 = 103;

fn parse_line(line: &str) -> Robot {
    let [p, v] = line
        .split(' ')
        .map(|s| {
            let (_, vs) = s.split_once('=').unwrap();
            let (x, y) = vs.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<(i32, i32)>>()[..]
    else {
        unreachable!()
    };

    (p, v)
}

fn position(robot: &Robot, t: i32) -> Position {
    let ((x, y), (vx, vy)) = robot;
    let xn = (x + t * vx).rem_euclid(MAX_X);
    let yn = (y + t * vy).rem_euclid(MAX_Y);

    (xn, yn)
}

fn quadrant((x, y): &Position) -> Option<usize> {
    let (mid_x, mid_y) = (MAX_X / 2, MAX_Y / 2);
    let q0 = (0..mid_x, 0..mid_y);
    let q1 = (mid_x + 1..MAX_X, 0..mid_y);
    let q2 = (0..mid_x, mid_y + 1..MAX_Y);
    let q3 = (mid_x + 1..MAX_X, mid_y + 1..MAX_Y);

    [q0, q1, q2, q3]
        .iter()
        .enumerate()
        .find(|(_, (xx, yy))| xx.contains(x) && yy.contains(y))
        .map(|(i, _)| i)
}

fn main() {
    let mut quadrants = [0; 4];
    io::stdin()
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .filter_map(|robot| quadrant(&position(&robot, 100)))
        .for_each(|q| { quadrants[q] += 1; });

    let factor: u32 = quadrants.iter().product();
    println!("{factor}");
}
