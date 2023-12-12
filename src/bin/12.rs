use rayon::{iter::ParallelIterator, str::ParallelString};
use tinyvec::ArrayVec;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.par_lines().map(|line| solve_line(line, 1)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.par_lines().map(|line| solve_line(line, 5)).sum())
}

fn solve_line(line: &str, copies: usize) -> u64 {
    let (spring_records, damaged_counts) = line.split_once(' ').expect("should be valid input");
    let spring_records = vec![spring_records; copies].join("?");
    let spring_records = spring_records.as_bytes();

    let damaged_counts = std::iter::repeat(
        damaged_counts
            .split(',')
            .map(|s| s.parse::<u32>().expect("should be valid spring count")),
    )
    .take(copies)
    .flatten()
    .collect::<ArrayVec<[u32; 64]>>();

    // solve_brute_force_recursion(spring_records, &damaged_counts, 0)
    solve_dp(spring_records, &damaged_counts)
}

fn solve_dp(spring_records: &[u8], damaged_counts: &[u32]) -> u64 {
    // Precompute how many springs could be damaged in a row ending at each spring.
    let mut damaged_runs = vec![0; spring_records.len()];
    let mut cur_damaged_run = 0;
    let mut first_damaged = spring_records.len();
    for (i, record) in spring_records.iter().copied().enumerate() {
        if record != b'.' {
            cur_damaged_run += 1;
        } else {
            cur_damaged_run = 0;
        }
        if record == b'#' && i < first_damaged {
            first_damaged = i;
        }
        damaged_runs[i] = cur_damaged_run;
    }

    // Solve the problem using DP, for f(M, N) where M is the number of
    // damaged springs that have been "resolved" and N is how many springs are used from the input.
    let mut prev = vec![0; spring_records.len() + 1];
    let mut cur = vec![0; spring_records.len() + 1];
    (0..=first_damaged).for_each(|i| {
        prev[i] = 1;
    });
    for (damaged_index, damaged_count) in damaged_counts.iter().copied().enumerate() {
        let damaged_count = damaged_count as usize;
        cur[0] = 0;
        for end_of_spring in 0..spring_records.len() {
            let last_record = spring_records[end_of_spring];
            let mut count = 0;
            if last_record == b'.' || last_record == b'?' {
                // This record can be operational. The count is the same
                // as f(damaged_count, end_of_spring).
                count += end_of_spring
                    .checked_sub(0)
                    .map(|index| cur[index])
                    .unwrap_or(0);
            }
            if last_record == b'#' || last_record == b'?' {
                // This record can be counted as damaged.
                // First we see if this can even be considered the end of the  next run
                // of damaged springs. To be considered the end of such a run, the next
                // spring cannot be damaged, and the previous springs (corresponding to the
                // next damaged count) have to be either unknown or damaged.
                // Finally, the character preceding the run of damaged springs must not
                // be damaged (so either . or ?).
                let next_is_definite_damaged = spring_records
                    .get(end_of_spring + 1)
                    .filter(|&&v| v == b'#')
                    .is_some();
                let has_correct_damaged_count =
                    !next_is_definite_damaged && damaged_runs[end_of_spring] >= damaged_count;
                let has_correct_preceding = has_correct_damaged_count
                    && end_of_spring
                        .checked_sub(damaged_count)
                        .map(|i| spring_records[i])
                        .filter(|&v| v == b'#')
                        .is_none();
                if has_correct_preceding {
                    count += end_of_spring
                        .checked_sub(damaged_count)
                        .map(|index| prev[index])
                        .unwrap_or(if damaged_index == 0 { 1 } else { 0 });
                }
            }

            cur[end_of_spring + 1] = count;
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[spring_records.len()]
}

#[allow(unused)]
fn solve_brute_force_recursion(
    spring_records: &[u8],
    damaged_counts: &[u32],
    current_damaged_length: u32,
) -> u64 {
    if spring_records.is_empty() && damaged_counts.is_empty() && current_damaged_length == 0
        || spring_records.is_empty()
            && damaged_counts.len() == 1
            && current_damaged_length == damaged_counts[0]
    {
        return 1;
    } else if spring_records.is_empty()
        || damaged_counts.is_empty() && current_damaged_length > 0
        || !damaged_counts.is_empty() && current_damaged_length > damaged_counts[0]
    {
        return 0;
    }

    let mut count = 0;
    if spring_records[0] == b'.' || spring_records[0] == b'?' {
        // Next record can be operational.
        count += if current_damaged_length > 0 {
            if current_damaged_length == damaged_counts[0] {
                solve_brute_force_recursion(&spring_records[1..], &damaged_counts[1..], 0)
            } else {
                0
            }
        } else {
            solve_brute_force_recursion(&spring_records[1..], damaged_counts, 0)
        };
    }
    if spring_records[0] == b'#' || spring_records[0] == b'?' {
        // Next record can be damaged.
        count += solve_brute_force_recursion(
            &spring_records[1..],
            damaged_counts,
            current_damaged_length + 1,
        );
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
