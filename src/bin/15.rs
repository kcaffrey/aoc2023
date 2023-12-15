advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hash_sum = 0u32;
    let mut cur_hash = 0u8;
    for ch in input.as_bytes() {
        match *ch {
            b',' | b'\n' => {
                hash_sum += cur_hash as u32;
                cur_hash = 0;
            }
            ch => cur_hash = cur_hash.wrapping_add(ch).wrapping_mul(17),
        }
    }
    Some(hash_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = LensMap::new();
    for op in input
        .trim()
        .as_bytes()
        .split(|&ch| ch == b',')
        .map(Operation::from)
    {
        match op {
            Operation::Remove(label) => map.remove(label),
            Operation::Add(lens) => map.upsert(lens),
        }
    }
    Some(map.focusing_power())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Lens<'a> {
    label: &'a [u8],
    focal_length: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation<'a> {
    Remove(&'a [u8]),
    Add(Lens<'a>),
}

impl<'a> From<&'a [u8]> for Operation<'a> {
    fn from(value: &'a [u8]) -> Self {
        let op_index = value
            .iter()
            .position(|&ch| ch == b'-' || ch == b'=')
            .unwrap();
        match value[op_index] {
            b'-' => Self::Remove(&value[..op_index]),
            b'=' => Self::Add(Lens {
                label: &value[..op_index],
                focal_length: (value[op_index + 1] as char).to_digit(10).unwrap() as u8,
            }),
            _ => unreachable!("invalid operation"),
        }
    }
}

struct LensMap<'a> {
    boxes: [Vec<Lens<'a>>; 256],
}

impl<'a> LensMap<'a> {
    pub const fn new() -> Self {
        const EMPTY_VEC: Vec<Lens> = Vec::new();
        Self {
            boxes: [EMPTY_VEC; 256],
        }
    }

    pub fn upsert(&mut self, lens: Lens<'a>) {
        let b = &mut self.boxes[self.hash(lens.label) as usize];
        if let Some(el) = b.iter_mut().find(|el| el.label == lens.label) {
            el.focal_length = lens.focal_length;
            return;
        }
        b.push(lens);
    }

    pub fn remove(&mut self, label: &[u8]) {
        let b = &mut self.boxes[self.hash(label) as usize];
        if let Some(index) = b.iter().position(|el| el.label == label) {
            b.remove(index);
        }
    }

    pub fn focusing_power(&self) -> u32 {
        (0..256)
            .map(|b| {
                self.boxes[b]
                    .iter()
                    .enumerate()
                    .map(|(s, el)| (b as u32 + 1) * (s as u32 + 1) * (el.focal_length as u32))
                    .sum::<u32>()
            })
            .sum()
    }

    fn hash(&self, label: &[u8]) -> u8 {
        label
            .iter()
            .copied()
            .filter(|&ch| ch != b'\n')
            .fold(0, |acc, ch| ((acc + ch as u16) * 17) % 256) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
