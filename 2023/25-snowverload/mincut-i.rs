use std::io;
use std::collections::{HashMap, HashSet};

type Vertex = usize;
type Edges = HashMap<Vertex, usize>;

struct Graph {
    edges: HashMap<Vertex, Edges>,
    ids: HashMap<String, Vertex>,
    names: HashMap<Vertex, String>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            ids: HashMap::new(),
            names: HashMap::new(),
        }
    }

    fn get_id(&mut self, name: &str) -> Vertex {
        if let Some(idx) = self.ids.get(name) { return *idx };

        let idx = self.ids.len();
        self.ids.insert(name.to_owned(), idx);
        self.names.insert(idx, name.to_owned());
        idx
    }

    fn get_name(&self, v: &Vertex) -> &str {
        self.names.get(v).unwrap()
    }

    fn get_id_from_ids(&mut self, i: &Vertex, j: &Vertex) -> Vertex {
        let s = self.names.get(i).unwrap();
        let t = self.names.get(j).unwrap();
        let st = format!("{s},{t}");
        self.get_id(&st)
    }

    fn add_edges(&mut self, src: &str, dests: &[String]) {
        let u = self.get_id(src);
        let mut u_edges = self.edges.remove(&u).unwrap_or_default();
        for v in dests {
            let v = self.get_id(v);
            let mut v_edges = self.edges.remove(&v).unwrap_or_default();
            u_edges.insert(v, 1);
            v_edges.insert(u, 1);
            self.edges.insert(v, v_edges);
        }
        self.edges.insert(u, u_edges);
    }

    fn get_edges(&self, vertex: &Vertex) -> &Edges {
        self.edges.get(vertex).unwrap()
    }

    fn len(&self) -> usize {
        self.edges.len()
    }

    fn collapse_vertices(&mut self, s: &Vertex, t: &Vertex) {
        let st = self.get_id_from_ids(s, t);
        let s_edges = self.edges.remove(s).unwrap();
        let t_edges = self.edges.remove(t).unwrap();

        let mut st_edges = HashMap::new();

        for (v, w) in s_edges.iter().chain(t_edges.iter()) {
            if v == s || v == t { continue };
            let stv_w = st_edges.get(v).unwrap_or(&0);
            st_edges.insert(*v, stv_w + w);
        }

        for (v, w) in &st_edges {
            let v_edges = self.edges.get_mut(v).unwrap();
            v_edges.remove(s);
            v_edges.remove(t);
            v_edges.insert(st, *w);
        }

        self.edges.insert(st, st_edges);
    }
}

fn parse_line(line: &str) -> (String, Vec<String>) {
    let (u, vs) = line.split_once(':').unwrap();
    (u.to_owned(), vs.split_whitespace().map(|v| v.to_owned()).collect())
}

fn max_connected_vertex(
    graph: &Graph,
    tcv: &mut HashSet<Vertex>,
    q: &mut Vec<(Vertex, usize)>,
) -> Vertex {
    let (v, _) = q.pop().unwrap();
    tcv.insert(v);
    graph.get_edges(&v)
        .iter()
        .filter(|(n, _)| !tcv.contains(*n))
        .for_each(|(n, w)| {
            if let Some(idx) = q.iter().position(|(u, _)| u == n) {
                let (u, uw) = &q[idx];
                q[idx] = (*u, uw + w);
                return;
            }
            q.push((*n, *w));
        });
    q.sort_unstable_by_key(|(_, w)| *w);
    v
}

fn min_cut_phase(graph: &mut Graph, a: &Vertex) -> (usize, Vertex) {
    let mut tcv = HashSet::from([*a]);
    let mut q = vec![(*a, 0)];

    let mut s = *a;
    let mut t = *a;

    while graph.len() != tcv.len() {
        let v = max_connected_vertex(graph, &mut tcv, &mut q);
        s = t;
        t = v;
    }

    let cut_of_phase = graph.get_edges(&t)
        .iter()
        .map(|(_, w)| *w)
        .sum();

    graph.collapse_vertices(&s, &t);

    (cut_of_phase, t)
}

fn cut_of_size(graph: &mut Graph, k: usize) -> (usize, usize) {
    let a = *graph.edges.keys().next().unwrap();
    let g_size = graph.len();
    let mut cut_set = a;

    while graph.len() > 1 {
        let (cut_of_phase, phase_set) = min_cut_phase(graph, &a);
        if cut_of_phase <= k {
            cut_set = phase_set;
            break;
        }
    }

    let s = graph.get_name(&cut_set);
    let s_size = s.split(',').count();
    let t_size = g_size - s_size;

    (s_size, t_size)
}

fn main() {
    let mut graph = Graph::new();

    for line in io::stdin().lines() {
        let (u, vs) = parse_line(&line.unwrap());
        graph.add_edges(&u, &vs);
    }

    let (s, t) = cut_of_size(&mut graph, 3);
    let product = s * t;
    println!("{product}");
}
