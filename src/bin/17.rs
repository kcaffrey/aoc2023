use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::{Display, Write},
};

use enum_ordinalize::Ordinalize;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    solve_a_star(input, 1, 3)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve_a_star(input, 4, 10)
}

fn solve_a_star(
    input: &str,
    min_straight_distance: usize,
    max_straight_distance: usize,
) -> Option<u32> {
    use Alignment::{Horizontal, Vertical};
    let map = Map::parse(input);
    let start = Coordinate::new(0, 0);
    let goal = Coordinate::new(map.height - 1, map.width - 1);
    let mut queue = BinaryHeap::new();
    let mut best_so_far = vec![vec![[u32::MAX; 2]; map.width]; map.height];
    best_so_far[start.row][start.col][0] = 0;
    best_so_far[start.row][start.col][1] = 0;
    let estimate_to_goal = start.manhattan_distance(goal);
    queue.push(Reverse((estimate_to_goal, 0, start, Alignment::Vertical)));
    queue.push(Reverse((estimate_to_goal, 0, start, Alignment::Horizontal)));
    while let Some(Reverse((_, so_far, cur, alignment))) = queue.pop() {
        if cur == goal {
            return Some(so_far);
        }
        for dir in [-1, 1] {
            let mut cumulative_cost = 0;
            for distance in 1..=max_straight_distance {
                if let Some((next, next_alignment)) = match (alignment, dir) {
                    (Alignment::Vertical, -1) => cur
                        .row
                        .checked_sub(distance)
                        .map(|r| (coord!(r, cur.col), Horizontal)),
                    (Alignment::Horizontal, -1) => cur
                        .col
                        .checked_sub(distance)
                        .map(|c| (coord!(cur.row, c), Vertical)),
                    (Alignment::Vertical, 1) => Some(cur.row + distance)
                        .filter(|&r| r < map.height)
                        .map(|r| (coord!(r, cur.col), Horizontal)),
                    (Alignment::Horizontal, 1) => Some(cur.col + distance)
                        .filter(|&c| c < map.width)
                        .map(|c| (coord!(cur.row, c), Vertical)),
                    _ => None,
                } {
                    let cost = map.costs[next.row][next.col];
                    cumulative_cost += cost;
                    if distance < min_straight_distance {
                        continue;
                    }
                    let estimate_to_goal = next.manhattan_distance(goal);
                    let cost_so_far = so_far + cumulative_cost;
                    if cost_so_far
                        < best_so_far[next.row][next.col][next_alignment.ordinal() as usize]
                    {
                        best_so_far[next.row][next.col][next_alignment.ordinal() as usize] =
                            cost_so_far;
                        queue.push(Reverse((
                            cost_so_far + estimate_to_goal,
                            cost_so_far,
                            next,
                            next_alignment,
                        )));
                    }
                } else {
                    break;
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
struct Map {
    costs: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Ordinalize)]
#[repr(u8)]
enum Alignment {
    #[default]
    Horizontal,
    Vertical,
}

impl Coordinate {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub const fn manhattan_distance(&self, other: Self) -> u32 {
        (self.row.abs_diff(other.row) + self.col.abs_diff(other.col)) as u32
    }
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let input = input.as_bytes();
        let width = input.iter().position(|&ch| ch == b'\n').unwrap();
        let height = (input.len() + 1) / (width + 1);
        let costs = input
            .split(|&ch| ch == b'\n')
            .map(|line| line.iter().map(|&ch| (ch - b'0') as u32).collect())
            .collect();
        Self {
            costs,
            width,
            height,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                f.write_char((self.costs[row][col] as u8 + b'0') as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
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
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
        let input = r#"
111111111111
999999999991
999999999991
999999999991
999999999991"#
            .trim();
        let result = part_two(input);
        assert_eq!(result, Some(71));
    }
}
