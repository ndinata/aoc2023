use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

use anyhow::Result;
use nom::bytes::complete::take;
use nom::character::complete::{space1, u32};
use nom::sequence::separated_pair;
use nom::IResult;

pub mod part1 {
    use super::*;

    /// Possible cards, from weakest (`Two`) to strongest (`A`).
    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        J,
        Q,
        K,
        A,
    }

    impl TryFrom<char> for Card {
        type Error = String;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Self::A),
                'K' => Ok(Self::K),
                'Q' => Ok(Self::Q),
                'J' => Ok(Self::J),
                'T' => Ok(Self::Ten),
                '9' => Ok(Self::Nine),
                '8' => Ok(Self::Eight),
                '7' => Ok(Self::Seven),
                '6' => Ok(Self::Six),
                '5' => Ok(Self::Five),
                '4' => Ok(Self::Four),
                '3' => Ok(Self::Three),
                '2' => Ok(Self::Two),
                _ => Err("cannot parse card from invalid char".to_string()),
            }
        }
    }

    /// Possible hand type, from weakest (`HighCard`) to strongest (`FiveKind`).
    #[derive(Eq, Ord, PartialEq, PartialOrd)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeKind,
        FullHouse,
        FourKind,
        FiveKind,
    }

    impl From<&Vec<Card>> for HandType {
        fn from(value: &Vec<Card>) -> Self {
            // Count each distinct card type
            let mut card_counts: HashMap<Card, usize> = HashMap::new();
            for card in value {
                card_counts
                    .entry(*card)
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
            }

            // Deduce the hand type from the counts
            match card_counts.values().collect::<Vec<_>>()[..] {
                [5] => Self::FiveKind,
                [1, 4] | [4, 1] => Self::FourKind,
                [2, 3] | [3, 2] => Self::FullHouse,
                [1, 1, 3] | [1, 3, 1] | [3, 1, 1] => Self::ThreeKind,
                [1, 2, 2] | [2, 1, 2] | [2, 2, 1] => Self::TwoPair,
                [1, 1, 1, 2] | [1, 1, 2, 1] | [1, 2, 1, 1] | [2, 1, 1, 1] => {
                    Self::OnePair
                }
                [1, 1, 1, 1, 1] => Self::HighCard,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Eq, PartialEq)]
    struct Hand {
        cards: Vec<Card>,
        hand_type: HandType,
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.hand_type.cmp(&other.hand_type) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                // If two hands have equal hand types, then check each card
                // for ordering.
                Ordering::Equal => self
                    .cards
                    .iter()
                    .zip(&other.cards)
                    .find_map(|(mine, other)| {
                        (!mine.cmp(other).is_eq()).then_some(mine.cmp(other))
                    })
                    .unwrap(),
            }
        }
    }

    impl FromStr for Hand {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let cards = s
                .chars()
                .map(|char| char.try_into())
                .collect::<Result<Vec<Card>, _>>()?;

            let hand_type: HandType = (&cards).into();

            Ok(Self { cards, hand_type })
        }
    }

    pub fn run(input: &str) -> Result<String> {
        // We use a BTreeMap here because it produces items in key order, so
        // we auto get weakest to strongest `Hand`s when iterating through it.
        let hands = input
            .lines()
            .map(|input| parse_hand_bid(input).unwrap().1)
            .collect::<BTreeMap<Hand, u32>>();

        let total = hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i as u32 + 1) * bid)
            .sum::<u32>();

        Ok(total.to_string())
    }

    /// Parses the hand and bid from the input.
    fn parse_hand_bid(input: &str) -> IResult<&str, (Hand, u32)> {
        separated_pair(take(5usize), space1, u32)(input)
            .map(|(input, (hand, bid))| (input, (hand.parse().unwrap(), bid)))
    }
}

pub mod part2 {
    use super::*;

    /// Possible cards, from weakest (`J`) to strongest (`A`).
    #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
    enum Card {
        J,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Q,
        K,
        A,
    }

