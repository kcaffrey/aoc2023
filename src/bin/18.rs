advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i64> {
    let mut border_points = 0;
    let mut area = 0;
    let mut prev = Point::origin();
    for line in input
        .as_bytes()
        .split(|&ch| ch == b'\n')
        .filter(|line| !line.is_empty())
    {
        let dir: Direction = line[0].try_into().expect("valid direction");
        let distance = line[2..]
            .iter()
            .copied()
            .take_while(|&ch| ch != b' ')
            .fold(0, |acc, ch| acc * 10 + i64::from(ch - b'0'));
        let next = prev.move_in_dir(dir, distance);
        area += prev.x * next.y - prev.y * next.x;
        border_points += distance;
        prev = next;
    }
    area = area.abs() / 2;
    Some(area + border_points / 2 + 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut border_points = 0;
    let mut area = 0;
    let mut prev = Point::origin();
    for line in input
        .as_bytes()
        .split(|&ch| ch == b'\n')
        .filter(|line| !line.is_empty())
    {
        let hex = &line[line.len() - 7..line.len() - 1];
        let dir: Direction = hex[5].try_into().expect("valid direction");
        let distance = hex[..5].iter().copied().fold(0, |acc, ch| {
            acc * 16 + char::from(ch).to_digit(16).unwrap() as i64
        });
        let next = prev.move_in_dir(dir, distance);
        area += prev.x * next.y - prev.y * next.x;
        border_points += distance;
        prev = next;
    }
    area = area.abs() / 2;
    Some(area + border_points / 2 + 1)
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
