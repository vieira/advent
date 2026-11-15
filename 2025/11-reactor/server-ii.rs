use std::collections::HashMap;
use std::io;

type Graph = HashMap<String, Vec<String>>;
type Cache = HashMap<State, usize>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    name: String,
    has_fft: bool,
    has_dac: bool,
}

impl State {
    fn new(device: &str) -> Self {
        Self {
            name: device.to_string(),
            has_fft: false,
            has_dac: false,
        }
    }
}

fn count_paths(node: &State, graph: &Graph, cache: &mut Cache) -> usize {
    if node.name == "out" {
        return if node.has_fft && node.has_dac { 1 } else { 0 };
    }

    if let Some(c) = cache.get(node) {
        return *c;
    }

    let Some(children) = graph.get(&node.name) else {
        panic!()
    };

    let count = children
        .iter()
        .map(|name| {
            let mut sucessor = node.clone();
            sucessor.name = name.clone();
            match &name[..] {
                "fft" => {
                    sucessor.has_fft = true;
                }
                "dac" => {
                    sucessor.has_dac = true;
                }
                _ => {}
            }
            count_paths(&sucessor, graph, cache)
        })
        .sum();

    cache.insert(node.clone(), count);
    count
}

fn main() {
    let mut cache = HashMap::new();
    let graph = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (src, dest) = line.split_once(':').unwrap();
            let dest = dest
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            (src.to_string(), dest)
        })
        .collect::<HashMap<_, _>>();

    let start = State::new("svr");
    let count = count_paths(&start, &graph, &mut cache);
    println!("{count}");
}
