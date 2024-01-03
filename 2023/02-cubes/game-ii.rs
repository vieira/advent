use std::io;
use std::cmp;

struct Game {
    draws: Vec<[u32;3]>,
}

fn parse_line(line: &str) -> Game {
    let draws = line.split(':').nth(1).unwrap().split(';');

    let game_draws: Vec<[u32;3]> = draws
        .map(|draw| {
            let mut rgb = [0;3];
            for color_count in draw.split(',') {
                let mut color_count = color_count.split_whitespace();
                let count = color_count.next().unwrap().parse().unwrap();
                let color = color_count.next().unwrap();
                let idx = match color { "red" => 0, "green" => 1, _ => 2 };
                rgb[idx] = count;
            }
            rgb
        })
        .collect();

    Game { draws: game_draws }
}

fn min_cubes(game: &Game) -> [u32;3] {
    let mut min_rgb = [0;3];

    for rgb in &game.draws {
        for c in 0..=2 {
            min_rgb[c] = cmp::max(min_rgb[c], rgb[c]);
        }
    }

    min_rgb
}

fn main() {
    let sum: u32 = io::stdin().lines()
        .map(|line| parse_line(&line.unwrap()))
        .map(|game| min_cubes(&game).iter().product::<u32>())
        .sum();
    println!("{sum}");
}
