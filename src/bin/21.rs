use std::{
    collections::{HashSet, VecDeque},
    fmt::{Display, Write},
};

use itertools::Itertools;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    Some(reachable_in_steps(input, 64))
}

fn reachable_in_steps(input: &str, steps: usize) -> u32 {
    let steps_parity = steps % 2 == 0;
    let map = Map::from(input);
    let mut visited = vec![false; map.width * map.height];
    let mut nodes = Vec::new();
    let mut next_nodes = Vec::new();
    nodes.push(map.starting_position);
    visited[map.starting_position.row as usize * map.width + map.starting_position.col as usize] =
        true;
    let mut reachable_count = if steps_parity { 1 } else { 0 };
    for step in 1..=steps {
        if nodes.is_empty() {
            break;
        }
        for node in nodes.drain(..) {
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = Some(node.step(dir.0, dir.1)).filter(|&c| {
                    c.row >= 0
                        && c.col >= 0
                        && (c.row as usize) < map.height
                        && (c.col as usize) < map.width
                });
                if let Some(next) = next {
                    let index = next.row as usize * map.width + next.col as usize;
                    if !visited[index] && map.get(next) == Tile::Garden {
                        visited[index] = true;
                        if (step % 2 == 0) == steps_parity {
                            reachable_count += 1;
                        }
                        next_nodes.push(next);
                    }
                }
            }
        }
        std::mem::swap(&mut nodes, &mut next_nodes);
    }
    reachable_count
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from(input);

    // Assumptions:
    // Grid is square
    // Starting position is in the center
    // There are clear paths in the cardinal directions
    // There are clear paths from the center of the edges to each other
    //
    // Yuck! Lots of assumptions. A general solution here seems possible but highly annoying.
    //
    // First BFS to find the distance from the start node to every other node, which
    // we can use to compute parity counts.
    let mut from_start = vec![0; map.width * map.height];
    let mut odd_full = 0;
    let mut even_full = 0;
    for (coord, distance) in distance_from_points(&map, [map.starting_position]) {
        from_start[coord.row as usize * map.width + coord.col as usize] = distance;
        if distance % 2 == 0 {
            even_full += 1;
        } else {
            odd_full += 1;
        }
    }

    // We also BFS from the edges and corners so that we can find the parities of the corners that we
    // add and remove to get the final answer.
    let from_edges = distance_from_points(
        &map,
        [
            Coordinate::new(0, 0),
            Coordinate::new(0, map.starting_position.col),
            Coordinate::new(0, map.width as isize - 1),
            Coordinate::new(map.starting_position.row, 0),
            Coordinate::new(map.starting_position.row, map.width as isize - 1),
            Coordinate::new(map.height as isize - 1, 0),
            Coordinate::new(map.height as isize - 1, map.starting_position.col),
            Coordinate::new(map.height as isize - 1, map.width as isize - 1),
        ],
    )
    .collect::<Vec<_>>();

    // We need the parity counts for both the full grid and just the outer corners, which we will use
    // next, so compute the corners here.
    let half_width = map.width / 2;
    let odd_corners = from_edges
        .iter()
        .filter(|&&(c, d)| {
            let from_start_distance = from_start[c.row as usize * map.width + c.col as usize];
            from_start_distance > half_width && d <= half_width && from_start_distance % 2 == 1
        })
        .count();
    let even_corners = from_edges
        .iter()
        .filter(|&&(c, d)| {
            let from_start_distance = from_start[c.row as usize * map.width + c.col as usize];
            from_start_distance > half_width && d <= half_width && from_start_distance % 2 == 0
        })
        .count();

    // The step count is side length/2 + (side length * k)
    // k is how many grids of tiles we can travel before reaching the outer edge.
    const DESIRED_STEPS: usize = 26501365;
    let k = (DESIRED_STEPS - half_width) / map.width;
    debug_assert_eq!(
        DESIRED_STEPS,
        k * map.width + map.width / 2,
        "step count ({}) should be 65 + 131k",
        DESIRED_STEPS
    );

    // We will end up with (k + 1)^2 odd grids, and k^2 even grids.
    // But we need to add in the missing corners around the edge, and
    // subtract out the extra corners.
    let total = (k + 1).pow(2) * odd_full + k.pow(2) * even_full - (k + 1) * odd_corners
        + (k) * even_corners;

    Some(total)
}

