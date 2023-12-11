advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    compute_expansion(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    compute_expansion(input, 1_000_000)
}

pub fn compute_expansion(input: &str, expansion_factor: u32) -> Option<u64> {
    let expansion_addition = expansion_factor.saturating_sub(1) as usize;

    // First compute the number of galaxies in each row and column.
    let mut row_counts = Vec::with_capacity(input.len());
    let mut col_counts = Vec::new();
    for line in input.lines() {
        if col_counts.is_empty() {
            col_counts.resize(line.len(), 0);
        }
        let mut row_count = 0;
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                col_counts[col] += 1;
                row_count += 1;
            }
        }
        row_counts.push(row_count);
    }

    // Now compute the sum of pairwise row differences and column differences
    // after expansion.
    Some(
        compute_diff_sum(&row_counts, expansion_addition)
            + compute_diff_sum(&col_counts, expansion_addition),
    )
}

fn compute_diff_sum(counts: &[i32], expansion_addition: usize) -> u64 {
    // This helper leverages an algebraic trick to turn the sum of distances
    // into a linear operation:
    // (x[i] - x[0]) + (x[i] - x[1]) + ... + (x[i] - x[i-1]) =
    // i * x[i] - (x[0] + x[1] + ... + x[i - 1])
    //
    // Since we are iterating over galaxies computing the row difference
    // and column difference according to that formula for every i from 1
    // to N, we reduce the operation to computing a running sum and then
    // subtracting i * x[i] - running_sum for every i, reducing a quadratic
    // operation into a linear one.
    //
    // HOWEVER! This requires the values to be sorted. To do this, we
    // reproduce the indexes by the row and column counts, which we
    // needed in order to produce the expanded row/col indexes in the first
    // place. So long as we iterate over those arrays in order, we are good!
    let mut difference_sum = 0;
    let mut total_sum = 0;
    let mut offset = 0;
    let mut galaxy_index = 0;
    for (i, c) in counts.iter().copied().enumerate() {
        let expanded_rowcol_index = i + offset;
        for _ in 0..c {
            difference_sum += galaxy_index * expanded_rowcol_index - total_sum;
            total_sum += expanded_rowcol_index;
            galaxy_index += 1;
        }
        if c == 0 {
            offset += expansion_addition;
        }
    }
    difference_sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_other_expansion_factors() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        assert_eq!(compute_expansion(input, 10), Some(1030));
        assert_eq!(compute_expansion(input, 100), Some(8410));
    }
}
