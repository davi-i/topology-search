use std::collections::VecDeque;

pub struct Graph<T> {
    matrix: Vec<bool>,
    info: Vec<T>,
}

impl<T> Graph<T> {
    pub fn new(nodes: Vec<T>, edges: &mut [(usize, usize)]) -> Self {
        edges.sort_unstable();
        let mut edges = edges.into_iter().peekable();
        let matrix = (0..nodes.len())
            .flat_map(|i| (0..nodes.len()).map(move |j| (i, j)))
            .map(|edge| edges.next_if_eq(&&edge).is_some())
            .collect();

        Graph {
            info: nodes,
            matrix,
        }
    }

    pub fn topology_search(&self) -> VecDeque<&T> {
        TopologySearch::search(self)
    }

    pub fn matrix(&self) -> impl Iterator<Item = &[bool]> {
        self.matrix.chunks(self.info.len())
    }

    pub fn nodes_from(&self, node: usize) -> impl Iterator<Item = usize> + '_ {
        let n_nodes = self.info.len();
        self.matrix[node * n_nodes..(node + 1) * n_nodes]
            .iter()
            .enumerate()
            .filter(|(_, &edge)| edge)
            .map(|(node, _)| node)
    }

    pub fn start_nodes(&self) -> impl Iterator<Item = usize> + '_ {
        let n_nodes = self.info.len();
        (0..n_nodes).filter(move |node| (0..n_nodes).all(move |i| !self.matrix[n_nodes * i + node]))
    }
}

struct TopologySearch<'a, T> {
    visited: Vec<bool>,
    list: VecDeque<&'a T>,
    graph: &'a Graph<T>,
}

impl<'a, T> TopologySearch<'a, T> {
    fn search(graph: &'a Graph<T>) -> VecDeque<&'a T> {
        let mut data = Self {
            visited: vec![false; graph.info.len()],
            list: VecDeque::with_capacity(graph.info.len()),
            graph,
        };

        for node in graph.start_nodes() {
            data.visit(node);
        }

        data.list
    }

    fn visit(&mut self, node: usize) {
        if !self.visited[node] {
            self.visited[node] = true;

            for node in self.graph.nodes_from(node) {
                self.visit(node)
            }
            self.list.push_front(&self.graph.info[node])
        }
    }
}
