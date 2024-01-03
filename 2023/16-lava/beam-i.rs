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
            energized: HashMap::from([((0, 0), vec![(1, 0)])]),
            heads: vec![((0, 0), (1, 0))],
            size: (0, 0),
        }
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

    contraption.move_heads();

    let energized_count = contraption.energized.len();
    println!("{energized_count}");
}
