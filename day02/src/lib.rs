use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u16, u32};
use nom::combinator::{cut, eof, fail};
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

pub mod part1 {
    use super::*;

    const MAX_RED: u16 = 12;
    const MAX_GREEN: u16 = 13;
    const MAX_BLUE: u16 = 14;

    pub fn run(input: &str) -> Result<String> {
        let total = input.lines().fold(0, |acc, line| {
            let (_, id) = parse_line(line).unwrap();

            acc + id.unwrap_or(0)
        });

        Ok(total.to_string())
    }

    /// Outputs the line's game ID if the cube sets are valid, None otherwise.
    pub(super) fn parse_line(line: &str) -> IResult<&str, Option<u16>> {
        let (rest, id) = parse_game_id(line)?;

        let id = match parse_game_sets(rest) {
            Ok(_) => Some(id),
            Err(_) => None,
        };

        Ok((rest, id))
    }

    fn parse_game_id(input: &str) -> IResult<&str, u16> {
        delimited(tag("Game "), u16, tag(": "))(input)
    }

    #[derive(Clone)]
    enum Cube {
        Red(u16),
        Green(u16),
        Blue(u16),
    }

    /// Parses the list of cubes in the given game (input).
    ///
    /// Exits with an error as soon as the first "impossible" cube is found.
    fn parse_game_sets(input: &str) -> IResult<&str, Vec<Vec<Cube>>> {
        // "3 blue; 1 red, 2 blue;" -> [[Cube::Blue(3)], [Cube::Red(1), Cube::Blue(2)]]
        // Notice the `cut(parse_cube)` here! This is what allows short-circuiting
        // the parsing as soon as an "impossible" cube is found.
        separated_list1(tag("; "), separated_list1(tag(", "), cut(parse_cube)))(
            input,
        )
    }

    /// Tries to parse an input like "2 red" into `Cube::Red(2)`.
    ///
    /// Fails if the input corresponds to an "impossible" cube.
    fn parse_cube(input: &str) -> IResult<&str, Cube> {
        // "2 red" -> (2, "red")
        let (rest, (count, colour)) = separated_pair(
            u16,
            char(' '),
            alt((tag("red"), tag("green"), tag("blue"))),
        )(input)?;

        // (2, "red") -> `Cube::Red(2)` ONLY if it's not "impossible", error
        // otherwise.
        let cube = match colour {
            "red" if count <= MAX_RED => Cube::Red(count),
            "green" if count <= MAX_GREEN => Cube::Green(count),
            "blue" if count <= MAX_BLUE => Cube::Blue(count),
            _ => return fail(input),
        };

        Ok((rest, cube))
    }
}

pub mod part2 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let total = input.lines().fold(0, |acc, line| {
            let (_, power) = parse_line(line).unwrap();

            acc + power
        });

        Ok(total.to_string())
    }

    /// Parses the line's/game's "power".
    pub(super) fn parse_line(line: &str) -> IResult<&str, u32> {
        preceded(parse_game_id, parse_game_power)(line)
    }

    fn parse_game_id(input: &str) -> IResult<&str, u16> {
        delimited(tag("Game "), u16, tag(": "))(input)
    }

    #[derive(Default)]
    struct CubeSet {
        red: u32,
        green: u32,
        blue: u32,
    }

    enum Cube {
        Red(u32),
        Green(u32),
        Blue(u32),
    }

    // Parses input like "3 blue; 1 red, 2 green; 2 green" into 6.
    fn parse_game_power(input: &str) -> IResult<&str, u32> {
        // `parse_cube` gets rid of separators like ", " and "; ", so we're good
        // to apply it repeatedly with `fold_many1`.
        let (rest, set) =
            fold_many1(parse_cube, CubeSet::default, |mut acc, cube| {
                // Record only the largest numbers of the different cubes
                match cube {
                    Cube::Red(c) if c > acc.red => acc.red = c,
                    Cube::Green(c) if c > acc.green => acc.green = c,
                    Cube::Blue(c) if c > acc.blue => acc.blue = c,
                    _ => (),
                };

                acc
            })(input)?;

        Ok((rest, set.red * set.green * set.blue))
    }

    /// Parses an input like "2 red" into `Cube::Red(2)`.
    fn parse_cube(input: &str) -> IResult<&str, Cube> {
        // "2 red" -> (2, "red")
        let (rest, (count, colour)) = separated_pair(
            u32,
            char(' '),
            alt((tag("red"), tag("green"), tag("blue"))),
        )(input)?;

        // Consume any suffix elements so subsequent parsings don't have to deal
        // with them.
        let (rest, _) = alt((tag(", "), tag("; "), eof))(rest)?;

        // (2, "red") => `Cube::Red(2)`
        let cube = match colour {
            "red" => Cube::Red(count),
            "green" => Cube::Green(count),
            "blue" => Cube::Blue(count),
            _ => unreachable!(),
        };

        Ok((rest, cube))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn part1_ok() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("8", part1::run(input).unwrap());
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Some(1))]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        Some(2)
    )]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", None)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", None)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Some(5))]
    fn part1_parse_line_ok(#[case] line: &str, #[case] expected: Option<u16>) {
        assert_eq!(expected, part1::parse_line(line).unwrap().1);
    }

    #[test]
    fn part2_ok() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", part2::run(input).unwrap());
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        12
    )]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn part2_parse_line_ok(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, part2::parse_line(line).unwrap().1);
    }
}