    impl TryFrom<char> for Card {
        type Error = String;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Self::A),
                'K' => Ok(Self::K),
                'Q' => Ok(Self::Q),
                'J' => Ok(Self::J),
                'T' => Ok(Self::Ten),
                '9' => Ok(Self::Nine),
                '8' => Ok(Self::Eight),
                '7' => Ok(Self::Seven),
                '6' => Ok(Self::Six),
                '5' => Ok(Self::Five),
                '4' => Ok(Self::Four),
                '3' => Ok(Self::Three),
                '2' => Ok(Self::Two),
                _ => Err("cannot parse card from invalid char".to_string()),
            }
        }
    }

    /// Possible hand type, from weakest (`HighCard`) to strongest (`FiveKind`).
    #[derive(Eq, Ord, PartialEq, PartialOrd)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeKind,
        FullHouse,
        FourKind,
        FiveKind,
    }

    impl From<&Vec<Card>> for HandType {
        fn from(value: &Vec<Card>) -> Self {
            let mut wildcards = 0;
            let mut card_counts: HashMap<Card, usize> = HashMap::new();

            // We count each distinct card type, EXCEPT for `J`s (the wildcard).
            // For that we keep a different counter. Reason below.
            for card in value {
                if matches!(card, Card::J) {
                    wildcards += 1;
                } else {
                    card_counts
                        .entry(*card)
                        .and_modify(|count| {
                            *count += 1;
                        })
                        .or_insert(1);
                }
            }

            // `J` morphs into whatever makes the strongest hand, meaning if we
            // have a list of card counts, it will turn into the card with the
            // biggest count because then it'd have improved the hand:
            // [4] (`FourKind`) -> [5] (`FiveKind`, improved)
            // [1, 3] (`ThreeKind`) -> [1, 4] (`FourKind`, improved)
            // [2, 2] (`TwoPair`) -> [2, 3] (`FullHouse`, improved)
            // [1, 1, 2] (`OnePair`) -> [1, 1, 3] (`ThreeKind`, improved)
            //
            // For that reason, first we sort the counts so we can take the last
            // one (the biggest one)...
            let mut counts = card_counts.into_values().collect::<Vec<_>>();
            counts.sort();

            // ...and bump it by how many wildcards are in the hand.
            if let Some(last) = counts.last_mut() {
                *last += wildcards;
            } else {
                // If the list of counts is empty, it must be that the hand is
                // all wildcards (`JJJJJ`), so it's a `FiveKind`.
                counts.push(5);
            }

            // Deduce the hand type from the counts
            match counts[..] {
                [5] => Self::FiveKind,
                [1, 4] | [4, 1] => Self::FourKind,
                [2, 3] | [3, 2] => Self::FullHouse,
                [1, 1, 3] | [1, 3, 1] | [3, 1, 1] => Self::ThreeKind,
                [1, 2, 2] | [2, 1, 2] | [2, 2, 1] => Self::TwoPair,
                [1, 1, 1, 2] | [1, 1, 2, 1] | [1, 2, 1, 1] | [2, 1, 1, 1] => {
                    Self::OnePair
                }
                [1, 1, 1, 1, 1] => Self::HighCard,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Eq, PartialEq)]
    struct Hand {
        cards: Vec<Card>,
        hand_type: HandType,
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.hand_type.cmp(&other.hand_type) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                // If two hands have equal hand types, then check each card
                // for ordering.
                Ordering::Equal => self
                    .cards
                    .iter()
                    .zip(&other.cards)
                    .find_map(|(mine, other)| {
                        (!mine.cmp(other).is_eq()).then_some(mine.cmp(other))
                    })
                    .unwrap(),
            }
        }
    }

    impl FromStr for Hand {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let cards = s
                .chars()
                .map(|char| char.try_into())
                .collect::<Result<Vec<Card>, _>>()?;

            let hand_type: HandType = (&cards).into();

            Ok(Self { cards, hand_type })
        }
    }

    pub fn run(input: &str) -> Result<String> {
        // We use a BTreeMap here because it produces items in key order, so
        // we auto get weakest to strongest `Hand`s when iterating through it.
        let hands = input
            .lines()
            .map(|input| parse_hand_bid(input).unwrap().1)
            .collect::<BTreeMap<Hand, u32>>();

        let total = hands
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i as u32 + 1) * bid)
            .sum::<u32>();

        Ok(total.to_string())
    }

    /// Parses the hand and bid from the input.
    fn parse_hand_bid(input: &str) -> IResult<&str, (Hand, u32)> {
        separated_pair(take(5usize), space1, u32)(input)
            .map(|(input, (hand, bid))| (input, (hand.parse().unwrap(), bid)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ok() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!("6440", part1::run(input).unwrap());
    }

    #[test]
    fn part2_ok() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!("5905", part2::run(input).unwrap());
    }
}
