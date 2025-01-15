use std::io;
use std::iter::zip;

fn difference(lists: &mut [Vec<i32>; 2]) -> i32 {
    lists.iter_mut().for_each(|l| l.sort());
    let [xs, ys] = lists;

    zip(xs, ys).map(|(x, y)| (*x - *y).abs()).sum()
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

    println!("{}", difference(&mut lists));
}
