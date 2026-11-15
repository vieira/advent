use std::io;

fn main() {
    let (_, count) = io::stdin().lines().fold((50, 0), |(pos, count), line| {
        let mut chars = line.as_ref().unwrap().chars();
        let dir = chars.next().unwrap();
        let clicks: i32 = chars.as_str().parse().unwrap();
        let next_pos = match dir {
            'L' => pos - clicks,
            _ => pos + clicks,
        }.rem_euclid(100);

        (next_pos, count + if next_pos == 0 { 1 } else { 0 })
    });

    println!("{count}");
}
