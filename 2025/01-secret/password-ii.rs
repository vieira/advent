use std::io;

fn next_position(pos: i32, dir: char, clicks: i32) -> (i32, i32) {
    let turns = clicks / 100;
    let rem_clicks = clicks % 100;
    let jump_pos = match dir {
        'L' => pos - rem_clicks,
        _ => pos + rem_clicks
    };
    let next_pos = jump_pos.rem_euclid(100);
    let zero_crosses = if pos != 0 && (next_pos != jump_pos || next_pos == 0) {
        turns + 1
    } else {
        turns
    };

    (next_pos, zero_crosses)
}

fn main() {
    let (_, count) = io::stdin().lines().fold((50, 0), |(pos, count), line| {
        let mut chars = line.as_ref().unwrap().chars();
        let dir = chars.next().unwrap();
        let clicks: i32 = chars.as_str().parse().unwrap();
        let (next_pos, zero_crosses) = next_position(pos, dir, clicks);

        (next_pos, count + zero_crosses)
    });

    println!("{count}");
}
