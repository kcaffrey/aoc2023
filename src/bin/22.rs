use rayon::iter::{IntoParallelIterator, ParallelIterator};
use smallvec::SmallVec;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u32> {
    let tower = build_tower(input);
    Some(
        (0..tower.bricks.len())
            .filter(|&brick| {
                !tower.supports[brick]
                    .iter()
                    .any(|&supported| tower.supported[supported].len() == 1)
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let tower = build_tower(input);
    Some(
        (0..tower.bricks.len())
            .into_par_iter()
            .map(|brick| {
                let mut fall_count = 0;
                let mut visited = vec![false; tower.bricks.len()];
                let mut stack = Vec::with_capacity(16);
                visited[brick] = true;
                stack.push(brick);
                while let Some(cur) = stack.pop() {
                    for &supported in &tower.supports[cur] {
                        if !visited[supported] {
                            let loose = tower.supported[supported]
                                .iter()
                                .filter(|&&base| !visited[base])
                                .count()
                                == 0;
                            if loose {
                                visited[supported] = true;
                                stack.push(supported);
                                fall_count += 1;
                            }
                        }
                    }
                }
                fall_count
            })
            .sum::<u32>(),
    )
}

fn build_tower(input: &str) -> Tower {
    let (mut bricks, (max_x, max_y)) = parse_input(input);
    bricks.sort_unstable();
    Tower::from_bricks(bricks, max_x, max_y)
}

fn parse_input(input: &str) -> (Vec<Brick>, (u16, u16)) {
    let mut input = input.as_bytes();
    let mut ret = Vec::with_capacity(input.len() / 16); // guess at total length
    let mut max_x = 0;
    let mut max_y = 0;

    while input.len() > 1 {
        let (rest, brick) = parse_brick(input);
        input = rest;
        if brick.ends[1].x > max_x {
            max_x = brick.ends[1].x
        }
        if brick.ends[1].y > max_y {
            max_y = brick.ends[1].y;
        }
        ret.push(brick);
    }

    (ret, (max_x, max_y))
}

fn parse_number(input: &[u8]) -> (&[u8], u16) {
    let mut ret = 0;
    for i in 0..input.len() {
        let ch = input[i];
        match ch {
            b'0'..=b'9' => {
                ret *= 10;
                ret += (ch - b'0') as u16;
            }
            _ => {
                return (&input[i..], ret);
            }
        }
    }
    (&input[input.len()..], ret)
}

fn parse_point3(input: &[u8]) -> (&[u8], Point3) {
    let (input, x) = parse_number(input);
    let (input, y) = parse_number(&input[1..]);
    let (input, z) = parse_number(&input[1..]);
    (input, Point3::new(x, y, z))
}

fn parse_brick(input: &[u8]) -> (&[u8], Brick) {
    let (input, mut left) = parse_point3(input);
    let (input, mut right) = parse_point3(&input[1..]);
    if right < left {
        std::mem::swap(&mut left, &mut right);
    }
    let brick = Brick {
        ends: [left, right],
    };
    (&input[1..], brick)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Point3 {
    x: u16,
    y: u16,
    z: u16,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point2 {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone)]
struct Grid2<T> {
    cells: Vec<T>,
    y_limit: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd)]
struct Brick {
    ends: [Point3; 2],
}

#[derive(Debug, Clone)]
struct Tower {
    bricks: Vec<Brick>,
    supports: Vec<SmallVec<[usize; 4]>>,
    supported: Vec<SmallVec<[usize; 4]>>,
    top_view: Grid2<(usize, u16)>,
}

impl Tower {
    pub fn from_bricks(bricks: Vec<Brick>, max_x: u16, max_y: u16) -> Self {
        let mut ret = Self {
            top_view: Grid2::new(max_x, max_y),
            supports: vec![Default::default(); bricks.len()],
            supported: vec![Default::default(); bricks.len()],
            bricks,
        };
        for i in 0..ret.bricks.len() {
            let brick = ret.bricks[i];
            let (_, floor_height) = ret.get_max_height(brick);
            let brick_height = brick.ends[1].z - brick.ends[0].z + floor_height + 1;
            for point in brick.xy_points() {
                let (loadbearing_index, loadbearing_height) = ret.top_view.get(point);
                if loadbearing_height == floor_height && loadbearing_height > 0 {
                    if !ret.supported[i].contains(&loadbearing_index) {
                        ret.supported[i].push(loadbearing_index);
                    }
                    if !ret.supports[loadbearing_index].contains(&i) {
                        ret.supports[loadbearing_index].push(i);
                    }
                }
                ret.top_view.set(point, (i, brick_height));
            }
        }
        ret
    }

    fn get_max_height(&self, brick: Brick) -> (usize, u16) {
        brick
            .xy_points()
            .map(|p| self.top_view.get(p))
            .max_by_key(|&(_, h)| h)
            .unwrap_or((0, 0))
    }
}

impl Point3 {
    pub const fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z }
    }

    pub const fn xy(&self) -> Point2 {
        Point2::new(self.x, self.y)
    }
}

impl Point2 {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl<T: Default + Clone> Grid2<T> {
    pub fn new(max_x: u16, max_y: u16) -> Self {
        Self {
            cells: vec![Default::default(); (max_x as usize + 1) * (max_y as usize + 1)],
            y_limit: max_y as usize + 1,
        }
    }
}

impl<T: Copy> Grid2<T> {
    pub fn get(&self, p: Point2) -> T {
        self.cells[p.x as usize * self.y_limit + p.y as usize]
    }

    pub fn set(&mut self, p: Point2, value: T) {
        self.cells[p.x as usize * self.y_limit + p.y as usize] = value;
    }
}

impl Brick {
    pub fn xy_points(&self) -> impl Iterator<Item = Point2> {
        let mut next = Some(self.ends[0].xy());
        let end = self.ends[1].xy();
        std::iter::from_fn(move || {
            let Some(ret) = next else {
                return None;
            };
            if ret.x < end.x {
                next = Some(Point2::new(ret.x + 1, ret.y));
            } else if ret.y < end.y {
                next = Some(Point2::new(ret.x, ret.y + 1));
            } else {
                next = None;
            }
            Some(ret)
        })
    }
}

impl Ord for Point3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.z.cmp(&other.z) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.x.cmp(&other.x) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.y.cmp(&other.y)
    }
}

impl PartialOrd for Point3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
