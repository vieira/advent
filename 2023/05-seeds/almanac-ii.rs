use std::io;
use std::ops::Range;

type Segment = Range<u64>;
type Path = (Segment, u64);

fn parse_numbers(s: &str) -> Vec<u64> {
    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn parse_seeds(s: &str) -> Vec<Segment> {
    let numbers = parse_numbers(s);
    numbers.iter()
        .enumerate()
        .step_by(2)
        .map(|(i, &start)| {
            let len = numbers[i+1];
            start..start + len
        })
        .collect()
}

fn parse_path(s: &str) -> Path {
    let r = parse_numbers(s);
    let [dest_start, src_start, len] = r[..] else { panic!("bad range") };

    (src_start..src_start + len, dest_start)
}

fn calc_segment(segment: &Segment, path: &Path) -> Segment {
    let (range, dest) = path;
    let Range { start, end } = segment;
    let len = start - range.start;
    *dest + len..*dest + (end - start) + len
}

fn next_segments(start: &Segment, paths: &Vec<Path>) -> Vec<Segment> {
    let mut rest = start.clone();
    let mut segments = vec![];

    for (s, dist) in paths {
        if rest.is_empty() { break };
        if !s.contains(&rest.start) && !rest.contains(&s.start) { continue };

        let src = s.clone();
        let dist = *dist;

        if rest.start <= s.start && rest.end <= s.end {
            segments.push(rest.start..s.start);
            segments.push(calc_segment(&(s.start..rest.end), &(src, dist)));
            rest = 0..0;
            break;
        }

        if rest.start >= s.start && rest.end <= s.end {
            segments.push(calc_segment(&(rest.start..rest.end), &(src, dist)));
            rest = 0..0;
            break;
        }

        if rest.start <= s.start && rest.end >= s.end {
            segments.push(rest.start..s.start);
            segments.push(calc_segment(&(s.start..s.end), &(src, dist)));
            rest = s.end..rest.end;
            continue;
        }

        if rest.start >= s.start && rest.end >= s.end {
            segments.push(calc_segment(&(rest.start..s.end), &(src, dist)));
            rest = s.end..rest.end;
            continue;
        }
    }

    segments.push(rest);
    segments.into_iter().filter(|r| !r.is_empty()).collect()
}

fn find_location(seeds: &[Segment], map: &Vec<Vec<Path>>) -> Vec<Segment> {
    let mut segments = seeds.to_owned();

    for paths in map {
        segments = segments.iter()
            .flat_map(|s| next_segments(s, paths))
            .collect();
    }

    segments
}

fn sort_paths(mut paths: Vec<Path>) -> Vec<Path> {
    paths.sort_by(|(a, _), (b, _)| a.start.cmp(&b.start));
    paths
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut paths = vec![];
    let mut map = vec![];

    let first_line = lines.next().unwrap().unwrap();
    let (_, ids) = first_line.split_once(':').unwrap();
    let seeds = parse_seeds(ids);

    for line in lines {
        let line = line.unwrap();
        match line {
            _ if line.is_empty() => { continue },
            _ if line.ends_with("map:") => {
                if paths.is_empty() { continue };
                map.push(sort_paths(paths));
                paths = vec![];
            },
            range => {
                paths.push(parse_path(&range));
            },
        }
    }
    map.push(sort_paths(paths));

    let min = find_location(&seeds, &map).iter()
        .map(|location| location.start)
        .min()
        .unwrap();

    println!("{min}");
}
