use std::str::FromStr;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let races = input.parse::<Races>().expect("input should parse");
    races
        .races
        .into_iter()
        .map(Race::number_of_ways_to_beat)
        .reduce(|acc, n| acc * n)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (time_str, distance_str) = input.split_once('\n')?;
    let time = time_str
        .split_once(':')?
        .1
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(u64::from))
        .reduce(|acc, d| acc * 10 + d)?;
    let distance = distance_str
        .split_once(':')?
        .1
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(u64::from))
        .reduce(|acc, d| acc * 10 + d)?;
    let race = Race {
        time,
        best_distance: distance,
    };
    Some(race.number_of_ways_to_beat())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Races {
    races: Vec<Race>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Race {
    time: u64,
    best_distance: u64,
}

impl Race {
    pub fn number_of_ways_to_beat(self) -> u64 {
        // If we charge for charge_time (charge_time <= race_time), then
        // the velocity (v) at the end will be v = charge_time.
        // We then have (race_time - charge_time) left in the race,
        // and distance = v * t = charge_time * (race_time - charge_time).
        //
        // We want to solve for distance > best_distance. If we solve for equality,
        // we get charge_time^2 - race_time*charge_time + best_distance = 0.
        //
        // Solving for charge_time using the quadratic formula results in
        // charge_time = 0.5 * (race_time +/- sqrt(race_time^2 - 4*best_distance))
        // We can compute these two bounds (taking the floor of the upper bound and ceiling
        // of the lower bound), check the value for the distance at those two points,
        // and if necessary shrink the range. This should then result in our answer:
        //    upper_bound - lower_bound + 1
        //
        let rt = self.time as f64;
        let bd = self.best_distance as f64;
        let mut lower_bound = (0.5 * (rt - (rt * rt - 4. * bd).sqrt())).ceil().max(0.) as u64;
        let mut upper_bound = (0.5 * (rt + (rt * rt - 4. * bd).sqrt())).floor().min(rt) as u64;
        if self.calculate_distance(lower_bound) <= self.best_distance {
            lower_bound += 1;
        }
        if self.calculate_distance(upper_bound) <= self.best_distance {
            upper_bound -= 1;
        }
        if upper_bound < lower_bound {
            return 0;
        }
        upper_bound - lower_bound + 1
    }

    pub fn calculate_distance(self, charge_time: u64) -> u64 {
        // See above
        charge_time * (self.time - charge_time)
    }
}

#[derive(Debug)]
struct ParseRacesErr;

impl FromStr for Races {
    type Err = ParseRacesErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (times_str, distances_str) = s.split_once('\n').ok_or(ParseRacesErr)?;
        let times = times_str
            .split_once(':')
            .ok_or(ParseRacesErr)?
            .1
            .split_whitespace()
            .map(|s| s.parse::<u64>().map_err(|_| ParseRacesErr));
        let distances = distances_str
            .split_once(':')
            .ok_or(ParseRacesErr)?
            .1
            .split_whitespace()
            .map(|s| s.parse::<u64>().map_err(|_| ParseRacesErr));
        Ok(Self {
            races: times
                .zip(distances)
                .map(|(t, d)| {
                    Ok(Race {
                        time: t?,
                        best_distance: d?,
                    })
                })
                .collect::<Result<Vec<_>, ParseRacesErr>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
