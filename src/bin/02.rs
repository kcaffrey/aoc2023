use std::str::FromStr;

use tinyvec::TinyVec;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| line.parse::<Game>().ok())
            .filter(|game| {
                game.possible(Colors {
                    red: 12,
                    green: 13,
                    blue: 14,
                })
            })
            .map(|game| game.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| line.parse::<Game>().ok())
            .filter_map(|game| {
                game.reveals
                    .iter()
                    .copied()
                    .reduce(|acc, reveal| acc.maximum(reveal))
            })
            .map(Colors::power)
            .sum(),
    )
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, PartialOrd)]
struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    reveals: TinyVec<[Colors; 10]>,
}

impl Game {
    pub fn possible(&self, bag: Colors) -> bool {
        self.reveals
            .iter()
            .all(|reveal| reveal.possible_reveal(bag))
    }
}

impl Colors {
    pub fn possible_reveal(self, bag: Colors) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    pub fn maximum(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct ParseGameErr;

impl FromStr for Game {
    type Err = ParseGameErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, reveal_str) = s.split_once(':').ok_or(ParseGameErr)?;
        if game_id.len() < 6 {
            return Err(ParseGameErr);
        }
        Ok(Self {
            id: game_id[5..].parse::<u32>().map_err(|_| ParseGameErr)?,
            reveals: reveal_str
                .trim()
                .split(';')
                .map(|reveal| reveal.parse::<Colors>())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl FromStr for Colors {
    type Err = ParseGameErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Colors {
            red: 0,
            green: 0,
            blue: 0,
        };
        for part in s.split(',') {
            let (num_str, color) = part.trim().split_once(' ').ok_or(ParseGameErr)?;
            let num = num_str.parse::<u32>().map_err(|_| ParseGameErr)?;
            match color {
                "red" => ret.red += num,
                "green" => ret.green += num,
                "blue" => ret.blue += num,
                _ => return Err(ParseGameErr),
            };
        }
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
