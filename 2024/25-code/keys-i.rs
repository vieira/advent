use std::io;
use std::iter::Peekable;

fn parse_object<'a, T>(mut object: Peekable<T>) -> Vec<u8>
where
    T: Iterator<Item = &'a Vec<char>> + Clone + ExactSizeIterator,
{
    let (rows, cols) = (object.len(), object.peek().unwrap().len());

    (0..cols)
        .map(|c| {
            object
                .clone()
                .enumerate()
                .find(|(_, r)| r[c] != '#')
                .map(|(i, _)| i - 1)
                .unwrap_or(rows) as u8
        })
        .collect()
}

fn fits(key: &[u8], lock: &[u8]) -> bool {
    key.iter().enumerate().all(|(i, ks)| lock[i] + ks <= 5)
}

fn count_fits(keys: &[Vec<u8>], locks: &[Vec<u8>]) -> usize {
    keys.iter().map(|k| locks.iter().filter(|l| fits(k, l)).count()).sum()
}

fn main() {
    let mut lines = io::stdin().lines().peekable();
    let mut locks = vec![];
    let mut keys = vec![];

    while lines.peek().is_some() {
        let object: Vec<Vec<char>> = lines
            .by_ref()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();


        if object.first().is_some_and(|f| f.iter().all(|c| *c == '#')) {
            locks.push(parse_object(object.iter().peekable()));
        }

        if object.last().is_some_and(|l| l.iter().all(|c| *c == '#')) {
            keys.push(parse_object(object.iter().rev().peekable()));
        }
    }

    let count = count_fits(&keys, &locks);
    println!("{count}");
}
