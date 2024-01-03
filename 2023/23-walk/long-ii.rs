use std::io;
use std::cmp;
use std::collections::{HashSet, HashMap};

type Trails = Vec<Vec<char>>;
type Position = (usize, usize);
type NodeId = usize;
type Steps = usize;
type Edge = (Position, Steps);
type Graph = Vec<Vec<(NodeId, Steps)>>;

#[derive(Clone)]
struct Path {
    position: Position,
    visited: HashSet<Position>,
}

struct State {
    id: NodeId,
    visited: usize,
    steps: Steps,
}

struct Ids {
    ids: HashMap<Position, NodeId>,
}

impl Ids {
    fn new() -> Self {
        Ids { ids: HashMap::new() }
    }

    fn get(&mut self, position: &Position) -> NodeId {
        if let Some(idx) = self.ids.get(position) { return *idx };

        let idx = self.ids.len();
        self.ids.insert(*position, idx);
        idx
    }
}

fn is_end(position: &Position, trails: &Trails) -> bool {
    let (_, y) = position;
    *y == trails.len() - 1
}

fn next_paths(path: &Path, trails: &Trails) -> Vec<Path> {
    let (x, y) = path.position;
    let width = trails[0].len();
    let height = trails.len();
    let x = x as isize;
    let y = y as isize;

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .filter_map(|(dx, dy)| {
            if x + dx < 0 || y + dy < 0 { return None };
            let nx = (x + dx) as usize;
            let ny = (y + dy) as usize;

            if nx >= width || ny >= height { return None };
            if trails[ny][nx] == '#' { return None };
            if path.visited.contains(&(nx, ny)) { return None };

            let mut next_path = path.clone();
            next_path.visited.insert((nx, ny));
            next_path.position = (nx, ny);

            Some(next_path)
        })
        .collect()
}

fn find_edges(start: &Position, trails: &Trails) -> Vec<Edge> {
    let initial = Path { position: *start, visited: HashSet::from([*start]) };
    let mut queue = vec![initial];
    let mut edges = vec![];

    while let Some(path) = queue.pop() {
        let Path { position, .. } = path;
        let mut paths = next_paths(&path, trails);

        if paths.len() > 1 && position != *start || is_end(&position, trails) {
            edges.push((position, path.visited.len() - 1));
            continue;
        }

        queue.append(&mut paths);
    }

    edges
}

fn build_graph(start: &Position, trails: &Trails, ids: &mut Ids) -> Graph {
    let mut queue = vec![*start];
    let mut graph: Graph = vec![vec![]; 64];

    while let Some(node) = queue.pop() {
        let node_id = ids.get(&node);

        if !graph[node_id].is_empty() { continue };
        if is_end(&node, trails) { continue };

        let edges = find_edges(&node, trails);

        for edge in &edges {
            let (next_node, _) = edge;
            queue.push(*next_node);
        }

        graph[node_id] = edges
            .iter()
            .map(|(pos, steps)| (ids.get(pos), *steps))
            .collect();
    }

    graph
}

fn is_goal(state: &State, goal: usize, graph: &Graph) -> Option<Steps> {
    let successors = &graph[state.id];
    successors.iter().find(|(n, _)| *n == goal).map(|(_, c)| *c)
}

fn successors<'a>(
    state: &'a State,
    graph: &'a Graph,
) -> impl Iterator<Item=State> + 'a {
    let nodes = &graph[state.id];

    nodes.iter().filter_map(move |(id, steps)| {
        if (state.visited & 1 << id) > 0 { return None };

        let next_state = State {
            id: *id,
            visited: state.visited | 1 << id,
            steps: state.steps + steps,
        };

        Some(next_state)
    })
}

fn search(initial: State, goal: usize, graph: &Graph) -> Steps {
    let mut queue = vec![initial];
    let mut max_steps = 0;

    while let Some(state) = queue.pop() {
        if let Some(c) = is_goal(&state, goal, graph) {
            max_steps = cmp::max(state.steps + c, max_steps);
            continue;
        }

        for next_state in successors(&state, graph) {
            queue.push(next_state);
        }
    }

    max_steps
}

fn main() {
    let trails: Trails = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect();

    let trail_opening = |y: usize| {
        trails[y].iter().position(|&c| c == '.').unwrap()
    };

    let start_x = trail_opening(0);
    let end_y = trails.len() - 1;
    let end_x = trail_opening(end_y);
    let start = (start_x, 0);
    let end = (end_x, end_y);

    let mut ids = Ids::new();
    let graph = build_graph(&start, &trails, &mut ids);

    let initial = State {
        id: ids.get(&start),
        visited: 0,
        steps: 0,
    };
    let goal = ids.get(&end);

    let max_steps = search(initial, goal, &graph);
    println!("{max_steps}");
}
