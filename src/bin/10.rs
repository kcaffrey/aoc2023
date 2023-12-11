use std::str::FromStr;

use either::Either;
use tinyvec::ArrayVec;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let field = input.parse::<Field>().expect("valid input");
    field.find_loop_length()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut field = input.parse::<Field>().expect("valid input");
    field.remove_non_loop_pipes();

    let mut inside_count = 0;
    for row in 0..field.rows {
        let mut inside = false;
        for col in 0..field.cols {
            match field.get(Position { row, col })? {
                Tile::VerticalPipe | Tile::SouthEastPipe | Tile::SouthWestPipe => inside = !inside,
                Tile::Ground if inside => inside_count += 1,
                _ => {}
            }
        }
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
struct Position {
    row: usize,
    col: usize,
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
        if position.row >= self.rows || position.col >= self.cols {
            return None;
        }
        let index = self.cols * position.row + position.col;
        Some(index)
    }

    pub fn find_loop_length(&self) -> Option<u32> {
        let mut distance = 0;
        let mut dir = match self.get(self.starting_position)? {
            Tile::HorizontalPipe | Tile::NorthEastPipe | Tile::SouthEastPipe => Direction::East,
            Tile::VerticalPipe | Tile::NorthWestPipe => Direction::North,
            Tile::SouthWestPipe => Direction::South,
            Tile::Ground => return None,
        };
        let mut cur = self.starting_position;
        loop {
            cur = cur.go(dir);
            dir = self.get(cur)?.next_direction(dir);
            distance += 1;
            if cur == self.starting_position {
                break;
            }
        }
        Some(distance / 2)
    }

    pub fn remove_non_loop_pipes(&mut self) {
        let mut new_tiles = vec![Tile::Ground; self.tiles.len()];
        let starting_tile = self
            .get(self.starting_position)
            .expect("should be a starting tile");
        new_tiles[self
            .index(self.starting_position)
            .expect("should be a valid position")] = starting_tile;
        let mut dir = match starting_tile {
            Tile::HorizontalPipe | Tile::NorthEastPipe | Tile::SouthEastPipe => Direction::East,
            Tile::VerticalPipe | Tile::NorthWestPipe if self.starting_position.row > 0 => {
                Direction::North
            }
            Tile::NorthWestPipe => Direction::West,
            Tile::VerticalPipe | Tile::SouthWestPipe => Direction::South,
            Tile::Ground => unreachable!("starting tile shouldn't be ground"),
        };
        let mut cur = self.starting_position;
        loop {
            cur = cur.go(dir);
            let index = self.index(cur).expect("should be a valid position");
            let cur_tile = self.tiles[index];
            new_tiles[index] = cur_tile;
            dir = cur_tile.next_direction(dir);
            if cur == self.starting_position {
                break;
            }
        }
        self.tiles = new_tiles;
    }

    pub fn pipe_neighbors(&self, position: Position) -> impl Iterator<Item = Position> {
        let tile = self.get(position).unwrap_or(Tile::Ground);
        tile.neighbors(position)
    }
}

impl Position {
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
        }
    }

    pub fn checked_go(self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::East => Self {
                row: self.row,
                col: self.col + 1,
            },
            Direction::West => Self {
                row: self.row,
                col: self.col.checked_sub(1)?,
            },
            Direction::North => Self {
                row: self.row.checked_sub(1)?,
                col: self.col,
            },
            Direction::South => Self {
                row: self.row + 1,
                col: self.col,
            },
        })
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

    pub fn next_direction(self, dir: Direction) -> Direction {
        match (self, dir) {
            (Tile::HorizontalPipe, _) | (Tile::VerticalPipe, _) => dir,
            (Tile::NorthEastPipe, Direction::South) => Direction::East,
            (Tile::NorthEastPipe, Direction::West) => Direction::North,
            (Tile::NorthWestPipe, Direction::South) => Direction::West,
            (Tile::NorthWestPipe, Direction::East) => Direction::North,
            (Tile::SouthEastPipe, Direction::North) => Direction::East,
            (Tile::SouthEastPipe, Direction::West) => Direction::South,
            (Tile::SouthWestPipe, Direction::North) => Direction::West,
            (Tile::SouthWestPipe, Direction::East) => Direction::South,
            _ => dir,
        }
    }

    pub fn neighbors(self, pos: Position) -> impl Iterator<Item = Position> {
        let mut ret = ArrayVec::<[Position; 2]>::new();
        use Direction::*;
        match self {
            Tile::HorizontalPipe => ret.extend(
                [pos.checked_go(West), pos.checked_go(East)]
                    .into_iter()
                    .flatten(),
            ),
            Tile::VerticalPipe => ret.extend(
                [pos.checked_go(North), pos.checked_go(South)]
                    .into_iter()
                    .flatten(),
            ),
            Tile::NorthEastPipe => ret.extend(
                [pos.checked_go(North), pos.checked_go(East)]
                    .into_iter()
                    .flatten(),
            ),
            Tile::NorthWestPipe => ret.extend(
                [pos.checked_go(North), pos.checked_go(West)]
                    .into_iter()
                    .flatten(),
            ),
            Tile::SouthEastPipe => ret.extend(
                [pos.checked_go(South), pos.checked_go(East)]
                    .into_iter()
                    .flatten(),
            ),
            Tile::SouthWestPipe => ret.extend(
                [pos.checked_go(South), pos.checked_go(West)]
                    .into_iter()
                    .flatten(),
            ),
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

    #[error("no valid tile was found for starting position")]
    NoValidStartingTile,
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
                let ch = ch?;
                let tile: Tile = ch.try_into()?;
                if ch == 'S' {
                    if starting_position.is_some() {
                        return Err(ParseFieldError::MultipleStartingPositions);
                    }
                    starting_position = Some(index);
                }
                Ok(tile)
            })
            .collect::<Result<_, _>>()?;
        let mut field = Self {
            tiles,
            rows,
            cols,
            starting_position: starting_position
                .ok_or(ParseFieldError::NoStartingPosition)
                .map(|index| Position {
                    row: (index / cols),
                    col: (index % cols),
                })?,
        };
        let starting_position = field.starting_position;
        let starting_tile = Tile::pipes()
            .find(|&pipe| {
                pipe.neighbors(starting_position)
                    .all(|n| field.pipe_neighbors(n).any(|n2| n2 == starting_position))
            })
            .ok_or(ParseFieldError::NoValidStartingTile)?;
        if let Some(cell) = field.get_mut(starting_position) {
            *cell = starting_tile;
        };
        Ok(field)
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
            // NOTE: we'll replace the starting tile later, so just return Ground as a placeholder
            '.' | 'S' => Ground,
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
