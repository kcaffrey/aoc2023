use fxhash::{FxHashMap, FxHashSet};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks = input.lines().map(Brick::from).collect::<Vec<_>>();
    bricks.sort_unstable();
    let tower = Tower::from_bricks(bricks);
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
    let mut bricks = input.lines().map(Brick::from).collect::<Vec<_>>();
    bricks.sort_unstable();
    let tower = Tower::from_bricks(bricks);
    Some(
        (0..tower.bricks.len())
            .into_par_iter()
            .map(|brick| {
                let mut fall_count = 0;
                let mut visited = vec![false; tower.bricks.len()];
                let mut stack = Vec::new();
                visited[brick] = true;
                stack.push(brick);
                while let Some(cur) = stack.pop() {
                    for &supported in &tower.supports[cur] {
                        let loose = tower.supported[supported]
                            .iter()
                            .filter(|&&base| !visited[base])
                            .count()
                            == 0;
                        if loose && !visited[supported] {
                            visited[supported] = true;
                            stack.push(supported);
                            fall_count += 1;
                        }
                    }
                }
                fall_count
            })
            .sum::<u32>(),
    )
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd)]
struct Brick {
    ends: [Point3; 2],
}

#[derive(Debug, Default, Clone)]
struct Tower {
    bricks: Vec<Brick>,
    brick_heights: Vec<u16>,
    supports: Vec<FxHashSet<usize>>,
    supported: Vec<FxHashSet<usize>>,
    xy_heights: FxHashMap<Point2, (usize, u16)>,
}

impl Tower {
    pub fn from_bricks(bricks: Vec<Brick>) -> Self {
        let mut ret = Self::default();
        ret.bricks = bricks;
        ret.brick_heights = vec![0; ret.bricks.len()];
        ret.supports = vec![Default::default(); ret.bricks.len()];
        ret.supported = vec![Default::default(); ret.bricks.len()];
        for i in 0..ret.bricks.len() {
            let brick = ret.bricks[i];
            let (_, floor_height) = ret.get_max_height(brick);
            let brick_height = brick.ends[1].z - brick.ends[0].z + floor_height + 1;
            ret.brick_heights[i] = floor_height + 1;
            for point in brick.xy_points() {
                let (loadbearing_index, loadbearing_height) = ret.get_height(point);
                if loadbearing_height == floor_height && loadbearing_height > 0 {
                    ret.supported[i].insert(loadbearing_index);
                    ret.supports[loadbearing_index].insert(i);
                }
                ret.xy_heights.insert(point, (i, brick_height));
            }
        }
        ret
    }

    pub fn get_height(&self, xy: Point2) -> (usize, u16) {
        self.xy_heights.get(&xy).copied().unwrap_or((0, 0))
    }

    pub fn get_max_height(&self, brick: Brick) -> (usize, u16) {
        brick
            .xy_points()
            .map(|p| self.get_height(p))
            .max_by_key(|&(_, h)| h)
            .unwrap_or((0, 0))
    }
}

impl Point3 {
    #[allow(unused)]
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

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once('~').unwrap();
        let (mut left, mut right) = (Point3::from(left), Point3::from(right));
        if right < left {
            std::mem::swap(&mut left, &mut right);
        }
        Self {
            ends: [left, right],
        }
    }
}

impl From<&str> for Point3 {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',').map(|s| s.parse().unwrap());
        Self {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        }
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
