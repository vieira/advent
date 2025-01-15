use std::io;
use std::str;

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

fn has_tree(points: &[Position]) -> bool {
    let mut grid = [[0; MAX_X as usize]; MAX_Y as usize];

    for (x, y) in points {
        let (x, y) = (*x as usize, *y as usize);
        grid[y][x] = b'#';
    }

    grid.iter().any(|row| {
        let s = str::from_utf8(row).unwrap();
        s.contains("########################")
    })
}

fn main() {
    let robots: Vec<Robot> = io::stdin()
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect();

    let Some(time) = (1..).find(|t| {
        let points: Vec<_> = robots.iter().map(|r| position(r, *t)).collect();
        has_tree(&points)
    }) else {
        unreachable!()
    };
    println!("{time}");
}
