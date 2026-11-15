use std::io;

type Position = (i64, i64);

fn max_rectangle(tiles: &[Position]) -> i64 {
    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            tiles[i + 1..]
                .iter()
                .map(move |(x2, y2)| ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1))
        })
        .max()
        .unwrap()
}

fn main() {
    let tiles: Vec<Position> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    println!("{}", max_rectangle(&tiles));
}
