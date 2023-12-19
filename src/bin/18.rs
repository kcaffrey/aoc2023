advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve_part1_fast(input))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve_part2_fast(input))
}

fn solve_part1_fast(input: &str) -> i64 {
    let input = input.as_bytes();
    let mut border_points = 0;
    let mut area = 0;
    let mut prev = Point::new(0, 0);
    let mut index = 3;
    while index < input.len() - 1 {
        let start = index - 3;
        if input[index] == b' ' {
            index -= 1;
        }
        let distance: i64 = input[start + 2..=index]
            .iter()
            .fold(0, |acc, &ch| (acc << 4) + ((ch - b'0') as i64));
        let next = match input[start] {
            b'R' => Point::new(prev.x + distance, prev.y),
            b'D' => Point::new(prev.x, prev.y + distance),
            b'L' => Point::new(prev.x - distance, prev.y),
            b'U' => Point::new(prev.x, prev.y - distance),
            _ => unreachable!(),
        };
        index += 15;
        area += prev.x * next.y - prev.y * next.x;
        border_points += distance;
        prev = next;
    }
    area = area.abs() / 2;
    area + border_points / 2 + 1
}

fn solve_part2_fast(input: &str) -> i64 {
    let input = input.as_bytes();
    let mut border_points = 0;
    let mut area = 0;
    let mut prev = Point::new(0, 0);
    let mut index = 6;
    while index < input.len() - 1 {
        if input[index] == b'#' {
            index += 1;
        }
        let distance: i64 = input[index..index + 5].iter().fold(0, |acc, &ch| {
            let d = if ch < b'a' { ch - b'0' } else { ch - b'a' + 10 };
            (acc << 4) + (d as i64)
        });
        let next = match input[index + 5] {
            b'0' => Point::new(prev.x + distance, prev.y),
            b'1' => Point::new(prev.x, prev.y + distance),
            b'2' => Point::new(prev.x - distance, prev.y),
            b'3' => Point::new(prev.x, prev.y - distance),
            _ => unreachable!(),
        };
        index += 14;
        area += prev.x * next.y - prev.y * next.x;
        border_points += distance;
        prev = next;
    }
    area = area.abs() / 2;
    area + border_points / 2 + 1
}

#[allow(unused)]
fn solve<F: Fn(&[u8]) -> (Direction, i64)>(input: &str, parser: F) -> i64 {
    let mut border_points = 0;
    let mut area = 0;
    let mut prev = Point::origin();
    for (dir, distance) in input
        .as_bytes()
        .split(|&ch| ch == b'\n')
        .filter(|line| !line.is_empty())
        .map(parser)
    {
        let next = prev.move_in_dir(dir, distance);
        area += prev.x * next.y - prev.y * next.x;
        border_points += distance;
        prev = next;
    }
    area = area.abs() / 2;
    area + border_points / 2 + 1
}

#[allow(unused)]
fn parse_part1(line: &[u8]) -> (Direction, i64) {
    let dir: Direction = line[0].try_into().expect("valid direction");
    let distance = line[2..]
        .iter()
        .copied()
        .take_while(|&ch| ch != b' ')
        .fold(0, |acc, ch| acc * 10 + i64::from(ch - b'0'));
    (dir, distance)
}

#[allow(unused)]
fn parse_part2(line: &[u8]) -> (Direction, i64) {
    let hex = &line[line.len() - 7..line.len() - 1];
    let dir: Direction = hex[5].try_into().expect("valid direction");
    let distance = hex[..5].iter().copied().fold(0, |acc, ch| {
        acc * 16 + char::from(ch).to_digit(16).unwrap() as i64
    });
    (dir, distance)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn origin() -> Self {
        Self::new(0, 0)
    }

    pub const fn move_in_dir(self, dir: Direction, distance: i64) -> Self {
        match dir {
            Direction::North => Point::new(self.x, self.y - distance),
            Direction::South => Point::new(self.x, self.y + distance),
            Direction::East => Point::new(self.x + distance, self.y),
            Direction::West => Point::new(self.x - distance, self.y),
        }
    }
}

#[derive(Debug)]
struct InvalidDirectionError(char);

impl TryFrom<u8> for Direction {
    type Error = InvalidDirectionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'U' | b'3' => Self::North,
            b'D' | b'1' => Self::South,
            b'L' | b'2' => Self::West,
            b'R' | b'0' => Self::East,
            _ => return Err(InvalidDirectionError(char::from(value))),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
