use std::io;

type Position = (i64, i64, i64);
type Circuit = Vec<Position>;
type Distances = Vec<(i64, (Position, Position))>;

fn distance((x1, y1, z1): &Position, (x2, y2, z2): &Position) -> i64 {
    ((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)).isqrt()
}

fn compute_distances(boxes: &[Position]) -> Distances {
    let mut distances = vec![];

    for (i, b1) in boxes.iter().enumerate() {
        for b2 in boxes[i + 1..].iter() {
            distances.push((distance(b1, b2), (*b1, *b2)));
        }
    }

    distances.sort_by_key(|(d, _)| *d);
    distances
}

fn merge_boxes(distances: &Distances, n: usize) -> Vec<Circuit> {
    let mut circuits: Vec<Circuit> = vec![];

    for (i, (_, (b1, b2))) in distances.iter().enumerate() {
        if i >= n {
            break;
        }

        let idx1 = circuits.iter().position(|c| c.contains(b1));
        let idx2 = circuits.iter().position(|c| c.contains(b2));

        match (idx1, idx2) {
            (Some(idx1), Some(idx2)) if idx1 == idx2 => continue,
            (None, None) => circuits.push(vec![*b1, *b2]),
            (Some(idx1), None) => circuits[idx1].push(*b2),
            (None, Some(idx2)) => circuits[idx2].push(*b1),
            (Some(idx1), Some(idx2)) => {
                let (i, j) = (idx1.min(idx2), idx1.max(idx2));
                let mut c = circuits.swap_remove(j);
                circuits[i].append(&mut c);
            }
        }
    }

    circuits.sort_by_key(|c| c.len());
    circuits
}

fn main() {
    let boxes: Vec<Position> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let coords: Vec<i64> =
                line.split(',').map(|n| n.parse().unwrap()).collect();
            let [x, y, z] = coords[..] else {
                panic!("wrong number of axis in coords")
            };
            (x, y, z)
        })
        .collect();

    let circuits = merge_boxes(&compute_distances(&boxes), 1000);
    let prod: usize = circuits.iter().rev().take(3).map(|c| c.len()).product();
    println!("{prod}");
}
