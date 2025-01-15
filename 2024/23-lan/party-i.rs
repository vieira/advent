use std::collections::{HashMap, HashSet};
use std::io;

type Graph = HashMap<String, Vec<String>>;

fn t_networks(graph: &Graph) -> HashSet<Vec<String>> {
    let mut t_nets = HashSet::new();

    for (n, nodes) in graph.iter().filter(|(k, _)| k.starts_with("t")) {
        for (i, n1) in nodes[..nodes.len() - 1].iter().enumerate() {
            for n2 in &nodes[i + 1..] {
                if !graph.get(n1).is_some_and(|edges| edges.contains(n2)) {
                    continue;
                }
                let mut t_net: Vec<_> =
                    [n, n1, n2].iter().map(|s| s.to_string()).collect();
                t_net.sort();
                t_nets.insert(t_net);
            }
        }
    }

    t_nets
}

fn main() {
    let mut graph: Graph = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (lhs, rhs) = line.split_once('-').unwrap();
        graph.entry(lhs.to_owned()).or_default().push(rhs.to_owned());
        graph.entry(rhs.to_owned()).or_default().push(lhs.to_owned());
    }

    let count = t_networks(&graph).len();
    println!("{count}");
}
