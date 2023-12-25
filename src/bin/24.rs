use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    (0..hailstones.len())
        .into_par_iter()
        .map(|i| {
            let mut count = 0;
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
            count
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<i64> {
    let hailstones = parse_hailstones(input);

    let (mut p, v) = hailstones
        .iter()
        .copied()
        .tuple_combinations()
        .filter_map(|(h0, h1, h2)| {
            let (x0, y0, z0) = h0.position.to_f64_tuple();
            let (dx0, dy0, dz0) = h0.velocity.to_f64_tuple();
            let (x1, y1, z1) = h1.position.to_f64_tuple();
            let (dx1, dy1, dz1) = h1.velocity.to_f64_tuple();
            let (x2, y2, z2) = h2.position.to_f64_tuple();
            let (dx2, dy2, dz2) = h2.velocity.to_f64_tuple();
            let a: Array2<f64> = array![
                [0., dz1 - dz0, dy0 - dy1, 0., z0 - z1, y1 - y0],
                [dz0 - dz1, 0., dx1 - dx0, z1 - z0, 0., x0 - x1],
                [dy1 - dy0, dx0 - dx1, 0., y0 - y1, x1 - x0, 0.],
                [0., dz2 - dz0, dy0 - dy2, 0., z0 - z2, y2 - y0],
                [dz0 - dz2, 0., dx2 - dx0, z2 - z0, 0., x0 - x2],
                [dy2 - dy0, dx0 - dx2, 0., y0 - y2, x2 - x0, 0.],
            ];
            let b: Array1<f64> = array![
                y1 * dz1 + dy0 * z0 - y0 * dz0 - dy1 * z1,
                z1 * dx1 + dz0 * x0 - z0 * dx0 - dz1 * x1,
                x1 * dy1 + dx0 * y0 - x0 * dy0 - dx1 * y1,
                y2 * dz2 + dy0 * z0 - y0 * dz0 - dy2 * z2,
                z2 * dx2 + dz0 * x0 - z0 * dx0 - dz2 * x2,
                x2 * dy2 + dx0 * y0 - x0 * dy0 - dx2 * y2,
            ];
            a.solve_into(b)
                .map(|x| {
                    (
                        Point3::new(
                            x[0].round() as i64,
                            x[1].round() as i64,
                            x[2].round() as i64,
                        ),
                        Point3::new(
                            x[3].round() as i64,
                            x[4].round() as i64,
                            x[5].round() as i64,
                        ),
                    )
                })
                .ok()
        })
        .take(1)
        .next()
        .unwrap();

    // Due to numerical precision issues, we may be slightly off on the x, y, and z coordinates.
    // Double check and if necessary modify with hailstone intersections to check.
    // For any hailstone, it's trivial to solve for t to find p + v * t = p0 + v0 * t
    // If we get different values of t for any of x, y, and z, we should modify one
    // component of the position vector to be consistent with the other two.
    let h = hailstones
        .into_iter()
        .find(|h| h.velocity.x != v.x && h.velocity.y != v.y && h.velocity.z != v.z)
        .unwrap();
    let tx = (p.x - h.position.x) / (h.velocity.x - v.x);
    let ty = (p.y - h.position.y) / (h.velocity.y - v.y);
    let tz = (p.z - h.position.z) / (h.velocity.z - v.z);
    if tx == ty && tx != tz {
        p.z = h.position.z + h.velocity.z * tx - v.z * tx;
    } else if tx == tz && tx != ty {
        p.y = h.position.y + h.velocity.y * tx - v.y * tx;
    } else if ty == tz && ty != tx {
        p.x = h.position.x + h.velocity.x * ty - v.x * ty;
    }

    Some(p.x + p.y + p.z)
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

impl Point3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
    pub const fn to_f64_tuple(self) -> (f64, f64, f64) {
        (self.x as f64, self.y as f64, self.z as f64)
    }
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
        assert_eq!(result, Some(47));
    }
}
