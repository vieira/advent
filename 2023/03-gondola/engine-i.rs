use std::io;
use std::collections::HashSet;

type Position = (i32, i32);
type Positions = HashSet<Position>;
type Digit = (u32, i32, i32);
type Digits = Vec<Digit>;

fn has_adjacent(digits: &Digits, symbols: &Positions) -> bool {
    for (_, x, y) in digits {
        for i in -1..=1 {
            for j in -1..=1 {
                if symbols.contains(&(x + i, y + j)) { return true }
            }
        }
    }
    false
}

fn number(digits: &Digits) -> u32 {
    digits.iter().rev().enumerate()
        .map(|(i, (v, _, _))| v * 10_u32.pow(i as u32))
        .sum()
}

fn main() {
    let mut numbers = vec![];
    let mut symbols = HashSet::new();

    for (y, line) in io::stdin().lines().enumerate() {
        let y = y as i32;
        let line = line.unwrap();
        let mut digits = vec![];

        for (x, col) in line.chars().enumerate() {
            let x = x as i32;
            if col.is_numeric() {
                digits.push((col.to_digit(10).unwrap(), x, y));
                continue;
            }

            if !col.is_numeric() && !digits.is_empty() {
                numbers.push(digits);
                digits = vec![];
            }

            if col != '.' {
                symbols.insert((x, y));
            }
        }

        if !digits.is_empty() {
            numbers.push(digits);
        };
    }

    let sum: u32 = numbers.iter()
        .filter(|n| has_adjacent(n, &symbols))
        .map(number)
        .sum();

    println!("{sum}");
}
