use std::io;

type Position = (usize, usize);
type Size = (usize, usize);
type Direction = (i32, i32);

struct Rocks {
    all: Vec<Vec<char>>,
    rounded: Vec<Position>,
    size: Size,
}

impl Rocks {
    fn new() -> Self {
        Rocks { all: vec![], rounded: vec![], size: (0, 0) }
    }

    fn add(&mut self, line: &str) {
        let max_x = line.len() - 1;
        let y = self.all.len();

        let mut rocks = vec![];
        for (x, r) in line.chars().enumerate() {
            rocks.push(r);

            if r == 'O' {
                self.rounded.push((x, y));
            }
        }

        self.all.push(rocks);
        self.size = (max_x, y);
    }

    fn sort_by_direction(&mut self, direction: &Direction) {
        self.rounded.sort_by(|(ax, ay), (bx, by)| {
            match direction {
                (0, 1) => by.cmp(ay),
                (-1, 0) => ax.cmp(bx),
                (1, 0) => bx.cmp(ax),
                _ => ay.cmp(by),
            }
        })
    }

    fn tilt(&mut self, direction: &Direction) {
        self.sort_by_direction(direction);

        let &(dx, dy) = direction;
        let (max_x, max_y) = self.size;

        for position in &mut self.rounded {
            let mut xi = position.0 as i32;
            let mut yi = position.1 as i32;

            loop {
                if xi + dx < 0 || yi + dy < 0 { break };
                let (xn, yn) = ((xi + dx) as usize, (yi + dy) as usize);
                if xn > max_x || yn > max_y { break };
                if self.all[yn][xn] != '.' { break };
                xi += dx;
                yi += dy;
            }

            let &mut (x, y) = position;
            let (xn, yn) = (xi as usize, yi as usize);

            if x == xn && y == yn { continue };

            self.all[yn][xn] = self.all[y][x];
            self.all[y][x] = '.';

            *position = (xn, yn);
        }
    }

    fn load(&self) -> usize {
        self.rounded.iter().map(|(_, y)| self.size.1 - y + 1).sum()
    }
}

fn main() {
    let mut rocks = Rocks::new();

    for line in io::stdin().lines() {
        rocks.add(&line.unwrap())
    }

    rocks.tilt(&(0, -1));

    let load = rocks.load();
    println!("{load}");
}
