use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{Display, Write},
};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let platform = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut score = 0;
    for col in 0..platform[0].len() {
        let mut empty_spaces = 0;
        for row in 0..platform.len() {
            match platform[row][col] {
                '.' => empty_spaces += 1,
                '#' => empty_spaces = 0,
                'O' => {
                    let new_row = row - empty_spaces;
                    score += platform.len() - new_row;
                }
                _ => unreachable!(),
            }
        }
    }
    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform = Platform::parse(input);
    let mut count = 0u64;
    let mut seen = HashMap::new();
    loop {
        let key = platform.cycle();
        count += 1;
        match seen.entry(key) {
            Entry::Occupied(val) => {
                let cycle_length = count - *val.get();
                let remainder = 1_000_000_000
                    - (((1_000_000_000 - count) as f64 / cycle_length as f64).floor() as u64
                        * cycle_length
                        + count);
                for _ in 0..remainder {
                    platform.cycle();
                }
                break;
            }
            Entry::Vacant(e) => {
                e.insert(count);
            }
        }
    }
    Some(platform.load())
}

#[derive(Debug, Default, Clone)]
struct Platform {
    width: u8,
    height: u8,
    round_rocks: Vec<Coordinate>,
    distance_to_cubed_rocks: Vec<Vec<CachedDistance>>,
    stacks: Vec<Vec<u8>>,
    iteration: u16,
}

impl Platform {
    pub fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = grid[0].len() as u8;
        let height = grid.len() as u8;
        let mut round_rocks = Vec::new();
        let mut distance_to_cubed_rocks =
            vec![vec![CachedDistance::default(); grid[0].len()]; grid.len()];
        for row in 0..width as usize {
            for col in 0..height as usize {
                if grid[row][col] == 'O' {
                    round_rocks.push(Coordinate::new(row as u8, col as u8));
                }
                let north = (0..)
                    .find(|&d| d > row || grid[row - d][col] == '#')
                    .unwrap() as u8;
                let south = (0..)
                    .find(|&d| row + d >= height as usize || grid[row + d][col] == '#')
                    .unwrap() as u8;
                let west = (0..)
                    .find(|&d| d > col || grid[row][col - d] == '#')
                    .unwrap() as u8;
                let east = (0..)
                    .find(|&d| col + d >= width as usize || grid[row][col + d] == '#')
                    .unwrap() as u8;
                distance_to_cubed_rocks[row][col] = CachedDistance {
                    north,
                    south,
                    west,
                    east,
                };
            }
        }
        Self {
            round_rocks,
            distance_to_cubed_rocks,
            iteration: 0,
            width,
            height,
            stacks: vec![vec![0; width as usize]; height as usize],
        }
    }

    pub fn cycle(&mut self) -> u128 {
        let mut key = 0;
        self.tilt(Direction::North);
        key += (self.load() as u128) << 96;
        self.tilt(Direction::West);
        key += (self.load() as u128) << 64;
        self.tilt(Direction::South);
        key += (self.load() as u128) << 32;
        self.tilt(Direction::East);
        key += self.load() as u128;
        key
    }

    fn tilt(&mut self, dir: Direction) {
        self.iteration += 1;
        for s in &mut self.stacks {
            s.fill(0);
        }
        for rock in &mut self.round_rocks {
            let distance =
                self.distance_to_cubed_rocks[rock.row as usize][rock.col as usize].get(dir);
            let cubed_rock = rock
                .move_in_dir(dir, distance)
                .limit_to(self.height - 1, self.width - 1);
            let stack = &mut self.stacks[cubed_rock.row as usize][cubed_rock.col as usize];
            if *stack > distance - 1 {
                let total_move_distance = *stack - distance + 1;
                *rock = rock.move_in_dir(dir.rev(), total_move_distance);
            } else {
                let total_move_distance = distance - 1 - *stack;
                *rock = rock.move_in_dir(dir, total_move_distance);
            }
            *stack += 1;
        }
    }

    fn load(&self) -> u32 {
        self.round_rocks
            .iter()
            .map(|&r| (self.height - r.row) as u32)
            .sum()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    row: u8,
    col: u8,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct CachedDistance {
    north: u8,
    south: u8,
    west: u8,
    east: u8,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct RockStack {
    iteration: u16,
    count: u8,
}

impl Coordinate {
    const fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }

    const fn move_in_dir(self, dir: Direction, distance: u8) -> Self {
        match dir {
            Direction::North => Self::new(self.row.saturating_sub(distance), self.col),
            Direction::South => Self::new(self.row + distance, self.col),
            Direction::West => Self::new(self.row, self.col.saturating_sub(distance)),
            Direction::East => Self::new(self.row, self.col + distance),
        }
    }

    fn limit_to(self, max_row: u8, max_col: u8) -> Self {
        Self::new(self.row.min(max_row), self.col.min(max_col))
    }
}

impl CachedDistance {
    const fn get(self, dir: Direction) -> u8 {
        match dir {
            Direction::North => self.north,
            Direction::West => self.west,
            Direction::South => self.south,
            Direction::East => self.east,
        }
    }
}

impl Direction {
    const fn rev(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::West => Self::East,
            Self::South => Self::North,
            Self::East => Self::West,
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = vec![vec!['.'; self.width as usize]; self.height as usize];
        for row in 0..self.height {
            for col in 0..self.width {
                if self.distance_to_cubed_rocks[row as usize][col as usize].north == 0 {
                    grid[row as usize][col as usize] = '#';
                }
            }
        }
        for rock in &self.round_rocks {
            grid[rock.row as usize][rock.col as usize] = 'O';
        }
        for row in grid {
            for ch in row {
                f.write_char(ch)?;
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
