use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

type Position = (i32, i32);
type PartIdx = usize;
type PartNumbers = HashMap<Position, PartIdx>;
type Gears = HashSet<PartIdx>;

fn number(digits: &[u32]) -> u32 {
    digits.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| v * 10_u32.pow(i as u32))
        .sum()
}

fn adjacents((x, y): Position) -> impl Iterator<Item = Position> {
    (-1..=1).flat_map(move |i| (-1..=1).map(move |j| (x + i, y + j)))
}

fn find_gears((x, y): Position, numbers: &PartNumbers) -> Option<Gears> {
    let gears: HashSet<usize> = adjacents((x, y))
        .filter_map(|(ax, ay)| numbers.get(&(ax, ay)).copied())
        .collect();

    if gears.len() == 2 { Some(gears) } else { None }
}

fn main() {
    let mut numbers = vec![];
    let mut positions = PartNumbers::new();
    let mut wildcards = HashSet::new();

    for (y, line) in io::stdin().lines().enumerate() {
        let y = y as i32;
        let line = line.unwrap();
        let mut digits = vec![];

        for (x, ch) in line.chars().enumerate() {
            let x = x as i32;
            if ch.is_numeric() {
                digits.push(ch.to_digit(10).unwrap());
                positions.insert((x, y), numbers.len());
                continue;
            }

            if !ch.is_numeric() && !digits.is_empty() {
                numbers.push(number(&digits));
                digits = vec![];
            }

            if ch == '*' {
                wildcards.insert((x, y));
            }
        }

        if !digits.is_empty() {
            numbers.push(number(&digits));
        };
    }

    let sum: u32 = wildcards.iter()
        .filter_map(move |&p| find_gears(p, &positions))
        .map(|ids| ids.iter().map(|&id| numbers[id]).collect::<Vec<u32>>())
        .map(|nums| nums.iter().product::<u32>())
        .sum();
    println!("{sum}");
}
