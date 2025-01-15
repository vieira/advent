use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

type Rules = HashMap<u32, HashSet<u32>>;

fn is_valid(ordering: &[u32], rules: &Rules) -> bool {
    ordering.iter().enumerate().all(|(i, x)| {
        let Some(after) = rules.get(x) else { return true };
        ordering[..i].iter().all(|y| !after.contains(y))
    })
}

fn main() {
    let mut rules: Rules = HashMap::new();

    let mut lines = io::stdin().lines();

    lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (lhs, rhs) = line.split_once('|').unwrap();
            let lhs: u32 = lhs.parse().unwrap();
            let rhs = rhs.parse().unwrap();

            let mut after = rules.remove(&lhs).unwrap_or_default();
            after.insert(rhs);
            rules.insert(lhs, after);
        });

    let sum: u32 = lines
        .map(|line| line.unwrap())
        .map(|line| {
            line.split(',').map(|x| x.parse().unwrap()).collect::<Vec<u32>>()
        })
        .filter(|ordering| is_valid(ordering, &rules))
        .map(|ordering| ordering[ordering.len() / 2])
        .sum();
    println!("{sum}");
}
