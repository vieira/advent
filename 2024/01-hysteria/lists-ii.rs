use std::io;

fn similarity(lists: &[Vec<i32>; 2]) -> i32 {
    let [xs, ys] = lists;
    xs.iter().map(|x| x * ys.iter().filter(|&y| x == y).count() as i32).sum()
}

fn main() {
    let mut lists: [Vec<i32>; 2] = [vec![], vec![]];

    for line in io::stdin().lines() {
        let line = line.unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .enumerate()
            .for_each(|(i, x)| lists[i].push(x));
    }

    println!("{}", similarity(&lists));
}
