use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(25, 1);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_graph(input);
    let s = 0;
    (1..graph.vertices).find_map(|t| find_cut_of_size(&graph, s, t, 3))
}

fn find_cut_of_size(graph: &Graph, s: usize, t: usize, cut: i16) -> Option<usize> {
    let mut net: NetworkFlow = graph.into();
    let mut queue = VecDeque::new();

    let mut flow = 0;
    let mut pred = vec![None; net.vertices];
    while flow <= cut {
        // Find the shortest path from s to t.
        pred.fill(None);
        queue.clear();
        queue.push_back(s);
        let mut seen_vertices = 0;
        while let Some(cur) = queue.pop_front() {
            if pred[t].is_some() {
                break;
            }
            seen_vertices += 1;
            for &next in &graph.adjacency[cur] {
                if next != s
                    && pred[next].is_none()
                    && net.capacity(cur, next) > net.flow(cur, next)
                {
                    pred[next] = Some(cur);
                    queue.push_back(next);
                }
            }
        }

        // If there was no path, and the cut was the right size,
        // return an answer.
        if pred[t].is_none() {
            return (flow <= cut).then_some(seen_vertices * (graph.vertices - seen_vertices));
        }

        // If we found a path, find the min flow along the path that we will use to update the
        // flow for the residual.
        let mut df = i16::MAX;
        let mut cur = t;
        while let Some(prev) = pred[cur] {
            df = df.min(net.capacity(prev, cur) - net.flow(prev, cur));
            cur = prev;
        }

        // Update the residual flow.
        let mut cur = t;
        while let Some(prev) = pred[cur] {
            net.add_flow(prev, cur, df);
            net.add_flow(cur, prev, -df);
            cur = prev;
        }

        // Update our current max flow (min cut).
        flow += df;
    }
    None
}

fn parse_graph(input: &str) -> Graph {
    let mut graph_builder = GraphBuilder::default();
    for line in input.lines() {
        let (vertex, adjacent) = line.split_once(": ").unwrap();
        let vertex = graph_builder.get_vertex_id(vertex);
        for a in adjacent.split_whitespace() {
            let a = graph_builder.get_vertex_id(a);
            graph_builder.insert_edge(vertex, a);
        }
    }
    graph_builder.build()
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Graph {
    vertices: usize,
    adjacency: Vec<HashSet<usize>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct GraphBuilder<'a> {
    graph: Graph,
    vertex_ids: HashMap<&'a str, usize>,
}

impl<'a> GraphBuilder<'a> {
    fn get_vertex_id<'b>(&'b mut self, vertex: &'a str) -> usize
    where
        'a: 'b,
    {
        *self.vertex_ids.entry(vertex).or_insert_with(|| {
            let id = self.graph.vertices;
            self.graph.vertices += 1;
            self.graph
                .adjacency
                .resize_with(self.graph.vertices, Default::default);
            id
        })
    }

    fn insert_edge(&mut self, a: usize, b: usize) {
        self.graph.adjacency[a].insert(b);
        self.graph.adjacency[b].insert(a);
    }

    fn build(self) -> Graph {
        self.graph
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NetworkFlow {
    vertices: usize,
    capacity: Vec<i16>,
    flow: Vec<i16>,
}

impl NetworkFlow {
    fn capacity(&self, a: usize, b: usize) -> i16 {
        self.capacity[a * self.vertices + b]
    }

    fn flow(&self, a: usize, b: usize) -> i16 {
        self.flow[a * self.vertices + b]
    }

    fn add_flow(&mut self, a: usize, b: usize, df: i16) {
        self.flow[a * self.vertices + b] += df;
    }
}

impl From<&Graph> for NetworkFlow {
    fn from(graph: &Graph) -> Self {
        let mut ret = Self {
            vertices: graph.vertices,
            capacity: vec![0; graph.vertices * graph.vertices],
            flow: vec![0; graph.vertices * graph.vertices],
        };
        for v0 in 0..ret.vertices {
            for &v1 in &graph.adjacency[v0] {
                ret.capacity[v0 * ret.vertices + v1] = 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
