use smallvec::SmallVec;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| {
                let mut row_values = SmallVec::<[u32; 32]>::new();
                let mut col_values = SmallVec::<[u32; 32]>::new();
                for line in pattern.lines() {
                    let mut row_encoded = 0;
                    if col_values.is_empty() {
                        col_values.resize(line.len(), 0);
                    }
                    for (col, ch) in line.char_indices() {
                        let encoded_val = if ch == '#' { 1 } else { 0 };
                        col_values[col] = (col_values[col] << 1) + encoded_val;
                        row_encoded = (row_encoded << 1) + encoded_val;
                    }
                    row_values.push(row_encoded);
                }

                // Try column mirroring.
                let col_mirror = (1..col_values.len()).find(|&col_split| {
                    (0..col_split.min(col_values.len() - col_split)).all(|offset| {
                        col_values[col_split - offset - 1] == col_values[col_split + offset]
                    })
                });
                if let Some(col_mirror) = col_mirror {
                    return col_mirror as u32;
                }

                // Try row mirroring.
                let row_mirror = (1..row_values.len()).find(|&row_split| {
                    (0..row_split.min(row_values.len() - row_split)).all(|offset| {
                        row_values[row_split - offset - 1] == row_values[row_split + offset]
                    })
                });
                if let Some(row_mirror) = row_mirror {
                    return row_mirror as u32 * 100;
                }

                unreachable!("every pattern shold have a solution")
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| {
                let mut row_values = SmallVec::<[u32; 32]>::new();
                let mut col_values = SmallVec::<[u32; 32]>::new();
                for line in pattern.lines() {
                    let mut row_encoded = 0u32;
                    if col_values.is_empty() {
                        col_values.resize(line.len(), 0u32);
                    }
                    for (col, ch) in line.char_indices() {
                        let encoded_val = if ch == '#' { 1 } else { 0 };
                        col_values[col] = (col_values[col] << 1) + encoded_val;
                        row_encoded = (row_encoded << 1) + encoded_val;
                    }
                    row_values.push(row_encoded);
                }

                // Try column mirroring.
                let col_mirror = (1..col_values.len()).find(|&col_split| {
                    (0..col_split.min(col_values.len() - col_split))
                        .map(|offset| {
                            (col_values[col_split - offset - 1] ^ col_values[col_split + offset])
                                .count_ones()
                        })
                        .sum::<u32>()
                        == 1
                });
                if let Some(col_mirror) = col_mirror {
                    return col_mirror as u32;
                }

                // Try row mirroring.
                let row_mirror = (1..row_values.len()).find(|&row_split| {
                    (0..row_split.min(row_values.len() - row_split))
                        .map(|offset| {
                            (row_values[row_split - offset - 1] ^ row_values[row_split + offset])
                                .count_ones()
                        })
                        .sum::<u32>()
                        == 1
                });
                if let Some(row_mirror) = row_mirror {
                    return row_mirror as u32 * 100;
                }

                unreachable!("every pattern shold have a solution")
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
