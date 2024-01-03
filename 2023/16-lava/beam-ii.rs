use std::io;
use std::collections::HashMap;

type Position = (usize, usize);
type Direction = (i32, i32);
type Head = (Position, Direction);

struct Contraption {
    map: Vec<Vec<char>>,
    energized: HashMap<Position, Vec<Direction>>,
    heads: Vec<Head>,
    size: (usize, usize),
}

impl Contraption {
    fn new() -> Self {
        Contraption {
            map: vec![],
            energized: HashMap::new(),
            heads: vec![],
            size: (0, 0),
        }
    }

    fn start_from(&mut self, p: Position, dir: Direction) -> usize {
        self.energized = HashMap::from([(p, vec![dir])]);
        self.heads = vec![(p, dir)];

        self.move_heads();

        self.energized.len()
    }

    fn add_line(&mut self, line: &str) {
        self.map.push(line.chars().collect());

        let (w, h) = &mut self.size;
        *w = line.len();
        *h = self.map.len();
    }

    fn move_head(&self, head: &Head) -> Vec<Head> {
        let (position, direction) = head;
        let (x, y) = position;
        let (dx, dy) = direction;
        let (w, h) = self.size;
        let device = self.map[*y][*x];

        let directions = match device {
            '/' => {
                vec![(-dy, -dx)]
            },
            '\\' =>  {
                vec![(*dy, *dx)]
            },
            '|' if *dx != 0 => {
                vec![(0, 1), (0, -1)]
            },
            '-' if *dy != 0 => {
                vec![(1, 0), (-1, 0)]
            },
            _ => vec![(*dx, *dy)],
        };

        directions
            .iter()
            .filter_map(|(ndx, ndy)| {
                let nx = ndx + *x as i32;
                let ny = ndy + *y as i32;

                if nx >= 0 && ny >= 0 {
                    Some(((nx as usize, ny as usize), (*ndx, *ndy)))
                } else {
                    None
                }
            })
            .filter(|((nx, ny), _)| *nx < w && *ny < h)
            .collect()
    }

    fn move_heads(&mut self) {
        while let Some(head) = self.heads.pop() {
            for next_head in self.move_head(&head) {
                let (position, direction) = next_head;
                if let Some(energized) = self.energized.get_mut(&position) {
                    if energized.contains(&direction) { continue };
                    energized.push(direction);
                } else {
                    self.energized.insert(position, vec![direction]);
                }
                self.heads.insert(0, next_head);
            }
        }
    }
}

fn main() {
    let mut contraption = Contraption::new();

    for line in io::stdin().lines() {
        contraption.add_line(&line.unwrap());
    }

    let (w, h) = contraption.size;

    let top_bottom = (0..w)
        .map(|x| {
            [(0, 1), (h - 1, -1)]
                .iter()
                .map(|(y, dy)| contraption.start_from((x, *y), (0, *dy)))
                .max()
                .unwrap_or(0)
        })
        .max();

    let left_right = (0..h)
        .map(|y| {
            [(0, 1), (w - 1, -1)]
                .iter()
                .map(|(x, dx)| contraption.start_from((*x, y), (*dx, 0)))
                .max()
                .unwrap_or(0)
        })
        .max();

    let max = [top_bottom, left_right]
        .iter()
        .map(|v| v.unwrap_or(0))
        .max()
        .unwrap_or(0);

    println!("{max}");
}