fn distance_from_points(
    map: &Map,
    points: impl IntoIterator<Item = Coordinate>,
) -> impl Iterator<Item = (Coordinate, usize)> + '_ {
    let mut distance_from_start = vec![0; map.width * map.height];
    let mut visited = vec![false; map.width * map.height];
    let mut queue = VecDeque::new();
    for start in points {
        queue.push_back((0, start));
        visited[start.row as usize * map.width + start.col as usize] = true;
    }
    while let Some((distance, cur)) = queue.pop_front() {
        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = Some(cur.step(dir.0, dir.1))
                .filter(|&c| {
                    c.row >= 0
                        && c.col >= 0
                        && (c.row as usize) < map.height
                        && (c.col as usize) < map.width
                })
                .filter(|&c| map.get(c) == Tile::Garden);
            if let Some(next) = next {
                let index = next.row as usize * map.width + next.col as usize;
                if !visited[index] {
                    visited[index] = true;
                    distance_from_start[index] = distance + 1;
                    queue.push_back((distance + 1, next));
                }
            }
        }
    }
    (0..map.height)
        .cartesian_product(0..map.width)
        .map(|(row, col)| {
            (
                Coordinate::new(row as isize, col as isize),
                row * map.width + col,
            )
        })
        .filter(move |&(_, index)| visited[index])
        .map(move |(coord, index)| (coord, distance_from_start[index]))
}

// Note: this is unused because it only works for a step count of a few thousand.
// I was mainly using it to sanity check my answer.
#[allow(unused)]
fn part2_brute_force(input: &str, steps: usize) -> u32 {
    let steps_parity = steps % 2 == 0;
    let map = Map::from(input);
    let mut visited = HashSet::new();
    let mut nodes = Vec::new();
    let mut next_nodes = Vec::new();
    nodes.push(map.starting_position);
    visited.insert(map.starting_position);
    let mut reachable_count = if steps_parity { 1 } else { 0 };
    for step in 1..=steps {
        if nodes.is_empty() {
            break;
        }
        for node in nodes.drain(..) {
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = node.step(dir.0, dir.1);
                if visited.insert(next) && map.get_infinite_wrapping(next) == Tile::Garden {
                    if (step % 2 == 0) == steps_parity {
                        reachable_count += 1;
                    }
                    next_nodes.push(next);
                }
            }
        }
        std::mem::swap(&mut nodes, &mut next_nodes);
    }
    reachable_count
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    starting_position: Coordinate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
struct Coordinate {
    row: isize,
    col: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Tile {
    Garden,
    #[default]
    Rock,
}

impl Map {
    pub fn get(&self, coord: Coordinate) -> Tile {
        self.tiles[coord.row as usize * self.width + coord.col as usize]
    }

    pub fn get_infinite_wrapping(&self, coord: Coordinate) -> Tile {
        let row = coord.row.rem_euclid(self.height as isize);
        let col = coord.col.rem_euclid(self.width as isize);
        self.tiles[row as usize * self.width + col as usize]
    }
}

impl Coordinate {
    pub const fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    pub fn step(&self, rows: isize, cols: isize) -> Self {
        Self::new(self.row + rows, self.col + cols)
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let input = input.as_bytes();
        let width = input.iter().position(|&ch| ch == b'\n').unwrap();
        let height = (input.len() + 1) / (width + 1);
        let mut tiles = vec![Tile::Rock; width * height];
        let mut starting_position = Coordinate::default();
        for row in 0..height {
            for col in 0..width {
                match input[row * (width + 1) + col] {
                    b'S' => {
                        tiles[row * width + col] = Tile::Garden;
                        starting_position = Coordinate::new(row as isize, col as isize);
                    }
                    b'.' => {
                        tiles[row * width + col] = Tile::Garden;
                    }
                    _ => {}
                }
            }
        }
        Self {
            tiles,
            width,
            height,
            starting_position,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let coord = Coordinate::new(row as isize, col as isize);
                if coord == self.starting_position {
                    f.write_char('S')?;
                } else {
                    f.write_char(match self.get(coord) {
                        Tile::Garden => '.',
                        Tile::Rock => '#',
                    })?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = reachable_in_steps(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_two_brute_force() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let answers = [
            (6, 16),
            (10, 50),
            (50, 1594),
            // (100, 6536),
            // (500, 167004),
            // (1000, 668697),
            // (5000, 16733044),
        ];
        for (steps, answer) in answers {
            let result = part2_brute_force(input, steps);
            assert_eq!(result, answer, "{} steps should be {}", steps, answer);
        }
    }
}
