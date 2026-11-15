use std::io;

type Position = (i64, i64);

fn is_line_in_polygon(line: (Position, Position), poly: &[Position]) -> bool {
    let ((x1, y1), (x2, y2)) = line;
    let n = poly.len();
    let min = if y1 == y2 { x1.min(x2) } else { y1.min(y2) };
    let max = if y1 == y2 { x1.max(x2) } else { y1.max(y2) };

    let mut crossings: Vec<i64> = vec![];
    let mut intervals: Vec<Position> = vec![];

    let mut add = |ad: i64, bd: i64, sd: i64, ao: i64, bo: i64| {
        if ad == bd && ad == sd {
            intervals.push((ao.min(bo), ao.max(bo)));
        } else if sd >= ad.min(bd) && sd < ad.max(bd) {
            crossings.push(ao);
        }
    };

    for i in 0..n {
        let (ax, ay) = poly[i];
        let (bx, by) = poly[(i + 1) % n];

        if y1 == y2 {
            add(ay, by, y1, ax, bx);
        } else {
            add(ax, bx, x1, ay, by);
        }
    }

    crossings.sort_unstable();
    for pair in crossings.chunks(2) {
        intervals.push((pair[0], pair[1]));
    }
    if intervals.is_empty() {
        return false;
    }

    intervals.sort_by_key(|&(a, _)| a);
    let mut merged: Vec<Position> = vec![];
    for (a, b) in intervals {
        if let Some(last) = merged.last_mut() {
            if a <= last.1 + 1 && b > last.1 {
                last.1 = b;
                continue;
            }
        }
        merged.push((a, b));
    }

    merged.into_iter().any(|(a, b)| min >= a && max <= b)
}

fn is_rect_in_polygon(rect: &(Position, Position), poly: &[Position]) -> bool {
    let ((x1, y1), (x2, y2)) = *rect;
    let (x_min, y_min) = (x1.min(x2), y1.min(y2));
    let (x_max, y_max) = (x1.max(x2), y1.max(y2));

    [
        ((x_min, y_min), (x_max, y_min)),
        ((x_max, y_min), (x_max, y_max)),
        ((x_max, y_max), (x_min, y_max)),
        ((x_min, y_max), (x_min, y_min)),
    ]
    .iter()
    .all(|&edge| is_line_in_polygon(edge, poly))
}

fn max_rectangle(poly: &[Position]) -> i64 {
    poly.iter()
        .enumerate()
        .flat_map(|(i, &p1)| {
            poly[i + 1..].iter().filter_map(move |&p2| {
                is_rect_in_polygon(&(p1, p2), poly).then_some({
                    let (x1, y1) = p1;
                    let (x2, y2) = p2;
                    ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1)
                })
            })
        })
        .max()
        .unwrap()
}

fn main() {
    let tiles: Vec<Position> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    println!("{}", max_rectangle(&tiles));
}
