use std::io;
use std::ops::Range;
use std::cmp::Ordering;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};

fn parse_numbers(s: &str) -> Vec<u32> {
    s.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let (start, end) = line.split_once('~').unwrap();

    (parse_numbers(start), parse_numbers(end))
}

trait Intersection {
    fn intersects(&self, other: &Self) -> bool;
}

impl Intersection for Range<u32> {
    fn intersects(&self, other: &Self) -> bool {
        if self.start <= other.start && self.end >= other.end { return true };
        if other.start <= self.start && other.end >= self.end { return true };

        self.contains(&other.start) || self.contains(&(other.end - 1))
    }
}

#[derive(Eq, PartialEq)]
struct Cube {
    id: usize,
    x: Range<u32>,
    y: Range<u32>,
    z: Range<u32>,
}

impl Cube {
    fn new(id: usize, (start, end): (Vec<u32>, Vec<u32>)) -> Self {
        Cube {
            id,
            x: start[0]..end[0] + 1,
            y: start[1]..end[1] + 1,
            z: start[2]..end[2] + 1,
        }
    }

    fn push_above(&mut self, z: u32) {
        let h = self.z.start - z;
        self.z = self.z.start - h..self.z.end - h;
    }

    fn find_intersecting(&self, landed: &[Cube]) -> (u32, Vec<usize>) {
        let mut intersecting = vec![];
        let mut z = 1;

        for cube in landed {
            if cube.z.end < z { break };
            if self.intersects(cube) {
                z = cube.z.end;
                intersecting.push(cube.id);
            }

        }

        (z, intersecting)
    }
}

impl Intersection for Cube {
    fn intersects(&self, other: &Self) -> bool {
        self.x.intersects(&other.x) && self.y.intersects(&other.y)
    }
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> Ordering {
        other.z.end.cmp(&self.z.end)
    }
}

fn intersection<T>(sets: &[&HashSet<T>]) -> HashSet<T>
where
    T: Clone + Eq + Hash,
{
    match sets.len() {
        0 => HashSet::new(),
        _ => sets[1..].iter().fold(sets[0].clone(), |mut acc, set| {
            acc.retain(|v| set.contains(v));
            acc
        }),
    }
}

fn cubes_supporting(cubes: &mut Vec<Cube>) -> HashMap<usize, Vec<usize>> {
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supporting = HashMap::new();
    let mut landed: Vec<Cube> = vec![];
    cubes.sort();

    while let Some(mut cube) = cubes.pop() {
        let (h, intersecting) = cube.find_intersecting(&landed);
        cube.push_above(h);

        let idx = match landed.binary_search(&cube) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        let sets = intersecting
            .iter()
            .filter_map(|idx| supported_by.get(idx))
            .collect::<Vec<&HashSet<usize>>>();

        let mut cube_support = intersection(&sets);

        if intersecting.len() == 1 {
            cube_support.insert(intersecting[0]);
        }

        for supporting_cube in cube_support.iter().copied() {
            let supported = vec![];
            let mut supported = supporting
                .remove(&supporting_cube)
                .unwrap_or(supported);

            supported.push(cube.id);
            supporting.insert(supporting_cube, supported);
        }

        supported_by.insert(cube.id, cube_support);
        landed.insert(idx, cube);
    }

    supporting
}

fn main() {
    let mut cubes: Vec<Cube> = io::stdin()
        .lines()
        .enumerate()
        .map(|(id, line)| Cube::new(id, parse_line(&line.unwrap())))
        .collect();

    let sum: usize = cubes_supporting(&mut cubes)
        .values()
        .map(|v| v.len())
        .sum();
    println!("{sum}");
}
