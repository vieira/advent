use std::collections::{HashMap, HashSet};
use std::io;

type Vertices = HashSet<String>;
type Graph = HashMap<String, Vertices>;

fn max_clique(g: &Graph, r: Vertices, p: Vertices, x: Vertices) -> Vertices {
    let mut cliques = vec![];

    if p.is_empty() && x.is_empty() {
        return r;
    }

    let (mut p, mut x) = (p, x);

    for v in &p.clone() {
        let neigh = g.get(v).unwrap();
        let mut ruv = r.clone();
        ruv.insert(v.to_owned());
        let pnv = p.intersection(neigh).cloned().collect();
        let xnv = x.intersection(neigh).cloned().collect();
        cliques.push(max_clique(g, ruv, pnv, xnv));
        p.remove(v);
        x.insert(v.to_owned());
    }

    cliques
        .iter()
        .max_by_key(|c| c.len())
        .cloned()
        .unwrap_or(HashSet::new())
}

fn main() {
    let mut graph: Graph = HashMap::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (lhs, rhs) = line.split_once('-').unwrap();
        graph.entry(lhs.to_owned()).or_default().insert(rhs.to_owned());
        graph.entry(rhs.to_owned()).or_default().insert(lhs.to_owned());
    }

    let r = HashSet::new();
    let p = graph.keys().cloned().collect();
    let x = HashSet::new();
    let mut clique: Vec<_> = max_clique(&graph, r, p, x).into_iter().collect();
    clique.sort();
    println!("{}", clique.join(","));
}
