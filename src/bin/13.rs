advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|pattern| {
                let pattern = pattern.as_bytes();
                let cols = pattern
                    .iter()
                    .position(|&ch| ch == b'\n')
                    .expect("should be more than one row");
                let rows = (pattern.len() + 1) / (cols + 1);

                // Try column mirroring.
                let col_mirror = (1..cols).find(|&col_split| {
                    (0..col_split.min(cols - col_split)).all(|offset| {
                        (0..rows).all(|r| {
                            pattern[r * (cols + 1) + col_split - offset - 1]
                                == pattern[r * (cols + 1) + col_split + offset]
                        })
                    })
                });
                if let Some(col_mirror) = col_mirror {
                    return col_mirror as u32;
                }

                // Try row mirroring.
                let row_mirror = (1..rows).find(|&row_split| {
                    (0..row_split.min(rows - row_split)).all(|offset| {
                        (0..cols).all(|c| {
                            pattern[(row_split - offset - 1) * (cols + 1) + c]
                                == pattern[(row_split + offset) * (cols + 1) + c]
                        })
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
                let pattern = pattern.as_bytes();
                let cols = pattern
                    .iter()
                    .position(|&ch| ch == b'\n')
                    .expect("should be more than one row");
                let rows = (pattern.len() + 1) / (cols + 1);

                let col_mirror = (1..cols).find(|&col_split| {
                    (0..col_split.min(cols - col_split))
                        .map(|offset| {
                            (0..rows)
                                .map(|r| -> u32 {
                                    (pattern[r * (cols + 1) + col_split - offset - 1]
                                        != pattern[r * (cols + 1) + col_split + offset])
                                        .into()
                                })
                                .sum::<u32>()
                        })
                        .sum::<u32>()
                        == 1
                });
                if let Some(col_mirror) = col_mirror {
                    return col_mirror as u32;
                }

                let row_mirror = (1..rows).find(|&row_split| {
                    (0..row_split.min(rows - row_split))
                        .map(|offset| {
                            (0..cols)
                                .map(|c| -> u32 {
                                    (pattern[(row_split - offset - 1) * (cols + 1) + c]
                                        != pattern[(row_split + offset) * (cols + 1) + c])
                                        .into()
                                })
                                .sum::<u32>()
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
