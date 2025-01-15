use std::io;

fn parse_diskmap(line: &str) -> Vec<i64> {
    let mut id = 0;
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(i, size)| match i % 2 {
            0 => {
                id += 1;
                vec![id - 1; size]
            }
            _ => vec![-1; size],
        })
        .collect()
}

fn compact_disk(mut diskmap: Vec<i64>) -> Vec<i64> {
    let mut last_j = 0;
    for i in (0..diskmap.len()).rev() {
        if diskmap[i] < 0 {
            continue;
        };
        for j in last_j..i {
            if diskmap[j] >= 0 {
                continue;
            };
            diskmap.swap(i, j);
            last_j = j;
            break;
        }
    }
    diskmap
}

fn checksum(diskmap: &[i64]) -> i64 {
    diskmap
        .iter()
        .take_while(|&id| *id >= 0)
        .enumerate()
        .map(|(i, id)| i as i64 * id)
        .sum()
}

fn main() {
    let line = io::stdin().lines().next().unwrap().unwrap();
    println!("{}", checksum(&compact_disk(parse_diskmap(&line))));
}
