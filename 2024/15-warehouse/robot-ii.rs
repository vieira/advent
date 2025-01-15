use std::collections::HashSet;
use std::io;

type Grid = Vec<Vec<char>>;
type Position = (isize, isize);
type Direction = (isize, isize);
type Boxes = HashSet<Position>;

fn boxes_from(grid: &Grid, p: &Position, d: &Direction) -> Option<Boxes> {
    let (dx, dy) = d;
    let mut queue = vec![*p];
    let mut boxes = HashSet::new();

    while let Some((x, y)) = queue.pop() {
        let (xn, yn) = (x + dx, y + dy);

        match grid[yn as usize][xn as usize] {
            ']' => {
                [(xn, yn), (xn - 1, yn)].iter().for_each(|pos| {
                    if !boxes.contains(pos) {
                        queue.push(*pos);
                        boxes.insert(*pos);
                    }
                });
            }
            '[' => {
                [(xn, yn), (xn + 1, yn)].iter().for_each(|pos| {
                    if !boxes.contains(pos) {
                        queue.push(*pos);
                        boxes.insert(*pos);
                    }
                });
            }
            '#' => return None,
            _ => continue,
        }
    }

    Some(boxes)
}

fn push_box(grid: &mut Grid, pos: &Position, dir: &Direction) -> bool {
    let Some(boxes) = boxes_from(grid, pos, dir) else {
        return false;
    };

    let (dx, dy) = dir;
    let mut boxes: Vec<_> = boxes.iter().collect();
    boxes.sort_by_key(|(x, y)| (x * -dx, y * -dy));

    for (x, y) in boxes {
        let (xn, yn) = ((x + dx) as usize, (y + dy) as usize);
        let (x, y) = (*x as usize, *y as usize);
        grid[yn][xn] = grid[y][x];
        grid[y][x] = '.';
    }

    true
}

fn simulate(mut grid: Grid, robot: &Position, moves: &str) -> Grid {
    let (mut x, mut y) = *robot;
    for m in moves.chars() {
        let (dx, dy) = match m {
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            _ => (0, -1),
        };
        let (xn, yn) = (x + dx, y + dy);
        let next_object = grid[yn as usize][xn as usize];

        match next_object {
            '[' | ']' if push_box(&mut grid, &(x, y), &(dx, dy)) => {
                (x, y) = (xn, yn);
            }
            '.' => {
                (x, y) = (xn, yn);
            }
            _ => {}
        }
    }

    grid
}

fn gps_coordinates(grid: &Grid) -> usize {
    let mut coords = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '[' { continue; }
            coords += 100 * y + x;
        }
    }

    coords
}

fn main() {
    let mut lines = io::stdin().lines().map(|line| line.unwrap());
    let mut grid: Grid = vec![];
    let mut robot = (0, 0);
    for (y, row) in lines.by_ref().enumerate() {
        if row.is_empty() {
            break;
        };
        let mut ss = String::from("");
        for (x, c) in row.chars().enumerate() {
            let s = match c {
                '@' => {
                    robot = (2 * x as isize, y as isize);
                    ".."
                }
                'O' => "[]",
                '#' => "##",
                _ => "..",
            };
            ss.push_str(s);
        }
        grid.push(ss.chars().collect::<Vec<_>>())
    }

    let moves = lines.collect::<String>();

    let coordinates = gps_coordinates(&simulate(grid, &robot, &moves));
    println!("{coordinates}");
}
