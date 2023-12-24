advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones = parse_hailstones(input);
    Some(count_xy_intersections(
        hailstones,
        200000000000000,
        400000000000000,
    ))
}

fn count_xy_intersections<T: AsRef<[Hailstone]>>(
    hailstones: T,
    min_position: i64,
    max_position: i64,
) -> u32 {
    let hailstones = hailstones.as_ref();
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let mut a = hailstones[i];
            let mut b = hailstones[j];
            if a.velocity.x == 0 || b.velocity.y * a.velocity.x == a.velocity.y * b.velocity.x {
                std::mem::swap(&mut a, &mut b);
            }
            if a.velocity.x == 0 || b.velocity.y * a.velocity.x == a.velocity.y * b.velocity.x {
                // Can't intersect?
                continue;
            }
            let t = (a.position.y * a.velocity.x + a.velocity.y * b.position.x
                - a.velocity.y * a.position.x
                - b.position.y * a.velocity.x) as f64
                / (b.velocity.y * a.velocity.x - a.velocity.y * b.velocity.x) as f64;
            let s = ((b.position.x - a.position.x) as f64 + b.velocity.x as f64 * t)
                / a.velocity.x as f64;
            if t < 0.0 || s < 0.0 {
                // Intersection in past.
                continue;
            }
            let x = a.position.x as f64 + a.velocity.x as f64 * s;
            let y = a.position.y as f64 + a.velocity.y as f64 * s;
            if x >= min_position as f64
                && x <= max_position as f64
                && y >= min_position as f64
                && y <= max_position as f64
            {
                count += 1;
            }
        }
    }
    count
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_hailstones(input: &str) -> Vec<Hailstone> {
    let mut input = input.as_bytes();
    let width = input.iter().position(|&ch| ch == b'\n').unwrap();
    let estimated_rows = (input.len() + 1) / (width + 1);
    let mut hailstones = Vec::with_capacity(estimated_rows);
    while input.len() > 1 {
        let (rest, hailstone) = parse_hailstone(input);
        input = &rest[1.min(rest.len())..];
        hailstones.push(hailstone);
    }
    hailstones
}

fn parse_hailstone(input: &[u8]) -> (&[u8], Hailstone) {
    let (input, position) = parse_point3(input);
    let input = &input[input
        .iter()
        .position(|&ch| ch != b'@' && ch != b' ')
        .unwrap()..];
    let (input, velocity) = parse_point3(input);
    (input, Hailstone { position, velocity })
}

fn parse_point3(input: &[u8]) -> (&[u8], Point3) {
    let (input, x) = parse_num(input);
    let input = &input[input
        .iter()
        .position(|&ch| ch != b',' && ch != b' ')
        .unwrap()..];
    let (input, y) = parse_num(input);
    let input = &input[input
        .iter()
        .position(|&ch| ch != b',' && ch != b' ')
        .unwrap()..];
    let (input, z) = parse_num(input);
    (input, Point3 { x, y, z })
}

fn parse_num(mut input: &[u8]) -> (&[u8], i64) {
    let sign = if input[0] == b'-' {
        input = &input[1..];
        -1
    } else {
        1
    };
    let mut num = 0;
    for i in 0..input.len() {
        let ch = input[i];
        match ch {
            b'0'..=b'9' => num = num * 10 + (ch - b'0') as i64,
            _ => {
                return (&input[i..], num * sign);
            }
        }
    }
    (&input[input.len()..], num * sign)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Hailstone {
    position: Point3,
    velocity: Point3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let hailstones = parse_hailstones(&advent_of_code::template::read_file("examples", DAY));
        let result = count_xy_intersections(hailstones, 7, 27);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
