use std::collections::{HashMap, HashSet};

use anyhow::Result;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{space0, space1, u32};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

pub mod part1 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let total = input.lines().fold(0, |acc, line| {
            let (_, points) = parse_line(line).unwrap();

            acc + points
        });

        Ok(total.to_string())
    }

    /// Parses the number of points the input card is worth.
    pub(super) fn parse_line(input: &str) -> IResult<&str, u32> {
        // Split "Card x: " from rest of string
        let (rest, _) = preceded(take_until1(": "), tag(": "))(input)?;

        // Split sets of winning numbers and our numbers
        let (rest, (winning, ours)) =
            separated_pair(parse_numbers, tag("|"), parse_numbers)(rest)?;

        // Calculate points based on matching numbers
        let points = match winning.intersection(&ours).count() {
            0 => 0,
            count => 2_u32.pow(count as u32 - 1),
        };

        Ok((rest, points))
    }

    /// Parses the space-delimited set of numbers.
    fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
        let (rest, numbers) =
            delimited(space0, separated_list1(space1, u32), space0)(input)?;

        Ok((rest, HashSet::from_iter(numbers)))
    }
}

pub mod part2 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        // List of matching numbers
        let matches = input
            .lines()
            .map(|line| parse_line(line).unwrap().1)
            .collect::<Vec<_>>();

        // Counter of card copies
        let mut copies: HashMap<u32, u32> = HashMap::new();

        for (i, match_count) in matches.iter().enumerate() {
            let i = i as u32 + 1;

            // For the original card, keep track of copies won
            for id in (i + 1)..(i + 1 + match_count) {
                copies
                    .entry(id)
                    .and_modify(|instance| *instance += 1)
                    .or_insert(1);
            }

            // For the _copy_ of the card, keep track of further copies won
            if let Some(&count) = copies.get(&i) {
                for id in (i + 1)..(i + 1 + match_count) {
                    copies
                        .entry(id)
                        // `*instance += count` instead of `+= 1` here because
                        // _each_ copy wins more copies.
                        .and_modify(|instance| *instance += count)
                        .or_insert(1);
                }
            }
        }

        // Total = number of copies + number of originals
        let total = copies.values().sum::<u32>() + matches.len() as u32;

        Ok(total.to_string())
    }

    /// Parses the count of matching numbers the input card has.
    fn parse_line(input: &str) -> IResult<&str, u32> {
        // Split "Card x: " from rest of string
        let (rest, _) = preceded(take_until1(": "), tag(": "))(input)?;

        // Split sets of winning numbers and our numbers
        let (rest, (winning, ours)) =
            separated_pair(parse_numbers, tag("|"), parse_numbers)(rest)?;

        Ok((rest, winning.intersection(&ours).count() as u32))
    }

    /// Parses the space-delimited set of numbers.
    fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
        let (rest, numbers) =
            delimited(space0, separated_list1(space1, u32), space0)(input)?;

        Ok((rest, HashSet::from_iter(numbers)))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn part1_ok() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!("13", part1::run(input).unwrap());
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn part1_parse_line_ok(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, part1::parse_line(line).unwrap().1);
    }

    #[test]
    fn part2_ok() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!("30", part2::run(input).unwrap());
    }
}
