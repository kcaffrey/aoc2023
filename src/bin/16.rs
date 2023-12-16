use std::{
    collections::VecDeque,
    fmt::{Debug, Display, Write},
    str::FromStr,
};

use rayon::iter::{ParallelBridge, ParallelIterator};
use thiserror::Error;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Grid = input.parse().expect("valid input");
    Some(energize_count(&grid, coord!(0, 0), Direction::East))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Grid = input.parse().expect("valid input");
    (0..grid.height)
        .map(|h| (coord!(h, 0), Direction::East))
        .chain((0..grid.height).map(|h| (coord!(h, grid.width - 1), Direction::West)))
        .chain((0..grid.width).map(|w| (coord!(0, w), Direction::South)))
        .chain((0..grid.width).map(|w| (coord!(grid.height - 1, w), Direction::North)))
        .par_bridge()
        .map(|(c, d)| energize_count(&grid, c, d))
        .max()
}

fn energize_count(grid: &Grid, start: Coordinate, start_dir: Direction) -> u32 {
    let mut energized = vec![false; grid.height * grid.width];
    let mut queue = VecDeque::new();
    energized[start.row * grid.width + start.col] = true;
    queue.push_back((start, start_dir, grid.get_tile(start)));
    while let Some((cur, dir, tile)) = queue.pop_front() {
        let mut do_next = |next_dir| {
            let Some(next) = grid.move_in_dir(cur, next_dir) else {
                return;
            };
            let tile = grid.get_tile(next);
            let i = next.row * grid.width + next.col;
            let was_energized = energized[i];
            energized[i] = true;
            match (tile, was_energized) {
                // If we hit a splitter that was already energized, we know we are entering a loop so we can stop
                (Tile::HorizontalSplitter, true) | (Tile::VerticalSplitter, true) => {}

                // Otherwise keep going
                _ => queue.push_back((next, next_dir, tile)),
            }
        };
        use Direction::{East, North, South, West};
        use Tile::{Empty, HorizontalSplitter, LeftMirror, RightMirror, VerticalSplitter};
        match (tile, dir) {
            (Empty, _) => do_next(dir),
            (HorizontalSplitter, East) | (HorizontalSplitter, West) => do_next(dir),
            (VerticalSplitter, North) | (VerticalSplitter, South) => do_next(dir),
            (LeftMirror, East) | (RightMirror, West) => do_next(South),
            (LeftMirror, West) | (RightMirror, East) => do_next(North),
            (LeftMirror, North) | (RightMirror, South) => do_next(West),
            (LeftMirror, South) | (RightMirror, North) => do_next(East),
            (HorizontalSplitter, _) => {
                do_next(West);
                do_next(East);
            }
            (VerticalSplitter, _) => {
                do_next(North);
                do_next(South);
            }
        }
    }
    energized.into_iter().filter(|&e| e).count() as u32
}

struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    RightMirror,
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Grid {
    pub fn get_tile(&self, coord: Coordinate) -> Tile {
        self.tiles[coord.row * self.width + coord.col]
    }

    fn move_in_dir(&self, coord: Coordinate, dir: Direction) -> Option<Coordinate> {
        coord
            .move_in_dir(dir)
            .filter(|&c| c.row < self.height && c.col < self.width)
    }
}

impl Coordinate {
    const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    const fn move_in_dir(self, dir: Direction) -> Option<Self> {
        Some(match dir {
            Direction::North => {
                if self.row == 0 {
                    return None;
                }
                Self::new(self.row - 1, self.col)
            }
            Direction::South => Self::new(self.row + 1, self.col),
            Direction::East => Self::new(self.row, self.col + 1),
            Direction::West => {
                if self.col == 0 {
                    return None;
                }
                Self::new(self.row, self.col - 1)
            }
        })
    }
}

#[derive(Error, Debug)]
enum ParseGridError {
    #[error("invalid tile character: {0}")]
    InvalidCharacter(char),

    #[error("only one row was in the input, expected more than one row")]
    OnlyOneRowFound,
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let width = s
            .iter()
            .position(|&ch| ch == b'\n')
            .ok_or(ParseGridError::OnlyOneRowFound)?;
        let height = s.len() / (width + 1);
        let tiles = s
            .split(|&ch| ch == b'\n')
            .filter(|l| !l.is_empty())
            .flat_map(|line| {
                line.iter().map(|&ch| {
                    Ok(match ch {
                        b'.' => Tile::Empty,
                        b'\\' => Tile::LeftMirror,
                        b'/' => Tile::RightMirror,
                        b'|' => Tile::VerticalSplitter,
                        b'-' => Tile::HorizontalSplitter,
                        _ => return Err(ParseGridError::InvalidCharacter(ch.into())),
                    })
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            width,
            height,
            tiles,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.get_tile(coord!(row, col)))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::RightMirror => f.write_char('/'),
            Tile::LeftMirror => f.write_char('\\'),
            Tile::VerticalSplitter => f.write_char('|'),
            Tile::HorizontalSplitter => f.write_char('-'),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr) => {
        Coordinate::new($x, $y)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
