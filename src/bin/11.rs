advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    compute_expansion(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    compute_expansion(input, 1_000_000)
}

pub fn compute_expansion(input: &str, expansion_factor: u32) -> Option<u64> {
    let expansion_addition = expansion_factor.saturating_sub(1) as i32;
    let mut col_counts = Vec::new();
    let mut galaxies = Vec::with_capacity(input.len());
    let mut row_offsets = Vec::with_capacity(input.len());
    let mut total_row_offset = 0;
    for (row, line) in input.lines().enumerate() {
        if col_counts.is_empty() {
            col_counts.resize(line.len(), 0);
        }
        let mut row_count = 0;
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((row as i32, col as i32));
                col_counts[col] += 1;
                row_count += 1;
            }
        }
        row_offsets.push(total_row_offset);
        if row_count == 0 {
            total_row_offset += expansion_addition;
        }
    }
    let mut col_offsets = col_counts;
    let mut total_col_offset = 0;
    for val in &mut col_offsets {
        let offset = total_col_offset;
        if *val == 0 {
            total_col_offset += expansion_addition;
        }
        *val = offset;
    }
    for galaxy in &mut galaxies {
        galaxy.0 += row_offsets[galaxy.0 as usize];
        galaxy.1 += col_offsets[galaxy.1 as usize];
    }
    Some(
        galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, &g1)| galaxies.iter().skip(i + 1).map(move |&g2| (g1, g2)))
            .map(|(g1, g2)| (g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)) as u64)
            .sum(),
    )
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
