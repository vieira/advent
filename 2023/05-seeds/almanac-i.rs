use std::io;
use std::ops::Range;

type Segment = Range<u64>;
type Path = (Segment, u64);

fn parse_numbers(s: &str) -> Vec<u64> {
    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn parse_path(s: &str) -> Path {
    let r = parse_numbers(s);
    let [dest_start, src_start, len] = r[..] else { panic!("bad range") };

    (src_start..src_start + len, dest_start)
}

fn calc_position(position: u64, path: &Path) -> u64 {
    let (range, dest) = path;
    let len = position - range.start;

    dest + len
}

fn next_position(position: u64, paths: &[Path]) -> u64 {
    paths.iter()
        .find(|(src, _)| src.contains(&position))
        .map(|path| calc_position(position, path))
        .unwrap_or(position)
}

fn find_location(start: u64, map: &[Vec<Path>]) -> u64 {
    map.iter().fold(start, |position, paths| next_position(position, paths))
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut paths = vec![];
    let mut map = vec![];

    let first_line = lines.next().unwrap().unwrap();
    let (_, ids) = first_line.split_once(':').unwrap();
    let seeds = parse_numbers(ids);

    for line in lines {
        let line = line.unwrap();
        match line {
            _ if line.is_empty() => { continue },
            _ if line.ends_with("map:") => {
                if paths.is_empty() { continue };
                map.push(paths);
                paths = vec![];
            },
            range => {
                paths.push(parse_path(&range));
            },
        }
    }
    map.push(paths);

    let min = seeds.iter()
        .map(|&seed| find_location(seed, &map))
        .min()
        .unwrap();

    println!("{min}");
}
