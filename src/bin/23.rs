use std::collections::BinaryHeap;

use tinyvec::ArrayVec;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u16> {
    let (graph, start, goal) = build_graph(input, true);

    // Dijkstras with negative costs to find the maximal path.
    let mut best_so_far = vec![0; graph.vertices];
    let mut queue = BinaryHeap::new();
    queue.push((0, start));
    while let Some((so_far, cur)) = queue.pop() {
        for &(next, cost) in &graph.adjacency[cur] {
            let next_so_far = so_far + cost;
            if next_so_far > best_so_far[next] {
                best_so_far[next] = next_so_far;
                queue.push((next_so_far, next));
            }
        }
    }

    Some(best_so_far[goal])
}

pub fn part_two(input: &str) -> Option<u16> {
    let (mut graph, start, mut goal) = build_graph(input, false);

    // The goal usually only has one connection, so trim it to save on the search space.
    let mut trimmed_length = 0;
    while graph.adjacency[goal].len() == 1 {
        let (new_goal, distance) = graph.adjacency[goal][0];
        trimmed_length += distance;
        graph.adjacency[goal].clear();
        let idx = graph.adjacency[new_goal]
            .iter()
            .position(|&(adj, _)| adj == goal)
            .unwrap();
        graph.adjacency[new_goal].remove(idx);
        goal = new_goal;
    }

    let mut visited = vec![false; graph.vertices];
    visited[start] = true;
    Some(trimmed_length + part_two_recursive_brute_force(&graph, start, goal, &mut visited, 0))
}

fn part_two_recursive_brute_force(
    graph: &Graph,
    cur: usize,
    goal: usize,
    visited: &mut [bool],
    so_far: u16,
) -> u16 {
    if cur == goal {
        return so_far;
    }

    let mut max = 0;
    for &(neighbor, cost) in &graph.adjacency[cur] {
        if !visited[neighbor] {
            visited[neighbor] = true;
            let next_so_far =
                part_two_recursive_brute_force(graph, neighbor, goal, visited, so_far + cost);
            max = max.max(next_so_far);
            visited[neighbor] = false;
        }
    }

    max
}

fn build_graph(input: &str, obey_slopes: bool) -> (Graph, usize, usize) {
    let mut graph = Graph::default();
    let input = input.as_bytes();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    let height = (input.len() + 1) / (width + 1);
    let start = Coordinate::new(0, 1);
    let goal = Coordinate::new(height - 1, width - 2);
    let mut visited = vec![None; width * height];
    visited[1] = Some(0);
    graph.vertices = 1;
    graph.adjacency.resize_with(1, ArrayVec::default);
    let mut stack = vec![(0, start, Coordinate::new(0, 0))];

    // A helper to get valid neighbors.
    let find_neighbors = |cur: Coordinate, prev: Coordinate, obey_slopes: bool| {
        Direction::ALL
            .into_iter()
            .flat_map(|d| cur.move_in_dir(d, width, height).map(|n| (n, d)))
            .filter_map(|(n, d)| {
                if n == prev {
                    return None;
                }
                let index = n.row * (width + 1) + n.col;
                let cell = input[index];
                use Direction::{Down, Left, Right, Up};
                match (cell, d, obey_slopes) {
                    (b'#', _, _) => return None,
                    (_, _, false) | (b'.', _, _) => {}
                    (b'^', Up, _) | (b'>', Right, _) | (b'<', Left, _) | (b'v', Down, _) => {}
                    _ => return None,
                }
                Some(n)
            })
            .collect::<ArrayVec<[_; 4]>>()
    };

    let mut goal_id = 0;
    while let Some((vertex_id, vertex, prev)) = stack.pop() {
        if vertex == goal {
            goal_id = vertex_id;
            continue;
        }
        // For each valid neighbor, we want to find the next vertex to add to the stack for
        // processing.
        let neighbors = find_neighbors(vertex, prev, obey_slopes);
        for neighbor in neighbors {
            if visited[neighbor.row * width + neighbor.col].is_some() {
                continue;
            }
            let mut cur = neighbor;
            let mut prev = vertex;
            let mut distance = 1;
            let (next_vertex, next_prev) = loop {
                let neighbors_ignoring_slopes = find_neighbors(cur, prev, false);
                if neighbors_ignoring_slopes.len() > 1 {
                    // This is a vertex.
                    break (cur, prev);
                }
                let next_neighbors = if obey_slopes {
                    find_neighbors(cur, prev, true)
                } else {
                    neighbors_ignoring_slopes
                };
                if next_neighbors.is_empty() {
                    // We hit the goal node.
                    break (cur, prev);
                }
                visited[cur.row * width + cur.col] = Some(0);
                prev = cur;
                cur = next_neighbors[0];
                distance += 1;
            };
            let next_vertex_map_index = next_vertex.row * width + next_vertex.col;
            let next_vertex_id = if let Some(idx) = visited[next_vertex_map_index] {
                idx
            } else {
                let idx = graph.vertices;
                graph.vertices += 1;
                graph
                    .adjacency
                    .resize_with(graph.vertices, ArrayVec::default);
                visited[next_vertex_map_index] = Some(idx);
                stack.push((idx, next_vertex, next_prev));
                idx
            };
            graph.adjacency[vertex_id].push((next_vertex_id, distance));
            if !obey_slopes {
                graph.adjacency[next_vertex_id].push((vertex_id, distance));
            }
        }
    }

    (graph, 0, goal_id)
}

#[derive(Debug, Default, Clone)]
struct Graph {
    vertices: usize,
    adjacency: Vec<ArrayVec<[(usize, u16); 4]>>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Coordinate {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn move_in_dir(&self, dir: Direction, width: usize, height: usize) -> Option<Coordinate> {
        match dir {
            Direction::Up => (self.row > 0).then(|| Self::new(self.row - 1, self.col)),
            Direction::Down => (self.row + 1 < height).then(|| Self::new(self.row + 1, self.col)),
            Direction::Left => (self.col > 0).then(|| Self::new(self.row, self.col - 1)),
            Direction::Right => (self.col + 1 < width).then(|| Self::new(self.row, self.col + 1)),
        }
    }
}

impl Direction {
    const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
