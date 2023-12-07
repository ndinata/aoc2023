use std::collections::HashMap;

use anyhow::Result;
use nom::bytes::complete::{tag, take, take_until1};
use nom::character::complete::{line_ending, multispace1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;
use num::Integer;

pub mod part1 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let (input, instructions) = parse_instruction(input).unwrap();
        let (_, map) = parse_nodes(input).unwrap();

        let mut count = 0;

        let mut current = "AAA";
        for instruction in instructions.chars().cycle() {
            count += 1;

            let (l, r) = map.get(current).unwrap();
            let next = if instruction == 'L' { l } else { r };

            // Stop when we finally reach "ZZZ", otherwise keep going according
            // to the instruction ("L" or "R").
            match next {
                &"ZZZ" => break,
                next => current = next,
            };
        }

        Ok(count.to_string())
    }

    /// Parses the instruction string (e.g. `LRL`).
    fn parse_instruction(input: &str) -> IResult<&str, &str> {
        terminated(take_until1("\n"), multispace1)(input)
    }

    /// Parses the map of each node to its left and right destinations.
    fn parse_nodes(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
        let (input, maps) = separated_list1(
            line_ending,
            separated_pair(
                take(3usize),
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(take(3usize), tag(", "), take(3usize)),
                    tag(")"),
                ),
            ),
        )(input)?;

        Ok((input, HashMap::from_iter(maps)))
    }
}

pub mod part2 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let (input, instructions) = parse_instruction(input).unwrap();
        let (_, map) = parse_nodes(input).unwrap();

        // Starting nodes are those ending with "A"
        let mut paths = map
            .keys()
            .filter(|key| key.ends_with('A'))
            .collect::<Vec<_>>();

        // This is a counter for the number of steps needed for EACH starting
        // node to reach its ending node (node ending with "Z").
        let mut steps = paths.iter().map(|_| 0).collect::<Vec<u64>>();

        for instruction in instructions.chars().cycle() {
            // Only stop when all nodes are ending nodes
            if paths.iter().all(|path| path.ends_with('Z')) {
                break;
            }

            for (path, step) in paths.iter_mut().zip(steps.iter_mut()) {
                // This particular node has reached its ending node, we can skip
                if path.ends_with('Z') {
                    continue;
                }

                // Keep upping the counter for this node until we find its
                // ending node.
                let (l, r) = map.get(path.to_owned()).unwrap();
                *path = if instruction == 'L' { l } else { r };
                *step += 1;
            }
        }

        // Apparently, the answer is achieved by LCM-ing all starting nodes'
        // number of steps to reach their own respective ending nodes? I never
        // would've guessed this — all credit goes to the comments at the AoC
        // subreddit (although they also seem baffled by how LCM turns out to
        // lead to the answer).
        let total = steps
            .into_iter()
            .reduce(|acc, step| acc.lcm(&step))
            .unwrap();

        Ok(total.to_string())
    }

    /// Parses the instruction string (e.g. `LRL`).
    fn parse_instruction(input: &str) -> IResult<&str, &str> {
        terminated(take_until1("\n"), multispace1)(input)
    }

    /// Parses the map of each node to its left and right destinations.
    fn parse_nodes(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
        let (input, maps) = separated_list1(
            line_ending,
            separated_pair(
                take(3usize),
                tag(" = "),
                delimited(
                    tag("("),
                    separated_pair(take(3usize), tag(", "), take(3usize)),
                    tag(")"),
                ),
            ),
        )(input)?;

        Ok((input, HashMap::from_iter(maps)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ok() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!("2", part1::run(input).unwrap());

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!("6", part1::run(input).unwrap());
    }

    #[test]
    fn part2_ok() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!("6", part2::run(input).unwrap());
    }
}
