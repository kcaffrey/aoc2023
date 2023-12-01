advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let mut nums = line.chars().filter(|ch| ch.is_numeric());
                let first = nums.next().and_then(|ch| ch.to_digit(10));
                let last = nums.next_back().and_then(|ch| ch.to_digit(10)).or(first);
                first.zip(last).map(|(a, b)| a * 10 + b)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let digit_strs = [
        ("one", 1u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    Some(
        input
            .to_ascii_lowercase()
            .lines()
            .filter_map(|line| {
                let first = digit_strs
                    .iter()
                    .filter_map(|&(s, val)| line.find(s).map(|index| (index, val)))
                    .min_by_key(|&(index, _)| index)
                    .map(|(_, val)| val);
                let last = digit_strs
                    .iter()
                    .filter_map(|&(s, val)| line.rfind(s).map(|index| (index, val)))
                    .max_by_key(|&(index, _)| index)
                    .map(|(_, val)| val);
                first.zip(last).map(|(a, b)| a * 10 + b)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
