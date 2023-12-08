use std::{collections::BTreeMap, fmt::Display, str::FromStr};

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
