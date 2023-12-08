use enum_ordinalize::Ordinalize;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    score(input, None)
}

pub fn part_two(input: &str) -> Option<u32> {
    score(input, Some(Card::Jack))
}

fn score(input: &str, joker: Option<Card>) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand_str, bid_str) = line.split_once(' ').ok_or(ParseHandErr)?;
            Ok::<_, ParseHandErr>((
                Hand::try_from_str(hand_str, joker)?,
                bid_str.parse::<u32>().map_err(|_| ParseHandErr)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("should parse hands correctly");
    hands.sort_unstable_by_key(|hand| hand.0);
    Some(
        hands
            .into_iter()
            .enumerate()
            .map(|(index, (_, bid))| (index as u32 + 1) * bid)
            .sum(),
    )
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Ordinalize)]
enum Card {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
enum HandType {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    cards_ordering: [usize; 5],
    joker: Option<Card>,
}

impl Hand {
    pub fn try_from_str(s: &str, joker: Option<Card>) -> Result<Self, ParseHandErr> {
        if s.len() != 5 {
            return Err(ParseHandErr);
        }
        let mut hand = Hand {
            joker,
            ..Default::default()
        };
        for (i, card) in s.chars().map(Card::try_from).enumerate() {
            let card = card?;
            hand.cards[i] = card;
            hand.cards_ordering[i] = match (joker, card) {
                (Some(j), c) if j == c => 0,
                (Some(_), c) => c.ordinal() as usize + 1,
                (None, c) => c.ordinal() as usize,
            }
        }
        hand.hand_type = hand_type(hand.cards, joker);
        Ok(hand)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.cards_ordering.cmp(&other.cards_ordering)
    }
}

fn hand_type(cards: [Card; 5], joker: Option<Card>) -> HandType {
    let mut counts = [0; 13];
    let mut joker_count = 0;
    for card in cards {
        if joker.filter(|&c| c == card).is_some() {
            joker_count += 1;
        } else {
            counts[card.ordinal() as usize] += 1;
        }
    }
    let mut five_count = 0;
    let mut four_count = 0;
    let mut three_count = 0;
    let mut two_count = 0;
    for count in counts {
        match count {
            5 => five_count += 1,
            4 => four_count += 1,
            3 => three_count += 1,
            2 => two_count += 1,
            _ => {}
        }
    }
    match (five_count, four_count, three_count, two_count, joker_count) {
        (1, _, _, _, _) => HandType::FiveOfAKind,
        (_, 1, _, _, 1) => HandType::FiveOfAKind,
        (_, _, 1, _, 2) => HandType::FiveOfAKind,
        (_, _, _, 1, 3) => HandType::FiveOfAKind,
        (_, _, _, _, 4) => HandType::FiveOfAKind,
        (_, _, _, _, 5) => HandType::FiveOfAKind,
        (_, 1, _, _, _) => HandType::FourOfAKind,
        (_, _, 1, _, 1) => HandType::FourOfAKind,
        (_, _, 0, 1, 2) => HandType::FourOfAKind,
        (_, _, 0, 0, 3) => HandType::FourOfAKind,
        (_, _, 1, 1, _) => HandType::FullHouse,
        (_, _, 0, 2, 1) => HandType::FullHouse,
        (_, _, 1, 0, _) => HandType::ThreeOfAKind,
        (_, _, 0, 1, 1) => HandType::ThreeOfAKind,
        (_, _, 0, 0, 2) => HandType::ThreeOfAKind,
        (_, _, _, 2, _) => HandType::TwoPair,
        (_, _, _, 1, _) => HandType::OnePair,
        (0, 0, 0, 0, 1) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

#[derive(Debug)]
struct ParseHandErr;

impl TryFrom<char> for Card {
    type Error = ParseHandErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => return Err(ParseHandErr),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
