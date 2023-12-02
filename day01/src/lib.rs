use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::{iterator, value};
use nom::IResult;

pub mod part1 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let total = input.lines().fold(0, |acc, line| {
            let mut chars = line.chars();

            // Find the first number in the line
            let first = chars.find_map(|c| c.to_digit(10)).unwrap();

            // Find the last (first from the back) number in the line
            let last = chars
                .rfind(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .unwrap_or(first);

            acc + (first * 10) + last
        });

        Ok(total.to_string())
    }
}

pub mod part2 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let total = input.lines().fold(0, |acc, line| {
            let (_, number) = parse_line(line).unwrap();

            acc + number
        });

        Ok(total.to_string())
    }

    /// Tries to parse the "calibration value" from the line.
    pub(super) fn parse_line(line: &str) -> IResult<&str, u32> {
        // Repeatedly apply the `parse_number` parser until we get through the
        // end of the string, collecting only `Some(number)`s.
        let mut it = iterator(line, parse_number);
        let numbers = it.flatten().collect::<Vec<_>>();
        let (rest, _) = it.finish()?;

        // Alternative method:
        // use nom::multi::many1;
        // let (rest, numbers) = many1(parse_number)(line)?;
        // let numbers = numbers.into_iter().flatten().collect::<Vec<_>>();

        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap_or(first);

        Ok((rest, first * 10 + last))
    }

    /// Tries to parse some digit from the input string.
    ///
    /// We first try to parse a "number word" ("one", etc.) from the string.
    ///
    /// If successful, that's the digit we need — return a tuple containing
    /// it and the rest of the string (for any further processing).
    ///
    /// If not successful, we check if the current char is a digit or not.
    /// If it is, we're done — return the same thing as above. If not, we return
    /// a `None` as the digit (meaning no digit is found).
    fn parse_number(input: &str) -> IResult<&str, Option<u32>> {
        let num_word_parse: IResult<&str, u32> = alt((
            value(1, tag("one")),
            value(2, tag("two")),
            value(3, tag("three")),
            value(4, tag("four")),
            value(5, tag("five")),
            value(6, tag("six")),
            value(7, tag("seven")),
            value(8, tag("eight")),
            value(9, tag("nine")),
        ))(input);

        // Split the current (first) char from the rest of the string
        let (rest, char) = anychar(input)?;

        match num_word_parse {
            // We use the rest of the string from moving by one char here instead
            // of from the number word because number words may overlap. Example:
            // "twone" -> [2, 1]
            // If we used `rest` from `num_word_res`, it would be just `ne`
            // instead of `wone` (which would've enabled us to catch `one` later)
            Ok((_, digit)) => Ok((rest, Some(digit))),

            // Can't parse any number words — that's fine, check if the current
            // char is a digit or not.
            Err(_) => Ok((rest, char.to_digit(10))),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn part1_ok() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!("142", part1::run(input).unwrap());
    }

    #[test]
    fn part2_ok() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!("281", part2::run(input).unwrap());
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn part2_parse_line_ok(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, part2::parse_line(line).unwrap().1);
    }
}
