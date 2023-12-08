use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use itertools::Itertools;
use num_integer::Integer;
use rayon::prelude::*;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse(input)?;
    let start = "AAA".parse().ok()?;
    Some(map.steps_to_dest(start, Node::is_zzz))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse(input)?;
    Some(
        map.adjacency
            .keys()
            .copied()
            .collect::<Vec<_>>()
            .into_par_iter()
            .filter(Node::ends_with_a)
            .map(|start| map.steps_to_dest(start, Node::ends_with_z))
            .reduce(|| 1, num_integer::lcm),
    )
}

pub fn part_three(input: &str) -> Option<u64> {
    let map = parse(input)?;

    // Find the cycles corresponding to each starting node.
    // Each cycle involves some starting offset S until a XXZ node is first reached
    // and then some cycle length L until the XXZ node is reached again.
    // If any starting node cycles before reaching a destination, then cycle_offset_and_length
    // will return none.
    // Note that XXZ might not be the first destination reached, as we need a destination that is
    // part of the cycle.
    let mut cycles = map
        .adjacency
        .keys()
        .copied()
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter(Node::ends_with_a)
        .map(|start| map.cycle_offset_and_length(start, Node::ends_with_z))
        // NOTE: if any of the cycles don't reach the destination, we can't find a solution,
        // so we return None.
        .collect::<Option<Vec<_>>>()?;

    // Iteratively combine cycles by finding when they first intersect and their period.
    // This can be done by solving the equation A + Bx = C + Dy for integer solutions,
    // which can be rewritten as the linear Diophantine equation Ax + By = C
    while let Some(last_cycle) = cycles.pop() {
        let Some(second_last_cycle) = cycles.last_mut() else {
            // We are at the last remaining cycle, so we are done!
            return Some(last_cycle.0);
        };

        // Use extended GCD/Euclids algorithm to find solutions to ax + by = c
        let a = last_cycle.1 as i64;
        let b = -(second_last_cycle.1 as i64);
        let c = second_last_cycle.0 as i64 - last_cycle.0 as i64;
        let gcd = a.extended_gcd(&b);
        if c % gcd.gcd != 0 {
            // No solutions exist.
            return None;
        }
        let mut x = gcd.x * (c / gcd.gcd);
        let y = gcd.y * (c / gcd.gcd);
        let x_period = -b / gcd.gcd;
        let y_period = a / gcd.gcd;

        // We need to find the smallest positive solution, so we adjust the solution found by the euclid
        // algorithm accordingly.
        // If we don't do this, the solution can explode in size (and likely miss the actual smallest solution at the end).
        let x_k = if x < 0 {
            (-x + x_period - 1) / x_period
        } else {
            0
        };
        let y_k = if y < 0 {
            (-y + y_period - 1) / y_period
        } else {
            0
        };
        let k = x_k.max(y_k);
        x += k * x_period;
        x %= x_period;
        if x == 0 {
            x = x_period;
        }

        // Now that we have a solution to the pair, we can plug in x to find the smallest
        // positive value that is the result of Z = A + Bx = C + Dy, which is the new offset,
        // and then the periodicity of the solution, which is the LCM(B, D).
        let x = x as u64;
        let new_offset = last_cycle.0 + last_cycle.1 * x;
        let new_cycle_length = ((a * b) / gcd.gcd).unsigned_abs();
        *second_last_cycle = (new_offset, new_cycle_length);
    }

    // This will only be hit if there were no starting nodes at all.
    None
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Node(u16);

#[derive(Clone, Debug)]
struct Map {
    instructions: Vec<Direction>,
    adjacency: BTreeMap<Node, (Node, Node)>,
}

impl Map {
    pub fn steps_to_dest<F: Fn(&Node) -> bool>(&self, start: Node, is_dest: F) -> u64 {
        let mut cur = start;
        let mut steps = 0;
        for instruction in self.instructions.iter().copied().cycle() {
            let &(left, right) = self
                .adjacency
                .get(&cur)
                .expect("should be in adjacency map");
            cur = match instruction {
                Direction::Left => left,
                Direction::Right => right,
            };
            steps += 1;
            if is_dest(&cur) {
                return steps;
            }
        }
        unreachable!()
    }

    pub fn cycle_offset_and_length<F: Fn(&Node) -> bool>(
        &self,
        start: Node,
        is_dest: F,
    ) -> Option<(u64, u64)> {
        let mut seen = BTreeMap::from_iter(
            self.adjacency
                .keys()
                .copied()
                .cartesian_product(0..self.instructions.len())
                .map(|(key, instruction_index)| ((key, instruction_index), None)),
        );
        let mut cur = start;
        let mut instructions = self
            .instructions
            .iter()
            .copied()
            .enumerate()
            .cycle()
            .peekable();

        // Find the start of the cycle.
        let mut offset_steps = 0;
        let cycle_length = loop {
            let &(instruction_index, _) = instructions.peek()?;
            let cur_seen = seen.get_mut(&(cur, instruction_index)).unwrap();
            if let Some(last_steps) = *cur_seen {
                break offset_steps - last_steps;
            }
            *cur_seen = Some(offset_steps);

            let (_, instruction) = instructions.next()?;
            let &(left, right) = self
                .adjacency
                .get(&cur)
                .expect("should be in adjacency map");
            cur = match instruction {
                Direction::Left => left,
                Direction::Right => right,
            };
            offset_steps += 1;
        };

        // We found a cycle starting at cur. We calculated the cycle length
        // inside the loop, so now we just need to find how far into the cycle
        // the first destination node is and add that to the cycle offset.
        let mut cycle_dest_offset = 0;
        while !is_dest(&cur) {
            let (_, instruction) = instructions.next()?;
            let &(left, right) = self
                .adjacency
                .get(&cur)
                .expect("should be in adjacency map");
            cur = match instruction {
                Direction::Left => left,
                Direction::Right => right,
            };
            cycle_dest_offset += 1;
        }

        Some((
            offset_steps - (cycle_length - cycle_dest_offset),
            cycle_length,
        ))
    }
}

impl Node {
    pub fn ends_with_a(&self) -> bool {
        self.0 % 26 == 0
    }

    pub fn ends_with_z(&self) -> bool {
        self.0 % 26 == 25
    }

    pub fn is_zzz(&self) -> bool {
        self.0 == 25 * 26 * 26 + 25 * 26 + 25
    }
}

fn parse(input: &str) -> Option<Map> {
    let (instructions, adjacency) = input.split_once("\n\n")?;
    let instructions = instructions
        .trim()
        .chars()
        .filter_map(|ch| match ch {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect();
    let adjacency = adjacency
        .lines()
        .filter_map(|line| {
            let (source, dest_strs) = line.split_once(" = ")?;
            let (left_dest, right_dest) = dest_strs
                .trim_matches(|ch| ch == '(' || ch == ')')
                .split_once(", ")?;
            Some((
                source.parse().ok()?,
                (left_dest.parse().ok()?, right_dest.parse().ok()?),
            ))
        })
        .collect();
    Some(Map {
        instructions,
        adjacency,
    })
}

#[derive(Debug)]
struct ParseNodeErr;

impl FromStr for Node {
    type Err = ParseNodeErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|ch| ch as u16 - 'A' as u16)
                .reduce(|acc, d| acc * 26 + d)
                .ok_or(ParseNodeErr)?,
        ))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = [self.0 / (26 * 26), (self.0 / 26) % 26, self.0 % 26]
            .into_iter()
            .map(|d| (d + 'A' as u16) as u8 as char)
            .collect();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
