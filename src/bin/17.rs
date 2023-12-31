use std::fmt::{Display, Write};

use enum_ordinalize::Ordinalize;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u16> {
    solve_a_star(input, 1, 3)
}

pub fn part_two(input: &str) -> Option<u16> {
    solve_a_star(input, 4, 10)
}

fn solve_a_star(
    input: &str,
    min_straight_distance: usize,
    max_straight_distance: usize,
) -> Option<u16> {
    use Alignment::{Horizontal, Vertical};
    let map = Map::parse(input);
    let start = Coordinate::new(0, 0);
    let goal = Coordinate::new(map.height - 1, map.width - 1);
    let mut queue = BucketQueue::new();
    let mut best_so_far = vec![[u16::MAX; 2]; map.width * map.height];
    best_so_far[start.row * map.width + start.col][0] = 0;
    best_so_far[start.row * map.width + start.col][1] = 0;
    let estimates = map.precalculate_heuristic(goal, min_straight_distance, max_straight_distance);
    let estimate_to_goal = estimates[0];
    queue.push(estimate_to_goal as usize, (0, start, Alignment::Vertical));
    queue.push(estimate_to_goal as usize, (0, start, Alignment::Horizontal));
    while let Some((_, (so_far, cur, alignment))) = queue.pop() {
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
                    let index = next.row * map.width + next.col;
                    let cost = map.costs[index];
                    cumulative_cost += cost;
                    if distance < min_straight_distance {
                        continue;
                    }
                    let estimate_to_goal = estimates[index];
                    let cost_so_far = so_far + cumulative_cost;
                    if cost_so_far < best_so_far[index][next_alignment.ordinal() as usize] {
                        best_so_far[index][next_alignment.ordinal() as usize] = cost_so_far;
                        queue.push(
                            (cost_so_far + estimate_to_goal) as usize,
                            (cost_so_far, next, next_alignment),
                        );
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
    costs: Vec<u16>,
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
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let input = input.as_bytes();
        let width = input.iter().position(|&ch| ch == b'\n').unwrap();
        let height = (input.len() + 1) / (width + 1);
        let costs = input
            .split(|&ch| ch == b'\n')
            .flat_map(|line| line.iter().map(|&ch| (ch - b'0') as u16))
            .collect();
        Self {
            costs,
            width,
            height,
        }
    }

    pub fn precalculate_heuristic(
        &self,
        goal: Coordinate,
        min_straight_distance: usize,
        max_straight_distance: usize,
    ) -> Vec<u16> {
        // The idea here is that the basic heuristic is manhattan distance to goal.
        // However, because we have a limit on how far we can go in a straight line,
        // long straight line paths are no good because you have to make a lot of turns
        // to keep going straight.
        // If we are diagonal from the goal, we can zig zag our way down, but if
        // we are close to an edge we'll have to slither back and forth to keep going
        // straight.
        // Thus, figure out which edge we are closer to, and then add a penalty for how
        // many times we'll have to turn to the heuristic.
        let mut ret = vec![0; self.width * self.height];
        for row in 0..self.height {
            for col in 0..self.width {
                let row_diff = row.abs_diff(goal.row);
                let col_diff = col.abs_diff(goal.col);
                let penalty = (col_diff.abs_diff(row_diff) / (2 * max_straight_distance))
                    * (2 * min_straight_distance);
                let estimate_to_goal = (row_diff + col_diff + penalty) as u16;
                ret[row * self.width + col] = estimate_to_goal;
            }
        }
        ret
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                f.write_char((self.costs[row * self.width + col] as u8 + b'0') as char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct BucketQueue<T> {
    buckets: Vec<Vec<T>>,
    first_non_empty: Option<usize>,
}

impl<T: Copy> BucketQueue<T> {
    pub const fn new() -> Self {
        Self {
            buckets: Vec::new(),
            first_non_empty: None,
        }
    }

    pub fn push(&mut self, cost: usize, value: T) {
        if cost >= self.buckets.len() {
            self.buckets
                .resize_with(cost + 1, || Vec::with_capacity(128));
        }
        self.buckets[cost].push(value);
        if self.first_non_empty.filter(|&f| f <= cost).is_none() {
            self.first_non_empty = Some(cost);
        }
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        let Some(min_cost) = self.first_non_empty else {
            return None;
        };
        let value = self.buckets[min_cost].pop().unwrap();
        if self.buckets[min_cost].is_empty() {
            self.first_non_empty =
                (min_cost + 1..self.buckets.len()).find(|&c| !self.buckets[c].is_empty());
        }
        Some((min_cost, value))
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
