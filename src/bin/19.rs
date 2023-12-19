use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let (workflows, start) = parse_workflows(workflows);
    let mut sum = 0;
    for part in parts.lines().map(Part::from) {
        let mut cur = start;
        loop {
            let workflow = &workflows[cur as usize];
            cur = match workflow.next(part) {
                Destination::Accept => {
                    sum += part.rating_sum();
                    break;
                }
                Destination::Reject => break,
                Destination::Next(d) => d,
            };
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let (workflows, start) = parse_workflows(workflows);

    // DFS until we find accept nodes. Each path to an accept node results
    // in a volume of possible ratings. The union of those volumes is our answer.
    let mut stack = Vec::new();
    stack.push((start, PartFilter::new(1, 4000)));
    let mut volume = 0;
    while let Some((cur, filter)) = stack.pop() {
        let workflow = &workflows[cur as usize];
        let mut workflow_filter = Some(filter);
        for rule in &workflow.rules {
            if let Some(new_filter) =
                workflow_filter.and_then(|f| f.constrain(rule.category, rule.test))
            {
                match rule.destination {
                    Destination::Accept => volume += new_filter.volume(),
                    Destination::Next(d) => stack.push((d, new_filter)),
                    Destination::Reject => {}
                }
            }
            workflow_filter =
                workflow_filter.and_then(|f| f.constrain(rule.category, rule.test.invert()));
            if workflow_filter.is_none() {
                break;
            }
        }
        if let Some(filter) = workflow_filter {
            match workflow.default_rule {
                Destination::Accept => volume += filter.volume(),
                Destination::Next(d) => stack.push((d, filter)),
                Destination::Reject => {}
            }
        }
    }
    Some(volume)
}

fn parse_workflows(input: &str) -> (Vec<Workflow>, u16) {
    let mut next_id = 0;
    let mut name_to_id = HashMap::new();
    let mut workflows = Vec::new();
    let mut start = 0;
    for line in input.lines() {
        let rule_start = line.find('{').unwrap();
        let name = &line[..rule_start];
        let name_id = *name_to_id.entry(name).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        if name == "in" {
            start = name_id;
        }
        let last_comma = line.rfind(',').unwrap();
        let default_rule = match &line[last_comma + 1..line.len() - 1] {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            d => {
                let name_id = *name_to_id.entry(d).or_insert_with(|| {
                    let id = next_id;
                    next_id += 1;
                    id
                });
                Destination::Next(name_id)
            }
        };
        let rules = line[rule_start + 1..last_comma]
            .split(',')
            .map(|s| {
                let (test, destination) = s.split_once(':').unwrap();
                let (category, value) = test.split_once(|ch| ch == '<' || ch == '>').unwrap();
                let value = value.parse::<u32>().unwrap();
                let test = match test.chars().nth(1).unwrap() {
                    '>' => RatingRange::greater_than(value),
                    '<' => RatingRange::less_than(value),
                    _ => unreachable!("unexpected rule test: {}", test),
                };
                let destination = match destination {
                    "A" => Destination::Accept,
                    "R" => Destination::Reject,
                    d => {
                        let name_id = *name_to_id.entry(d).or_insert_with(|| {
                            let id = next_id;
                            next_id += 1;
                            id
                        });
                        Destination::Next(name_id)
                    }
                };
                Rule {
                    category: category.into(),
                    test,
                    destination,
                }
            })
            .collect::<Vec<_>>();
        let workflow = Workflow {
            name: name_id,
            rules,
            default_rule,
        };
        if name_id as usize >= workflows.len() {
            workflows.resize_with(name_id as usize + 1, Workflow::default);
        }
        workflows[name_id as usize] = workflow;
    }
    (workflows, start)
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    name: u16,
    rules: Vec<Rule>,
    default_rule: Destination,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Rule {
    category: Category,
    test: RatingRange,
    destination: Destination,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
enum Destination {
    Accept,
    #[default]
    Reject,
    Next(u16),
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct RatingRange {
    lower_bound: Option<u32>,
    upper_bound: Option<u32>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct PartFilter {
    x: RatingRange,
    m: RatingRange,
    a: RatingRange,
    s: RatingRange,
}

impl Workflow {
    pub fn next(&self, part: Part) -> Destination {
        self.rules
            .iter()
            .filter_map(|&rule| rule.test(part))
            .next()
            .unwrap_or(self.default_rule)
    }
}

impl Rule {
    pub fn test(&self, part: Part) -> Option<Destination> {
        self.test
            .contains(part.get(self.category))
            .then_some(self.destination)
    }
}

impl RatingRange {
    pub const fn less_than(value: u32) -> Self {
        Self {
            lower_bound: None,
            upper_bound: Some(value),
        }
    }

    pub const fn greater_than(value: u32) -> Self {
        Self {
            lower_bound: Some(value),
            upper_bound: None,
        }
    }

    pub const fn range(lower: u32, upper: u32) -> Self {
        Self {
            lower_bound: Some(lower),
            upper_bound: Some(upper),
        }
    }

    pub const fn contains(&self, value: u32) -> bool {
        match (self.lower_bound, self.upper_bound) {
            (None, None) => true,
            (Some(lower), None) => value > lower,
            (None, Some(upper)) => value < upper,
            (Some(lower), Some(upper)) => value > lower && value < upper,
        }
    }

    pub fn invert(&self) -> Self {
        match (self.lower_bound, self.upper_bound) {
            (None, Some(upper)) => Self::greater_than(upper - 1),
            (Some(lower), None) => Self::less_than(lower + 1),
            _ => panic!("should only call invert on an open range"),
        }
    }

    pub fn len(&self) -> u32 {
        let (Some(lower), Some(upper)) = (self.lower_bound, self.upper_bound) else {
            // We don't really care about this case.
            unreachable!("length of an open range is infinite");
        };
        upper - lower - 1
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        match (
            self.lower_bound,
            self.upper_bound,
            other.lower_bound,
            other.upper_bound,
        ) {
            (None, None, _, _) => Some(other),
            (_, _, None, None) => Some(self),
            (None, Some(a), None, Some(b)) => Some(Self::less_than(a.min(b))),
            (Some(a), None, Some(b), None) => Some(Self::greater_than(a.max(b))),
            (None, Some(a), Some(b), None) | (Some(b), None, None, Some(a)) => {
                (a.saturating_sub(b) > 1).then(|| Self::range(b, a))
            }
            (None, Some(u2), Some(l), Some(u1)) | (Some(l), Some(u1), None, Some(u2)) => {
                (u2.saturating_sub(l) > 1).then(|| Self::range(l, u1.min(u2)))
            }
            (Some(l2), None, Some(l1), Some(u)) | (Some(l1), Some(u), Some(l2), None) => {
                (u.saturating_sub(l2) > 1).then(|| Self::range(l1.max(l2), u))
            }
            (Some(l1), Some(u1), Some(l2), Some(u2)) => (u1.saturating_sub(l2) > 1
                && u2.saturating_sub(l1) > 1)
                .then(|| Self::range(l1.max(l2), u1.min(u2))),
        }
    }
}

impl Part {
    pub const fn get(&self, category: Category) -> u32 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    pub const fn rating_sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl PartFilter {
    pub const fn new(minimum_rating: u32, maximum_rating: u32) -> Self {
        Self {
            x: RatingRange::range(minimum_rating - 1, maximum_rating + 1),
            m: RatingRange::range(minimum_rating - 1, maximum_rating + 1),
            a: RatingRange::range(minimum_rating - 1, maximum_rating + 1),
            s: RatingRange::range(minimum_rating - 1, maximum_rating + 1),
        }
    }

    pub fn constrain(self, category: Category, range: RatingRange) -> Option<Self> {
        let mut ret = self;
        match category {
            Category::X => ret.x = self.x.intersection(range)?,
            Category::M => ret.m = self.m.intersection(range)?,
            Category::A => ret.a = self.a.intersection(range)?,
            Category::S => ret.s = self.s.intersection(range)?,
        }
        Some(ret)
    }

    pub fn volume(&self) -> u64 {
        self.x.len() as u64 * self.m.len() as u64 * self.a.len() as u64 * self.s.len() as u64
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let value = value.trim_matches(|ch| ch == '{' || ch == '}');
        let mut part = Self::default();
        for rating in value.split(',') {
            let (category, value) = rating.split_once('=').unwrap();
            let value = value.parse::<u32>().unwrap();
            match Category::from(category) {
                Category::X => part.x = value,
                Category::M => part.m = value,
                Category::A => part.a = value,
                Category::S => part.s = value,
            }
        }
        part
    }
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!("invalid category: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }

    #[test]
    fn test_part_filter() {
        let filter = PartFilter::new(1, 4000);
        let result = filter.constrain(Category::A, RatingRange::less_than(3000));
        assert_eq!(
            result,
            Some(PartFilter {
                x: RatingRange::range(0, 4001),
                m: RatingRange::range(0, 4001),
                a: RatingRange::range(0, 3000),
                s: RatingRange::range(0, 4001)
            })
        );
        let filter = result.unwrap();
        let result = filter.constrain(Category::A, RatingRange::greater_than(3000));
        assert_eq!(result, None);
        let result = filter.constrain(Category::A, RatingRange::range(1000, 3500));
        assert_eq!(
            result,
            Some(PartFilter {
                x: RatingRange::range(0, 4001),
                m: RatingRange::range(0, 4001),
                a: RatingRange::range(1000, 3000),
                s: RatingRange::range(0, 4001)
            })
        );
        let filter = result.unwrap();
        let result = filter.constrain(Category::A, RatingRange::range(2999, 4001));
        assert_eq!(result, None);

        assert_eq!(filter.volume(), 4000 * 4000 * 1999 * 4000);
    }
}
