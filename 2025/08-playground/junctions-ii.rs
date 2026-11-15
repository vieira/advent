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

fn merge_boxes(boxes: &[Position], dists: &Distances) -> (Position, Position) {
    let mut circuits: Vec<Circuit> = boxes.iter().map(|&p| vec![p]).collect();

    for (_, (b1, b2)) in dists.iter().cycle() {
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

                if circuits.len() <= 1 {
                    return (*b1, *b2);
                }
            }
        }
    }

    unreachable!()
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

    let distances = compute_distances(&boxes);
    let ((x1, _, _), (x2, _, _)) = merge_boxes(&boxes, &distances);
    println!("{}", x1 * x2);
}
