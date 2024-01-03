use std::io;

type Valley = Vec<Vec<char>>;

fn parse_input() -> Vec<Valley> {
    let mut valleys = vec![];
    let mut valley: Valley = vec![];

    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            valleys.push(valley);
            valley = vec![];
            continue;
        }
        valley.push(line.chars().collect());
    }
    valleys.push(valley);

    valleys
}

fn transpose(valley: &Valley) -> Valley {
    let mut transposed = vec![];
    let max_x = valley[0].len();
    let max_y = valley.len();

    for x in 0..max_x {
        let mut line = vec![];
        for y in 0..max_y {
            line.push(valley[y][x]);
        }
        transposed.push(line);
    }

    transposed
}

fn is_reflection(valley: &Valley, idx: usize) -> bool {
    valley.iter()
        .all(|row| {
            (0..idx)
                .rev()
                .zip(idx..row.len())
                .all(|(i, j)| row[i] == row[j])
        })
}

fn find_reflection(valley: &Valley) -> Option<usize> {
    let size = valley.first().unwrap().len();

    (1..size).find(|&i| is_reflection(valley, i))
}

fn main() {
    let valleys = parse_input();
    let sum: usize = valleys.iter()
        .map(|v| {
            find_reflection(v)
                .or_else(|| find_reflection(&transpose(v)).map(|r| 100 * r))
                .unwrap()
        })
        .sum();

    println!("{sum}");
}
