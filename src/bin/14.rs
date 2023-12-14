use std::collections::{hash_map::Entry, HashMap};

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
    let mut platform = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let mut count = 0u64;
    let mut seen = HashMap::new();
    loop {
        cycle(&mut platform);
        count += 1;
        match seen.entry(stringify(&platform)) {
            Entry::Occupied(val) => {
                let cycle_length = count - *val.get();
                let remainder = 1_000_000_000
                    - (((1_000_000_000 - count) as f64 / cycle_length as f64).floor() as u64
                        * cycle_length
                        + count);
                for _ in 0..remainder {
                    cycle(&mut platform);
                }
                break;
            }
            Entry::Vacant(e) => {
                e.insert(count);
            }
        }
    }
    Some(load(&platform))
}

fn north(platform: &mut [Vec<u8>]) {
    for col in 0..platform[0].len() {
        let mut empty_spaces = 0;
        for row in 0..platform.len() {
            match platform[row][col] {
                b'.' => empty_spaces += 1,
                b'#' => empty_spaces = 0,
                b'O' if empty_spaces > 0 => {
                    let new_row = row - empty_spaces;
                    platform[row][col] = b'.';
                    platform[new_row][col] = b'O';
                }
                _ => {}
            }
        }
    }
}

fn west(platform: &mut [Vec<u8>]) {
    for row in 0..platform.len() {
        let mut empty_spaces = 0;
        for col in 0..platform[0].len() {
            match platform[row][col] {
                b'.' => empty_spaces += 1,
                b'#' => empty_spaces = 0,
                b'O' if empty_spaces > 0 => {
                    let new_col = col - empty_spaces;
                    platform[row][col] = b'.';
                    platform[row][new_col] = b'O';
                }
                _ => {}
            }
        }
    }
}

fn south(platform: &mut [Vec<u8>]) {
    for col in 0..platform[0].len() {
        let mut empty_spaces = 0;
        for row in (0..platform.len()).rev() {
            match platform[row][col] {
                b'.' => empty_spaces += 1,
                b'#' => empty_spaces = 0,
                b'O' if empty_spaces > 0 => {
                    let new_row = row + empty_spaces;
                    platform[row][col] = b'.';
                    platform[new_row][col] = b'O';
                }
                _ => {}
            }
        }
    }
}

fn east(platform: &mut [Vec<u8>]) {
    for row in 0..platform.len() {
        let mut empty_spaces = 0;
        for col in (0..platform[0].len()).rev() {
            match platform[row][col] {
                b'.' => empty_spaces += 1,
                b'#' => empty_spaces = 0,
                b'O' if empty_spaces > 0 => {
                    let new_col = col + empty_spaces;
                    platform[row][col] = b'.';
                    platform[row][new_col] = b'O';
                }
                _ => {}
            }
        }
    }
}

fn cycle(platform: &mut [Vec<u8>]) {
    north(platform);
    west(platform);
    south(platform);
    east(platform);
}

fn load(platform: &[Vec<u8>]) -> u32 {
    let mut score = 0;
    for (row_idx, row) in platform.iter().enumerate() {
        for ch in row.iter().copied() {
            if ch == b'O' {
                score += platform.len() - row_idx;
            }
        }
    }
    score as u32
}

fn stringify(platform: &[Vec<u8>]) -> String {
    platform
        .iter()
        .flat_map(|r| r.iter())
        .copied()
        .map(char::from)
        .collect::<String>()
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
