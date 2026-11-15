use std::io;

type Position = (usize, usize);

struct Block {
    positions: Vec<Position>,
    size: usize,
}

impl Block {
    fn new(vec: Vec<Vec<char>>) -> Self {
        Self {
            positions: vec
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(_, &c)| c == '#')
                        .map(move |(x, _)| (x, y))
                })
                .collect(),

            size: if vec.is_empty() {
                0
            } else {
                vec.len() * vec[0].len()
            },
        }
    }

    fn len(&self) -> usize {
        self.positions.len()
    }
}

struct Grid {
    x: usize,
    y: usize,
    blocks: Vec<usize>,
}

impl Grid {
    fn new((x, y): (usize, usize), blocks: Vec<usize>) -> Self {
        Self { x, y, blocks }
    }

    fn area(&self) -> usize {
        self.x * self.y
    }

    fn can_possibly_fit(&self, blocks: &[Block]) -> bool {
        let min_area = self
            .blocks
            .iter()
            .enumerate()
            .map(|(idx, count)| count * blocks[idx].len())
            .sum();

        self.area() >= min_area
    }

    fn can_surely_fit(&self, blocks: &[Block]) -> bool {
        let max_area = self
            .blocks
            .iter()
            .enumerate()
            .map(|(idx, count)| count * blocks[idx].size)
            .sum();

        self.area() >= max_area
    }
}

fn parse_grid(line: &str) -> Grid {
    let (size, ids) = line.split_once(": ").unwrap();
    let (x, y) = size.split_once('x').unwrap();

    let size = (x.parse().unwrap(), y.parse().unwrap());
    let ids = ids.split_whitespace().map(|v| v.parse().unwrap()).collect();

    Grid::new(size, ids)
}

fn parse_input() -> (Vec<Block>, Vec<Grid>) {
    let lines = io::stdin().lines();

    let mut current_block = vec![];
    let mut blocks = vec![];
    let mut grids = vec![];

    for line in lines {
        let line = line.unwrap();
        match line.as_bytes() {
            b"" => {}
            [b'#' | b'.', ..] => current_block.push(line.chars().collect()),
            id if id.ends_with(b":") => {
                if !current_block.is_empty() {
                    blocks.push(Block::new(current_block));
                    current_block = vec![];
                }
            }
            _ => grids.push(parse_grid(&line)),
        }
    }

    if !current_block.is_empty() {
        blocks.push(Block::new(current_block));
    }

    (blocks, grids)
}

fn main() {
    let (blocks, grids) = parse_input();
    let count = grids
        .iter()
        .filter(|grid| grid.can_possibly_fit(&blocks))
        .filter(|grid| grid.can_surely_fit(&blocks))
        .count();

    println!("{count}");
}
