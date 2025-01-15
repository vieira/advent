use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

type Rules = HashMap<u32, HashSet<u32>>;

fn sort(mut ordering: Vec<u32>, rules: &Rules) -> Option<Vec<u32>> {
    let mut swaps = 0;
    let mut i = 0;
    while i < ordering.len() {
        let num = ordering[i];
        let Some(after) = rules.get(&num) else {
            i += 1;
            continue;
        };
        for (j, num) in &mut ordering[..i].iter().enumerate() {
            if after.contains(num) {
                ordering.swap(i, j);
                swaps += 1;
                i = j;
                break;
            }
        }
        i += 1;
    }

    match swaps {
        0 => None,
        _ => Some(ordering),
    }
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
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter_map(|ordering| sort(ordering, &rules))
        .map(|ordering| ordering[ordering.len() / 2])
        .sum();
    println!("{sum}");
}
