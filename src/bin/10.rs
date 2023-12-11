use std::{collections::HashSet, str::FromStr};

use either::Either;
use tinyvec::ArrayVec;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let field = input.parse::<Field>().expect("valid input");
    field
        .find_definite_loop_position()
        .map(|(_, distance)| distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut field = input.parse::<Field>().expect("valid input");

    // Figure out where the loop is, and construct a new binary map of loop/not loop.
    let (loop_position, _) = field.find_definite_loop_position()?;
    let mut is_in_loop = vec![false; field.tiles.len()];
    is_in_loop[field.index(loop_position)?] = true;
    is_in_loop[field.index(field.starting_position)?] = true;
    let mut cur = ArrayVec::<[(Position, Position); 2]>::new();
    cur.extend(
        field
            .pipe_neighbors(loop_position)
            .map(|n| (n, loop_position)),
    );
    if cur.len() != 2 {
        return None;
    }
    while !cur.is_empty() {
        let mut next = ArrayVec::<[(Position, Position); 2]>::new();
        for (pos, from_pos) in cur
            .iter()
            .flat_map(|&(pos, last_pos)| field.next(pos, last_pos))
        {
            let Some(index) = field.index(from_pos) else {
                continue;
            };
            if !is_in_loop[index] {
                is_in_loop[index] = true;
                if pos != field.starting_position {
                    next.push((pos, from_pos));
                }
            }
        }
        cur = next;
    }

    // Figure out the missing tile for the start position to make things easier later on.
    let starting_tile = Tile::pipes().find(|&pipe| {
        pipe.neighbors(field.starting_position).all(|pos| {
            let is_in_loop = field
                .index(pos)
                .map(|index| is_in_loop[index])
                .unwrap_or(false);
            let is_connected = field
                .pipe_neighbors(pos)
                .any(|p| p == field.starting_position);
            is_in_loop && is_connected
        })
    })?;
    if let Some(cell) = field.get_mut(field.starting_position) {
        *cell = starting_tile;
    } else {
        return None;
    }

    // Create a helper that will let us determine the size of a connected region bounded
    // by "loop pipes".
    let mut closed = HashSet::new();
    let mut flood_fill = |position: Position| -> u32 {
        if closed.contains(&position) {
            return 0;
        }
        let mut open = vec![position];
        closed.insert(position);
        let mut tile_count = 0;
        while let Some(cur) = open.pop() {
            tile_count += 1;
            for dir in Direction::cardinal() {
                let next = cur.go(dir);
                if let Some(index) = field.index(next) {
                    if !is_in_loop[index] && !closed.contains(&next) {
                        open.push(next);
                        closed.insert(next);
                    }
                }
            }
        }
        tile_count
    };

    // Find a spot on the loop where the inside/outside direction is known.
    // We can start from the north on the column where the definite loop position is,
    // and go south until we hit a loop tile. Then we know the direction north from
    // that tile is "outside" and south is "inside".
    // From there, we will walk clockwise direction around the loop, so we always know the
    // outside direction.
    let row = (0..field.rows).find(|&row| {
        field
            .index(Position {
                row: row as isize,
                col: loop_position.col,
            })
            .filter(|&index| is_in_loop[index])
            .is_some()
    })?;
    let start = Position {
        row: row as isize,
        col: loop_position.col,
    };
    let mut cur = start;
    let mut outside_dir = match field.get(start) {
        // Due to how the algorithm below works, we need to pretend
        // we were coming from the south, so the outside is "west",
        // in order to get properly rotated to north when we move.
        Some(Tile::SouthEastPipe) => Direction::West,
        _ => Direction::North,
    };
    let mut inside_count = 0;
    loop {
        for inside_pos in field.inside_neighbors(cur, outside_dir).filter(|&pos| {
            field
                .index(pos)
                .filter(|&index| is_in_loop[index])
                .is_none()
        }) {
            inside_count += flood_fill(inside_pos);
        }
        let (next_dir, next_outside) = match (field.get(cur)?, outside_dir) {
            (Tile::HorizontalPipe, Direction::North) => (Direction::East, Direction::North),
            (Tile::HorizontalPipe, Direction::South) => (Direction::West, Direction::South),
            (Tile::VerticalPipe, Direction::East) => (Direction::South, Direction::East),
            (Tile::VerticalPipe, Direction::West) => (Direction::North, Direction::West),
            (Tile::NorthEastPipe, Direction::South) => (Direction::North, Direction::West),
            (Tile::NorthEastPipe, _) => (Direction::East, Direction::North),
            (Tile::NorthWestPipe, Direction::East) => (Direction::West, Direction::South),
            (Tile::NorthWestPipe, _) => (Direction::North, Direction::West),
            (Tile::SouthEastPipe, Direction::West) => (Direction::East, Direction::North),
            (Tile::SouthEastPipe, _) => (Direction::South, Direction::East),
            (Tile::SouthWestPipe, Direction::North) => (Direction::South, Direction::East),
            (Tile::SouthWestPipe, _) => (Direction::West, Direction::South),
            _ => unreachable!(),
        };
        let next_pos = cur.go(next_dir);
        if next_pos == start {
            break;
        }
        cur = next_pos;
        outside_dir = next_outside;
    }

    Some(inside_count)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
    Ground,
    StartingPosition,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
struct Position {
    row: isize,
    col: isize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Field {
    tiles: Vec<Tile>,
    rows: usize,
    cols: usize,
    starting_position: Position,
}

impl Field {
    pub fn get(&self, position: Position) -> Option<Tile> {
        self.tiles.get(self.index(position)?).copied()
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut Tile> {
        let index = self.index(position)?;
        self.tiles.get_mut(index)
    }

    pub fn index(&self, position: Position) -> Option<usize> {
        if position.row < 0
            || position.col < 0
            || position.row >= self.rows as isize
            || position.col >= self.cols as isize
        {
            return None;
        }
        let index = self.cols as isize * position.row + position.col;
        Some(index as usize)
    }

    pub fn next(
        &self,
        pos: Position,
        last_pos: Position,
    ) -> impl Iterator<Item = (Position, Position)> {
        let mut next = ArrayVec::<[(Position, Position); 4]>::new();
        use Tile::*;
        match (self.get(pos), pos.direction_from(last_pos)) {
            (Some(StartingPosition), _) => {
                for dir in Direction::cardinal() {
                    next.push((pos.go(dir), pos));
                }
            }
            (Some(HorizontalPipe), Direction::East) => next.push((pos.go(Direction::West), pos)),
            (Some(HorizontalPipe), Direction::West) => next.push((pos.go(Direction::East), pos)),
            (Some(VerticalPipe), Direction::North) => next.push((pos.go(Direction::South), pos)),
            (Some(VerticalPipe), Direction::South) => next.push((pos.go(Direction::North), pos)),
            (Some(NorthEastPipe), Direction::North) => next.push((pos.go(Direction::East), pos)),
            (Some(NorthEastPipe), Direction::East) => next.push((pos.go(Direction::North), pos)),
            (Some(NorthWestPipe), Direction::North) => next.push((pos.go(Direction::West), pos)),
            (Some(NorthWestPipe), Direction::West) => next.push((pos.go(Direction::North), pos)),
            (Some(SouthWestPipe), Direction::South) => next.push((pos.go(Direction::West), pos)),
            (Some(SouthWestPipe), Direction::West) => next.push((pos.go(Direction::South), pos)),
            (Some(SouthEastPipe), Direction::South) => next.push((pos.go(Direction::East), pos)),
            (Some(SouthEastPipe), Direction::East) => next.push((pos.go(Direction::South), pos)),
            _ => {}
        }
        next.into_iter()
    }

    pub fn find_definite_loop_position(&self) -> Option<(Position, u32)> {
        let mut distance = 0;
        let mut cur = ArrayVec::<[(Position, Position); 4]>::new();
        let mut definite_loop_position = None;
        cur.push((self.starting_position, self.starting_position));
        while definite_loop_position.is_none() && !cur.is_empty() {
            let mut next = ArrayVec::<[(Position, Position); 4]>::new();
            for (pos, from_pos) in cur
                .iter()
                .flat_map(|&(pos, last_pos)| self.next(pos, last_pos))
            {
                if next.iter().any(|&(p, _)| pos == p) {
                    definite_loop_position = Some(pos);
                    break;
                }
                next.push((pos, from_pos));
            }
            cur = next;
            distance += 1;
        }
        let Some(pos) = definite_loop_position else {
            return None;
        };
        Some((pos, distance))
    }

    pub fn pipe_neighbors(&self, position: Position) -> impl Iterator<Item = Position> {
        let tile = self.get(position).unwrap_or(Tile::Ground);
        tile.neighbors(position)
    }

    pub fn inside_neighbors(
        &self,
        position: Position,
        outside_dir: Direction,
    ) -> impl Iterator<Item = Position> {
        let mut ret = ArrayVec::<[Position; 2]>::new();
        match (self.get(position), outside_dir) {
            (Some(Tile::HorizontalPipe), _) => ret.push(position.go(outside_dir.rev())),
            (Some(Tile::VerticalPipe), _) => ret.push(position.go(outside_dir.rev())),
            (Some(Tile::SouthWestPipe), Direction::North) => {} // Nothing inside here
            (Some(Tile::SouthWestPipe), _) => {
                ret.push(position.go(Direction::North));
                ret.push(position.go(Direction::East));
            }
            (Some(Tile::SouthEastPipe), Direction::West) => {} // Nothing inside here
            (Some(Tile::SouthEastPipe), _) => {
                ret.push(position.go(Direction::North));
                ret.push(position.go(Direction::West));
            }
            (Some(Tile::NorthWestPipe), Direction::East) => {} // Nothing inside here
            (Some(Tile::NorthWestPipe), _) => {
                ret.push(position.go(Direction::South));
                ret.push(position.go(Direction::East));
            }
            (Some(Tile::NorthEastPipe), Direction::South) => {} // Nothing inside here
            (Some(Tile::NorthEastPipe), _) => {
                ret.push(position.go(Direction::South));
                ret.push(position.go(Direction::West));
            }
            _ => {}
        }
        ret.into_iter()
    }
}

impl Position {
    pub fn direction_from(self, from: Self) -> Direction {
        match (
            (self.row - from.row).signum(),
            (self.col - from.col).signum(),
        ) {
            (0, 0) => Direction::None,
            (0, -1) => Direction::East,
            (0, 1) => Direction::West,
            (-1, 0) => Direction::South,
            (1, 0) => Direction::North,
            _ => unreachable!(),
        }
    }

    pub fn go(self, direction: Direction) -> Self {
        match direction {
            Direction::East => Self {
                row: self.row,
                col: self.col + 1,
            },
            Direction::West => Self {
                row: self.row,
                col: self.col - 1,
            },
            Direction::North => Self {
                row: self.row - 1,
                col: self.col,
            },
            Direction::South => Self {
                row: self.row + 1,
                col: self.col,
            },
            Direction::None => self,
        }
    }
}

impl Direction {
    pub fn cardinal() -> impl Iterator<Item = Self> {
        [Self::North, Self::South, Self::East, Self::West].into_iter()
    }

    pub fn rev(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            _ => self,
        }
    }
}

impl Tile {
    pub fn pipes() -> impl Iterator<Item = Self> {
        [
            Self::HorizontalPipe,
            Self::VerticalPipe,
            Self::NorthEastPipe,
            Self::NorthWestPipe,
            Self::SouthEastPipe,
            Self::SouthWestPipe,
        ]
        .into_iter()
    }

    pub fn neighbors(self, position: Position) -> impl Iterator<Item = Position> {
        let mut ret = ArrayVec::<[Position; 2]>::new();
        match self {
            Tile::HorizontalPipe => {
                ret.extend([position.go(Direction::West), position.go(Direction::East)])
            }
            Tile::VerticalPipe => {
                ret.extend([position.go(Direction::North), position.go(Direction::South)])
            }
            Tile::NorthEastPipe => {
                ret.extend([position.go(Direction::North), position.go(Direction::East)])
            }
            Tile::NorthWestPipe => {
                ret.extend([position.go(Direction::North), position.go(Direction::West)])
            }
            Tile::SouthEastPipe => {
                ret.extend([position.go(Direction::South), position.go(Direction::East)])
            }
            Tile::SouthWestPipe => {
                ret.extend([position.go(Direction::South), position.go(Direction::West)])
            }
            _ => {}
        }
        ret.into_iter()
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseFieldError {
    #[error("{0} is not a valid tile character")]
    InvalidTileCharacter(char),

    #[error("input had no tiles")]
    EmptyInput,

    #[error("rows do not have equal column counts")]
    InconsistentColumnCount,

    #[error("no starting position was found")]
    NoStartingPosition,

    #[error("multiple starting positions were found")]
    MultipleStartingPositions,
}

impl FromStr for Field {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let mut rows = 0;
        let cols = lines.peek().ok_or(ParseFieldError::EmptyInput)?.len();
        let mut starting_position = None;
        let tiles = lines
            .flat_map(|line| {
                rows += 1;
                if line.len() != cols {
                    return Either::Left(std::iter::once(Err(
                        ParseFieldError::InconsistentColumnCount,
                    )));
                }
                Either::Right(line.chars().map(Ok))
            })
            .enumerate()
            .map(|(index, ch)| {
                let tile: Tile = ch?.try_into()?;
                if tile == Tile::StartingPosition {
                    if starting_position.is_some() {
                        return Err(ParseFieldError::MultipleStartingPositions);
                    }
                    starting_position = Some(index);
                }
                Ok(tile)
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            tiles,
            rows,
            cols,
            starting_position: starting_position
                .ok_or(ParseFieldError::NoStartingPosition)
                .map(|index| Position {
                    row: (index / cols) as isize,
                    col: (index % cols) as isize,
                })?,
        })
    }
}

impl TryFrom<char> for Tile {
    type Error = ParseFieldError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile::*;
        Ok(match value {
            '|' => VerticalPipe,
            '-' => HorizontalPipe,
            'L' => NorthEastPipe,
            'J' => NorthWestPipe,
            '7' => SouthWestPipe,
            'F' => SouthEastPipe,
            '.' => Ground,
            'S' => StartingPosition,
            ch => return Err(ParseFieldError::InvalidTileCharacter(ch)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(8));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(10));
    }
}
