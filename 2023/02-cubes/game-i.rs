use std::io;

struct Game {
    id: u8,
    draws: Vec<[u32;3]>,
}

fn parse_line(line: &str) -> Game {
    let (game_id, draws) = line.split_once(':').unwrap();
    let game_id = game_id.split_whitespace().nth(1).unwrap().parse().unwrap();
    let draws = draws.split(';');

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

    Game { id: game_id, draws: game_draws }
}

fn valid_game(game: &Game) -> bool {
    game.draws.iter().all(|rgb| rgb[0] <= 12 && rgb[1] <= 13 && rgb[2] <= 14)
}

fn main() {
    let sum: u32 = io::stdin().lines()
        .map(|line| parse_line(&line.unwrap()))
        .filter(valid_game)
        .map(|game| game.id as u32)
        .sum();
    println!("{sum}");
}
