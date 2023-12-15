advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .split(|&ch| ch == b',')
            .map(|s| {
                s.iter()
                    .copied()
                    .filter(|&ch| ch != b'\n')
                    .fold(0, |acc, ch| ((acc + ch as u32) * 17) % 256)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = LensMap::new();
    for op in input
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
    boxes: [List<Lens<'a>>; 256],
}

struct List<T> {
    head: Option<usize>,
    tail: Option<usize>,
    elems: Vec<Option<ListElement<T>>>,
    next_free: usize,
    len: usize,
}

struct ListElement<T> {
    value: T,
    next: Option<usize>,
    prev: Option<usize>,
}

impl<'a> LensMap<'a> {
    pub const fn new() -> Self {
        const EMPTY_LIST: List<Lens> = List::new();
        Self {
            boxes: [EMPTY_LIST; 256],
        }
    }

    pub fn upsert(&mut self, lens: Lens<'a>) {
        let hash = self.hash(lens.label) as usize;
        if let Some(el) = self.boxes[hash].find(|el| el.label == lens.label) {
            el.focal_length = lens.focal_length;
            return;
        }
        self.boxes[hash].push(lens);
    }

    pub fn remove(&mut self, label: &[u8]) {
        let hash = self.hash(label) as usize;
        self.boxes[hash].remove_one_with(|el| el.label == label);
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

impl<T> List<T> {
    pub const fn new() -> Self {
        Self {
            elems: vec![],
            next_free: 0,
            head: None,
            tail: None,
            len: 0,
        }
    }

    #[allow(unused)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[allow(unused)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.into_iter()
    }

    pub fn find<F: Fn(&T) -> bool>(&mut self, f: F) -> Option<&mut T> {
        self.elems
            .iter_mut()
            .flatten()
            .find_map(|el| f(&el.value).then_some(&mut el.value))
    }

    pub fn push(&mut self, elem: T) {
        if self.next_free >= self.elems.len() {
            self.elems
                .resize_with((2 * self.elems.len()).max(8), Default::default);
        }
        let index = self.next_free;
        self.elems[index] = Some(ListElement {
            value: elem,
            next: None,
            prev: None,
        });
        self.len += 1;
        match self.tail {
            None => {
                self.head = Some(index);
                self.tail = Some(index);
            }
            Some(tail) => {
                self.elems[tail].as_mut().unwrap().next = Some(index);
                self.elems[index].as_mut().unwrap().prev = Some(tail);
                self.tail = Some(index);
            }
        }
        self.find_next_free();
    }

    pub fn remove_one_with<F: Fn(&T) -> bool>(&mut self, f: F) {
        let Some(index) =
            (0..self.elems.len()).find(|&i| matches!(&self.elems[i], Some(elem) if f(&elem.value)))
        else {
            return;
        };
        let (prev, next) = {
            let elem = self.elems[index].as_ref().unwrap();
            (elem.prev, elem.next)
        };
        if let Some(Some(prev)) = self.elems[index]
            .as_ref()
            .unwrap()
            .prev
            .map(|i| &mut self.elems[i])
        {
            prev.next = next;
        }
        if let Some(Some(next)) = self.elems[index]
            .as_ref()
            .unwrap()
            .next
            .map(|i| &mut self.elems[i])
        {
            next.prev = prev;
        }
        if matches!(self.head, Some(i) if i == index) {
            self.head = next;
        }
        if matches!(self.tail, Some(i) if i == index) {
            self.tail = prev;
        }
        self.elems[index] = None;
        self.len -= 1;
        if self.next_free >= self.elems.len() {
            self.next_free = index;
        }
    }

    fn find_next_free(&mut self) {
        while self.next_free < self.elems.len() && self.elems[self.next_free].is_some() {
            self.next_free += 1;
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;

    type IntoIter = ListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            list: self,
            next: self.head,
        }
    }
}
struct ListIterator<'a, T> {
    list: &'a List<T>,
    next: Option<usize>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next?;
        let elem = self.list.elems[next]
            .as_ref()
            .expect("should be an element at the stored index");
        self.next = elem.next;
        Some(&elem.value)
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

    #[test]
    fn test_list() {
        let mut list = List::<u32>::new();
        assert!(list.is_empty());
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![]);

        list.push(5);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![5]);

        list.remove_one_with(|&el| el < 4);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 1);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![5]);

        list.remove_one_with(|&el| el < 10);
        assert!(list.is_empty());
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![]);

        list.push(1);
        list.push(2);
        list.push(3);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);

        list.remove_one_with(|&el| el == 2);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 3]);

        list.push(5);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 3, 5]);

        list.remove_one_with(|&el| el > 1);
        list.remove_one_with(|&el| el > 1);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1]);

        for i in 2..20 {
            list.push(i);
        }
        assert_eq!(
            list.iter().copied().collect::<Vec<_>>(),
            (1..20).collect::<Vec<_>>()
        );
    }
}
