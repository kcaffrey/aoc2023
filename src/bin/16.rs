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
        for (next, next_dir, tile) in grid.neighbors(cur, dir, tile) {
            let i = next.row * grid.width + next.col;
            let was_energized = energized[i];
            energized[i] = true;
            match (tile, was_energized) {
                // If we hit a splitter that was already energized, we know we are entering a loop so we can stop
                (Tile::HorizontalSplitter, true) | (Tile::VerticalSplitter, true) => {}

                // Otherwise keep going
                _ => queue.push_back((next, next_dir, tile)),
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
    pub fn neighbors(
        &self,
        coord: Coordinate,
        dir: Direction,
        tile: Tile,
    ) -> impl Iterator<Item = (Coordinate, Direction, Tile)> + '_ {
        tile.next(dir).filter_map(move |dir| {
            self.move_in_dir(coord, dir)
                .map(|c| (c, dir, self.get_tile(c)))
        })
    }

    pub fn get_tile(&self, coord: Coordinate) -> Tile {
        self.tiles[coord.row * self.width + coord.col]
    }

    fn move_in_dir(&self, coord: Coordinate, dir: Direction) -> Option<Coordinate> {
        coord
            .move_in_dir(dir)
            .filter(|&c| c.row < self.height && c.col < self.width)
    }
}

impl Tile {
    pub fn next(self, dir: Direction) -> impl Iterator<Item = Direction> {
        let mut next = [None, None, None];
        if self.can_go_straight(dir) {
            next[0] = Some(dir);
        }
        if self.can_turn_left(dir) {
            next[1] = Some(dir.rotate_left());
        }
        if self.can_turn_right(dir) {
            next[2] = Some(dir.rotate_right());
        }
        next.into_iter().flatten()
    }

    const fn can_go_straight(self, dir: Direction) -> bool {
        matches!(
            (self, dir),
            (Self::Empty, _)
                | (Self::VerticalSplitter, Direction::South)
                | (Self::VerticalSplitter, Direction::North)
                | (Self::HorizontalSplitter, Direction::East)
                | (Self::HorizontalSplitter, Direction::West)
        )
    }

    const fn can_turn_left(self, dir: Direction) -> bool {
        matches!(
            (self, dir),
            (Self::VerticalSplitter, Direction::East)
                | (Self::VerticalSplitter, Direction::West)
                | (Self::HorizontalSplitter, Direction::North)
                | (Self::HorizontalSplitter, Direction::South)
                | (Self::LeftMirror, Direction::North)
                | (Self::LeftMirror, Direction::South)
                | (Self::RightMirror, Direction::East)
                | (Self::RightMirror, Direction::West)
        )
    }

    const fn can_turn_right(self, dir: Direction) -> bool {
        matches!(
            (self, dir),
            (Self::VerticalSplitter, Direction::East)
                | (Self::VerticalSplitter, Direction::West)
                | (Self::HorizontalSplitter, Direction::North)
                | (Self::HorizontalSplitter, Direction::South)
                | (Self::LeftMirror, Direction::East)
                | (Self::LeftMirror, Direction::West)
                | (Self::RightMirror, Direction::North)
                | (Self::RightMirror, Direction::South)
        )
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

impl Direction {
    const fn rotate_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    const fn rotate_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
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
