use rayon::{iter::ParallelIterator, str::ParallelString};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .par_lines()
            .map(|line| {
                let nums = line
                    .split_whitespace()
                    .map(|ch| ch.parse::<i32>().expect("should be a number"))
                    .collect::<Vec<_>>();
                find_next(nums)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(
        input
            .par_lines()
            .map(|line| {
                let nums = line
                    .split_whitespace()
                    .rev()
                    .map(|ch| ch.parse::<i32>().expect("should be a number"))
                    .collect::<Vec<_>>();
                find_next(nums)
            })
            .sum(),
    )
}

fn find_next(input: Vec<i32>) -> i32 {
    let Some(&last_value) = input.last() else {
        return 0;
    };
    let mut differences = input;
    let mut last_difference_sum = 0;
    while differences.len() > 1 {
        let mut all_zeros = true;
        for i in 1..differences.len() {
            let difference = differences[i] - differences[i - 1];
            if difference != 0 {
                all_zeros = false;
            }
            differences[i - 1] = difference;
            if i == differences.len() - 1 {
                last_difference_sum += difference;
            }
        }
        if all_zeros {
            break;
        }
        differences.pop();
    }
    last_value + last_difference_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
