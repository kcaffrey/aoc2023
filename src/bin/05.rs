use std::{ops::Range, str::FromStr};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let almanac = input.parse::<Almanac>().expect("parse error");
    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.lookup_location(seed) as u32)
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let almanac = input.parse::<Almanac>().expect("parse error");
    almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .filter_map(|r| almanac.min_location(r))
        .map(|l| l as u32)
        .min()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

impl Almanac {
    pub fn lookup_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);

        self.humidity_to_location.map(humidity)
    }

    pub fn min_location(&self, input: Range<u64>) -> Option<u64> {
        let mappings = [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ];
        let mut ranges = vec![input];
        let mut next_ranges = vec![];
        for mapping in mappings {
            for range in ranges.drain(..) {
                next_ranges.extend(mapping.map_range(range));
            }
            std::mem::swap(&mut ranges, &mut next_ranges);
        }
        ranges.into_iter().filter_map(|r| r.min()).min()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AlmanacMap {
    ranges: Vec<RangeMapping>,
}

impl AlmanacMap {
    pub fn map(&self, input: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|mapping| mapping.map(input))
            .unwrap_or(input)
    }

    pub fn map_range(&self, input: Range<u64>) -> Vec<Range<u64>> {
        let mut input = input;
        let mut output = vec![];
        for range_mapping in &self.ranges {
            if range_mapping.source_range.start >= input.end {
                output.push(input);
                return output;
            }
            if input.start < range_mapping.source_range.start {
                output.push(input.start..range_mapping.source_range.start);
            }
            let start = std::cmp::max(input.start, range_mapping.source_range.start);
            let end = std::cmp::min(input.end, range_mapping.source_range.end);
            if end > start {
                output.push(
                    (start as i64 + range_mapping.offset) as u64
                        ..(end as i64 + range_mapping.offset) as u64,
                );
                if end < input.end {
                    input = end..input.end;
                } else {
                    return output;
                }
            }
        }
        output.push(input);
        output
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RangeMapping {
    source_range: Range<u64>,
    offset: i64,
}

impl RangeMapping {
    pub fn map(&self, input: u64) -> Option<u64> {
        if !self.source_range.contains(&input) {
            return None;
        }
        Some(((input as i64) + self.offset) as u64)
    }
}

#[derive(Debug)]
struct ParseAlmanacErr;

impl FromStr for Almanac {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<_>>();
        if parts.len() != 8 {
            return Err(ParseAlmanacErr);
        }
        let seeds = parts[0]
            .split_once(':')
            .ok_or(ParseAlmanacErr)?
            .1
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseAlmanacErr)?;
        let mut maps = parts
            .into_iter()
            .skip(1)
            .map(|s| {
                s.split_once(":\n")
                    .ok_or(ParseAlmanacErr)
                    .and_then(|(_, s)| s.parse::<AlmanacMap>())
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();
        Ok(Self {
            seeds,
            seed_to_soil: maps.next().ok_or(ParseAlmanacErr)?,
            soil_to_fertilizer: maps.next().ok_or(ParseAlmanacErr)?,
            fertilizer_to_water: maps.next().ok_or(ParseAlmanacErr)?,
            water_to_light: maps.next().ok_or(ParseAlmanacErr)?,
            light_to_temperature: maps.next().ok_or(ParseAlmanacErr)?,
            temperature_to_humidity: maps.next().ok_or(ParseAlmanacErr)?,
            humidity_to_location: maps.next().ok_or(ParseAlmanacErr)?,
        })
    }
}

impl FromStr for AlmanacMap {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s
            .lines()
            .map(|line| line.parse::<RangeMapping>())
            .collect::<Result<Vec<_>, _>>()?;
        ranges.sort_by_key(|r| r.source_range.start);
        Ok(Self { ranges })
    }
}

impl FromStr for RangeMapping {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseAlmanacErr)?;
        if parts.len() != 3 {
            return Err(ParseAlmanacErr);
        }
        Ok(Self {
            source_range: parts[1]..(parts[1] + parts[2]),
            offset: (parts[0] as i64) - (parts[1] as i64),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_map_range() {
        let almananc_map = AlmanacMap {
            ranges: vec![
                RangeMapping {
                    source_range: 50..98,
                    offset: 2,
                },
                RangeMapping {
                    source_range: 98..100,
                    offset: -48,
                },
            ],
        };
        assert_eq!(almananc_map.map_range(79..93), vec![81..95]);
        assert_eq!(almananc_map.map_range(90..100), vec![92..100, 50..52]);
        assert_eq!(
            almananc_map.map_range(90..102),
            vec![92..100, 50..52, 100..102]
        );

        let almanac_map = AlmanacMap {
            ranges: vec![
                RangeMapping {
                    source_range: 0..15,
                    offset: 39,
                },
                RangeMapping {
                    source_range: 15..52,
                    offset: -15,
                },
                RangeMapping {
                    source_range: 52..54,
                    offset: -15,
                },
            ],
        };
        assert_eq!(almanac_map.map_range(81..95), vec![81..95]);
    }
}
