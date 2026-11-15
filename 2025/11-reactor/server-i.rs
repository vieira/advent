use std::collections::{HashMap, HashSet};
use std::io;

type Node = String;
type Graph = HashMap<Node, HashSet<Node>>;

fn count_paths(start: &Node, graph: &Graph) -> usize {
    let mut queue = vec![start.to_owned()];
    let mut count = 0;

    while let Some(node) = queue.pop() {
        if node == "out" {
            count += 1;
            continue;
        }

        let Some(dests) = graph.get(&node) else { continue };
        for next in dests {
            queue.push(next.to_owned());
        }
    }

    count
}

fn main() {
    let graph = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (src, dest) = line.split_once(':').unwrap();
            let dest = dest
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<HashSet<_>>();

            (src.to_string(), dest)
        })
        .collect::<HashMap<_, _>>();

    let count = count_paths(&"you".to_owned(), &graph);
    println!("{count}");
}
