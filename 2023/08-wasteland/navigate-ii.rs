use std::io;
use std::cmp;
use std::collections::HashMap;

type Map = HashMap<String, Vec<String>>;

fn gcd(a: usize, b: usize) -> usize {
    let mut num = cmp::max(a, b);
    let mut div = cmp::min(a, b);

    while div != 0 {
        let rem = num % div;
        num = div;
        div = rem;
    }
    num
}

fn lcm(nums: &[usize]) -> usize {
    nums.iter().copied().reduce(|acc, v| acc * v / gcd(acc, v)).unwrap()
}

fn parse_directions(line: &str) -> Vec<u8> {
    line.chars().map(|c| match c { 'L' => 0, _ => 1 }).collect()
}

fn parse_node(line: &str) -> (String, Vec<String>) {
    let (src, dests) = line.split_once('=').unwrap();
    let src = src.trim();
    let dests = dests.trim().replace(['(', ')'], "");
    let (left, right) = dests.split_once(',').unwrap();

    (src.to_string(), vec![left.to_string(), right.trim().to_string()])
}

fn count_steps(map: &Map, dirs: &[u8]) -> usize {
    let mut freqs = HashMap::new();
    let mut positions = map.keys()
        .filter(|p| p.ends_with(&"A"))
        .collect::<Vec<&String>>();

    for (i, dir) in dirs.iter().cycle().enumerate() {
        for position in &mut positions {
            let freq_key = position.to_string();
            *position = &map[&position[..]][*dir as usize];

            if !freq_key.ends_with(&"Z") { continue };
            if freqs.contains_key(&freq_key) { continue };

            freqs.insert(freq_key, i);
        }
        if freqs.len() == positions.len() { break }
    }

    lcm(&freqs.values().copied().collect::<Vec<usize>>())
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut map = HashMap::new();
    let directions = parse_directions(&lines.next().unwrap().unwrap());
    lines.next();

    for line in lines {
        let line = line.unwrap();
        let (src, dest) = parse_node(&line);
        map.insert(src, dest);
    }

    let steps = count_steps(&map, &directions);
    println!("{steps}");
}
