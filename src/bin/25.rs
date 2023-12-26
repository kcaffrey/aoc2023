use std::collections::VecDeque;

use fxhash::FxHashMap;
use rand::{seq::SliceRandom, thread_rng};

advent_of_code::solution!(25, 1);

pub fn part_one(input: &str) -> Option<usize> {
    let mut flow: NetworkFlow = parse_graph(input).into();
    let s = 0;
    let mut vertices = (1..flow.vertices).collect::<Vec<_>>();
    vertices.shuffle(&mut thread_rng());
    vertices
        .into_iter()
        .find_map(|t| find_cut_of_size(&mut flow, s, t, 3))
}

fn find_cut_of_size(net: &mut NetworkFlow, s: usize, t: usize, cut: i16) -> Option<usize> {
    net.flow.clear();

    let mut flow = 0;
    while flow <= cut {
        // Find the shortest path from s to t.
        net.pred.fill(None);
        net.queue.clear();
        net.queue.push_back(s);
        let mut seen_vertices = 0;
        while let Some(cur) = net.queue.pop_front() {
            if net.pred[t].is_some() {
                break;
            }
            seen_vertices += 1;
            for &next in &net.adjacency[cur] {
                if next != s && net.pred[next].is_none() && 1 > net.flow(cur, next) {
                    net.pred[next] = Some(cur);
                    net.queue.push_back(next);
                }
            }
        }

        // If there was no path, and the cut was the right size,
        // return an answer.
        if net.pred[t].is_none() {
            return (flow <= cut).then_some(seen_vertices * (net.vertices - seen_vertices));
        }

        // If we found a path, find the min flow along the path that we will use to update the
        // flow for the residual.
        let mut df = i16::MAX;
        let mut cur = t;
        while let Some(prev) = net.pred[cur] {
            df = df.min(1 - net.flow(prev, cur));
            cur = prev;
        }

        // Update the residual flow.
        let mut cur = t;
        while let Some(prev) = net.pred[cur] {
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
    let mut graph_builder = GraphBuilder {
        graph: Graph::default(),
        vertex_ids: [u16::MAX; 26 * 26 * 26],
    };
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
    adjacency: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GraphBuilder {
    graph: Graph,
    vertex_ids: [u16; 26 * 26 * 26],
}

impl GraphBuilder {
    fn get_vertex_id(&mut self, vertex: &str) -> usize {
        let encoded = vertex
            .as_bytes()
            .iter()
            .fold(0, |acc, &ch| acc * 26 + (ch - b'a') as u16) as usize;
        let existing = self.vertex_ids[encoded];
        if existing < u16::MAX {
            return existing as usize;
        }
        let id = self.graph.vertices;
        self.graph.vertices += 1;
        self.graph
            .adjacency
            .resize_with(self.graph.vertices, Default::default);
        self.vertex_ids[encoded] = id as u16;
        id
    }

    fn insert_edge(&mut self, a: usize, b: usize) {
        self.graph.adjacency[a].push(b);
        self.graph.adjacency[b].push(a);
    }

    fn build(self) -> Graph {
        self.graph
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct NetworkFlow {
    vertices: usize,
    adjacency: Vec<Vec<usize>>,
    flow: FxHashMap<(usize, usize), i16>,
    pred: Vec<Option<usize>>,
    queue: VecDeque<usize>,
}

impl NetworkFlow {
    fn flow(&self, a: usize, b: usize) -> i16 {
        self.flow.get(&(a, b)).copied().unwrap_or_default()
    }

    fn add_flow(&mut self, a: usize, b: usize, df: i16) {
        *self.flow.entry((a, b)).or_default() += df;
    }
}

impl From<Graph> for NetworkFlow {
    fn from(graph: Graph) -> Self {
        let mut ret = Self {
            vertices: graph.vertices,
            pred: vec![None; graph.vertices],
            ..Default::default()
        };
        ret.adjacency = graph.adjacency;
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
