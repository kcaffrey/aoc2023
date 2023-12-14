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
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut old = platform.clone();
    let mut count = 0u64;
    let mut seen = HashMap::new();
    while cycle(&mut platform, &mut old) {
        count += 1;
        let state = print(&platform);
        match seen.entry(state) {
            Entry::Occupied(val) => {
                let cycle_length = count - *val.get();
                let remainder = 1_000_000_000
                    - (((1_000_000_000 - count) as f64 / cycle_length as f64).floor() as u64
                        * cycle_length
                        + count);
                for _ in 0..remainder {
                    cycle(&mut platform, &mut old);
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

fn north(platform: &mut [Vec<char>]) -> u32 {
    let mut score = 0;
    for col in 0..platform[0].len() {
        let mut empty_spaces = 0;
        for row in 0..platform.len() {
            match platform[row][col] {
                '.' => empty_spaces += 1,
                '#' => empty_spaces = 0,
                'O' => {
                    let new_row = row - empty_spaces;
                    platform[row][col] = '.';
                    platform[new_row][col] = 'O';
                    score += platform[0].len() - new_row;
                }
                _ => unreachable!(),
            }
        }
    }
    score as u32
}

fn west(platform: &mut [Vec<char>]) {
    for row in 0..platform.len() {
        let mut empty_spaces = 0;
        for col in 0..platform[0].len() {
            match platform[row][col] {
                '.' => empty_spaces += 1,
                '#' => empty_spaces = 0,
                'O' => {
                    let new_col = col - empty_spaces;
                    platform[row][col] = '.';
                    platform[row][new_col] = 'O';
                }
                _ => unreachable!(),
            }
        }
    }
}

fn south(platform: &mut [Vec<char>]) {
    for col in 0..platform[0].len() {
        let mut empty_spaces = 0;
        for row in (0..platform.len()).rev() {
            match platform[row][col] {
                '.' => empty_spaces += 1,
                '#' => empty_spaces = 0,
                'O' => {
                    let new_row = row + empty_spaces;
                    platform[row][col] = '.';
                    platform[new_row][col] = 'O';
                }
                _ => unreachable!(),
            }
        }
    }
}

fn east(platform: &mut [Vec<char>]) {
    for row in 0..platform.len() {
        let mut empty_spaces = 0;
        for col in (0..platform[0].len()).rev() {
            match platform[row][col] {
                '.' => empty_spaces += 1,
                '#' => empty_spaces = 0,
                'O' => {
                    let new_col = col + empty_spaces;
                    platform[row][col] = '.';
                    platform[row][new_col] = 'O';
                }
                _ => unreachable!(),
            }
        }
    }
}

fn cycle(platform: &mut [Vec<char>], old: &mut [Vec<char>]) -> bool {
    let mut changed = false;
    north(platform);
    west(platform);
    south(platform);
    east(platform);
    for r in 0..platform.len() {
        for c in 0..platform[0].len() {
            if !changed && old[r][c] != platform[r][c] {
                changed = true;
            }
            old[r][c] = platform[r][c];
        }
    }
    changed
}

fn load(platform: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;
    for (row_idx, row) in platform.iter().enumerate() {
        for ch in row.iter().copied() {
            if ch == 'O' {
                score += platform.len() - row_idx;
            }
        }
    }
    score as u32
}

fn print(platform: &[Vec<char>]) -> String {
    platform
        .iter()
        .flat_map(|r| r.iter().chain(std::iter::once(&'\n')))
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
