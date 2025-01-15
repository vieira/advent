use std::collections::HashMap;
use std::io;

type Position = (i32, i32);
type Subsequences = HashMap<Vec<char>, usize>;

fn difference((xa, ya): Position, (xb, yb): Position) -> (i32, i32) {
    (xa - xb, ya - yb)
}

struct Pad {
    position: Position,
    key_position: fn(char) -> Position,
}

impl Pad {
    fn new(key_position: fn(char) -> Position) -> Self {
        Self {
            position: key_position('A'),
            key_position,
        }
    }

    fn press(&mut self, key: char) -> Vec<char> {
        let target = (self.key_position)(key);
        let (dx, dy) = difference(target, self.position);
        let xx = (0..dx.abs()).map(|_| match dx {
            ..0 => '<',
            0.. => '>',
        });
        let yy = (0..dy.abs()).map(|_| match dy {
            ..0 => 'v',
            0.. => '^',
        });

        let empty = (self.key_position)('E');
        let (x, y) = self.position;
        self.position = target;

        if (x, y + dy) == empty || (x + dx, y) != empty && dx < 0 {
            xx.chain(yy).chain(vec!['A']).collect()
        } else {
            yy.chain(xx).chain(vec!['A']).collect()
        }
    }

    fn press_sequence(&mut self, seqs: Subsequences) -> Subsequences {
        let mut seqs_next = HashMap::new();
        for (ss, count) in seqs {
            for sss in ss.iter().map(|k| self.press(*k)) {
                *seqs_next.entry(sss).or_insert(0) += count;
            }
        }

        seqs_next
    }
}

fn numpad(key: char) -> Position {
    match key {
        'E' => (0, 0),
        '0' => (1, 0),
        'A' => (2, 0),
        _ => {
            let number = key.to_digit(10).unwrap() as i32;
            let y = ((number - 1) / 3) + 1;
            let x = (number - 1) % 3;
            (x, y)
        }
    }
}

fn dirpad(key: char) -> Position {
    match key {
        'E' => (0, 1),
        'A' => (2, 1),
        '^' => (1, 1),
        '<' => (0, 0),
        'v' => (1, 0),
        '>' => (2, 0),
        _ => unreachable!(),
    }
}

fn main() {
    let sum: usize = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|code| {
            let mut num = Pad::new(numpad);
            let mut input = HashMap::from([(code.clone(), 1)]);
            input = num.press_sequence(input);

            for _ in 0..25 {
                let mut dir = Pad::new(dirpad);
                input = dir.press_sequence(input);
            }

            let code_number: usize = code[..code.len() - 1]
                .iter()
                .collect::<String>()
                .parse()
                .unwrap();

            let len: usize = input.iter().map(|(s, c)| s.len() * c).sum();
            code_number * len
        })
        .sum();

    println!("{sum}");
}
