use std::io;

type Grid = Vec<Vec<char>>;
type Position = (isize, isize);
type Direction = (isize, isize);

fn move_box(grid: &mut Grid, (x, y): &Position, (dx, dy): &Direction) -> bool {
    let (mut xn, mut yn) = (x + dx, y + dy);

    while grid[yn as usize][xn as usize] == 'O' {
        (xn, yn) = (xn + dx, yn + dy);
    }

    let (x, y) = (*x as usize, *y as usize);
    let (xn, yn) = (xn as usize, yn as usize);

    if grid[yn][xn] == '#' {
        return false;
    }

    grid[yn][xn] = 'O';
    grid[y][x] = '.';
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
            'O' if move_box(&mut grid, &(xn, yn), &(dx, dy)) => {
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
            if *c != 'O' {
                continue;
            };
            coords += 100 * y + x;
        }
    }

    coords
}

fn main() {
    let mut lines = io::stdin().lines().map(|line| line.unwrap());
    let mut robot = (0, 0);
    let grid: Grid = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '@' => {
                        robot = (x as isize, y as isize);
                        '.'
                    }
                    _ => c,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let moves = lines.collect::<String>();

    let coordinates = gps_coordinates(&simulate(grid, &robot, &moves));
    println!("{coordinates}");
}
