use std::{
    collections::{HashSet, VecDeque},
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
    let mut energized = HashSet::new();
    let mut queue = VecDeque::new();
    energized.insert(start);
    queue.push_back((start, start_dir));
    while let Some((cur, dir)) = queue.pop_front() {
        for (next, next_dir) in grid.neighbors(cur, dir) {
            if let Some(tile) = grid.get_tile(next) {
                match (tile, energized.insert(next)) {
                    // If we hit a splitter that was already energized, we know we are entering a loop so we can stop
                    (Tile::HorizontalSplitter, false) | (Tile::VerticalSplitter, false) => {}

                    // Otherwise keep going
                    _ => queue.push_back((next, next_dir)),
                }
            }
        }
    }
    energized.len() as u32
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
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
    ) -> impl Iterator<Item = (Coordinate, Direction)> + '_ {
        std::iter::once(self.get_tile(coord))
            .flatten()
            .flat_map(move |tile| tile.next(dir))
            .filter_map(move |dir| self.move_in_dir(coord, dir).map(|c| (c, dir)))
    }

    pub fn get_tile(&self, coord: Coordinate) -> Option<Tile> {
        self.tiles
            .get(coord.row)
            .and_then(|row| row.get(coord.col))
            .copied()
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
        let mut rows = Vec::new();
        for line in s.split(|&ch| ch == b'\n') {
            if line.is_empty() {
                continue;
            }
            rows.push(
                line.iter()
                    .map(|&ch| {
                        Ok(match ch {
                            b'.' => Tile::Empty,
                            b'\\' => Tile::LeftMirror,
                            b'/' => Tile::RightMirror,
                            b'|' => Tile::VerticalSplitter,
                            b'-' => Tile::HorizontalSplitter,
                            _ => return Err(ParseGridError::InvalidCharacter(ch.into())),
                        })
                    })
                    .collect::<Result<_, _>>()?,
            );
        }
        Ok(Self {
            width,
            height: rows.len(),
            tiles: rows,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
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

    #[test]
    fn test_grid_neighbors() {
        use Direction::{East, North, South, West};
        let grid: Grid = advent_of_code::template::read_file("examples", DAY)
            .parse()
            .unwrap();
        let neighbors_vec = |coord, dir| grid.neighbors(coord, dir).collect::<Vec<_>>();
        assert_eq!(neighbors_vec(coord!(0, 0), North), vec![]);
        assert_eq!(neighbors_vec(coord!(0, 0), West), vec![]);
        assert_eq!(
            neighbors_vec(coord!(0, 0), East),
            vec![(coord!(0, 1), East)]
        );
        assert_eq!(
            neighbors_vec(coord!(0, 1), East),
            vec![(coord!(1, 1), South)]
        );
        assert_eq!(
            neighbors_vec(coord!(1, 2), South),
            vec![(coord!(1, 3), East), (coord!(1, 1), West)]
        );
        assert_eq!(
            neighbors_vec(coord!(1, 4), West),
            vec![(coord!(0, 4), North)]
        );
    }
}