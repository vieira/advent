use std::io;

type Universe = Vec<Vec<char>>;
type Position = (usize, usize);

fn transpose(universe: &Universe) -> Universe {
    let mut transposed = vec![];
    let max_x = universe[0].len();
    let max_y = universe.len();

    for x in 0..max_x {
        let mut line = vec![];
        for y in 0..max_y {
            line.push(universe[y][x]);
        }
        transposed.push(line);
    }

    transposed
}

fn expand_axis(universe: &Universe) -> Universe {
    let mut expanded = vec![];

    for line in universe {
        let has_galaxy = line.iter().any(|ch| *ch == '#');
        if !has_galaxy { expanded.push(line.clone()) };
        expanded.push(line.clone());
    }

    expanded
}

fn expand_universe(universe: &Universe) -> Universe {
    transpose(&expand_axis(&transpose(&expand_axis(universe))))
}

fn galaxies(universe: &Universe) -> Vec<Position> {
    let mut galaxies = vec![];

    for (y, line) in universe.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch != '#' { continue }
            galaxies.push((x, y));
        }
    }

    galaxies
}

fn distance(a: &Position, b: &Position) -> u32 {
    let ax = a.0 as i64;
    let ay = a.1 as i64;
    let bx = b.0 as i64;
    let by = b.1 as i64;

    ((bx - ax).abs() + (by - ay).abs()) as u32
}

fn main() {
    let mut total_distance = 0;
    let universe: Universe = io::stdin().lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let positions = galaxies(&expand_universe(&universe));

    for i in 0..positions.len() - 1 {
        for j in i + 1..positions.len() {
            total_distance += distance(&positions[i], &positions[j]);
        }
    }

    println!("{total_distance}");
}
