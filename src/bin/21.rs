use std::fmt::{Display, Write};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    Some(reachable_in_steps(input, 64))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn reachable_in_steps(input: &str, steps: usize) -> u32 {
    let steps_parity = steps % 2 == 0;
    let map = Map::from(input);
    let mut visited = vec![false; map.width * map.height];
    let mut nodes = Vec::new();
    let mut next_nodes = Vec::new();
    nodes.push(map.starting_position);
    visited[map.starting_position.row * map.width + map.starting_position.col] = true;
    let mut reachable_count = if steps_parity { 1 } else { 0 };
    for step in 1..=steps {
        if nodes.is_empty() {
            break;
        }
        for node in nodes.drain(..) {
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next = node
                    .step(dir.0, dir.1)
                    .filter(|&c| c.row < map.height && c.col < map.width);
                if let Some(next) = next {
                    let index = next.row * map.width + next.col;
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    starting_position: Coordinate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Tile {
    Garden,
    #[default]
    Rock,
}

impl Map {
    pub fn get(&self, coord: Coordinate) -> Tile {
        self.tiles[coord.row * self.width + coord.col]
    }
}

impl Coordinate {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn step(&self, rows: isize, cols: isize) -> Option<Self> {
        Some(Self::new(
            self.row.checked_add_signed(rows)?,
            self.col.checked_add_signed(cols)?,
        ))
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
                        starting_position = Coordinate::new(row, col);
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
                let coord = Coordinate::new(row, col);
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
