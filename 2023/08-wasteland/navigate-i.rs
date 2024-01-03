use std::io;
use std::collections::HashMap;

type Map = HashMap<String, Vec<String>>;

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

fn count_steps(map: &Map, dirs: &[u8], from: &str, to: &str) -> u32 {
    let mut position = from;

    for (i, dir) in dirs.iter().cycle().enumerate() {
        if position == to {
            return i as u32;
        }
        position = &map[position][*dir as usize];
    }

    0
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

    let steps = count_steps(&map, &directions, "AAA", "ZZZ");
    println!("{steps}");
}
