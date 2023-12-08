use rayon::{iter::ParallelIterator, str::ParallelString};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .par_lines()
            .filter_map(winning_number_count)
            .filter(|&c| c > 0)
            .map(|c| 1 << (c - 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut copies = vec![1; lines.len()];
    let mut total = 0;
    for (index, line) in lines.into_iter().enumerate() {
        let card_copies = copies[index];
        total += card_copies;
        let winning_numbers = winning_number_count(line).unwrap_or(0) as usize;
        for i in 0..winning_numbers {
            if let Some(elem) = copies.get_mut(index + i + 1) {
                *elem += card_copies;
            }
        }
    }
    Some(total)
}

fn winning_number_count(card: &str) -> Option<u32> {
    let (_, numbers_part) = card.split_once(':')?;
    let (winning_str, have_str) = numbers_part.split_once('|')?;
    let mut winning_numbers = [false; 100];
    for num in winning_str.split_whitespace().map(|s| s.parse::<usize>()) {
        winning_numbers[num.ok()?] = true;
    }
    Some(
        have_str
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .filter(|&num| winning_numbers[num])
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
