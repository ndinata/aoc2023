use std::collections::{HashMap, HashSet};

use anyhow::Result;

type Position = (i32, i32);

pub mod part1 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        // A list of "digit list"s (a number broken down into individual digits).
        // With an example input of ".12.....8.", this will store:
        // [ [((1,0), '1'), ((2,0), '2')], [((8,0), '8')] ]
        // This repr is useful to cover the positions of the entire number string.
        let mut numbers: Vec<Vec<(Position, char)>> = Vec::new();

        // Set of positions of all symbols encountered
        let mut symbol_positions: HashSet<Position> = HashSet::new();

        // Parse the input line-by-line...
        for (y, line) in input.lines().enumerate() {
            let mut char_iter = line.char_indices().peekable();

            // ...and char-by-char in each line.
            while let Some((x, char)) = char_iter.next() {
                match char {
                    '0'..='9' => {
                        let mut number = Vec::new();
                        number.push(((x as i32, y as i32), char));

                        // Consume consecutive digits while there's any
                        while let Some((_, '0'..='9')) = char_iter.peek() {
                            let (next_x, next_digit) =
                                char_iter.next().unwrap();
                            number
                                .push(((next_x as i32, y as i32), next_digit));
                        }

                        numbers.push(number);
                    }
                    '.' => (), // Ignore dots
                    _ => {
                        symbol_positions.insert((x as i32, y as i32));
                    }
                };
            }
        }

        let total = numbers
            .iter()
            .filter_map(|number| parse_part_number(&symbol_positions, number))
            .sum::<u32>();

        Ok(total.to_string())
    }

    /// Returns some part number if the given number is adjacent to any symbol,
    /// None otherwise.
    fn parse_part_number(
        symbols: &HashSet<Position>,
        number: &[(Position, char)],
    ) -> Option<u32> {
        let digit_positions =
            number.iter().map(|&(pos, _)| pos).collect::<Vec<_>>();

        // Calculate set of all neighbouring positions of the number
        let neighbours = number
            .iter()
            .flat_map(|&((x, y), _)| {
                [
                    (x, y - 1),     // top
                    (x, y + 1),     // bottom
                    (x - 1, y),     // left
                    (x + 1, y),     // right
                    (x - 1, y - 1), // top-left
                    (x + 1, y - 1), // top-right
                    (x - 1, y + 1), // bottom-left
                    (x + 1, y + 1), // bottom-right
                ]
            })
            // Neighbours that contain digits are not included tho
            .filter(|pos| !digit_positions.contains(pos))
            .collect::<HashSet<_>>();

        // If any of the number's neighbours is a symbol, we consider it a
        // part number.
        neighbours
            .iter()
            .any(|neighbour| symbols.get(neighbour).is_some())
            .then(|| {
                // Combine the list of digits into a full number
                number
                    .iter()
                    .map(|(_, digit)| digit)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
    }
}

pub mod part2 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        // Repr of a list of "digit list"s (numbers), similar to part 1 above.
        let mut numbers: Vec<Vec<(Position, char)>> = Vec::new();

        // Set of positions of only asterisk symbols
        let mut asterisks: HashSet<Position> = HashSet::new();

        // Mapping of possible gears (asterisks with >=2 adjacent part numbers)
        let mut gear_candidates: HashMap<Position, (usize, u32)> =
            HashMap::new();

        // Parse the input line-by-line...
        for (y, line) in input.lines().enumerate() {
            let mut char_iter = line.char_indices().peekable();

            // ...and char-by-char in each line.
            while let Some((x, char)) = char_iter.next() {
                match char {
                    '*' => {
                        asterisks.insert((x as i32, y as i32));
                    }
                    '0'..='9' => {
                        let mut number = Vec::new();
                        number.push(((x as i32, y as i32), char));

                        // Consume consecutive digits while there's any
                        while let Some((_, '0'..='9')) = char_iter.peek() {
                            let (next_x, next_digit) =
                                char_iter.next().unwrap();
                            number
                                .push(((next_x as i32, y as i32), next_digit));
                        }

                        numbers.push(number);
                    }
                    _ => (), // Only care about '*' and numbers, ignore others
                };
            }
        }

        for number in numbers {
            let digit_positions =
                number.iter().map(|&(pos, _)| pos).collect::<Vec<_>>();

            // Calculate set of all neighbouring positions of the number
            let neighbours = number
                .iter()
                .flat_map(|&((x, y), _)| {
                    [
                        (x, y - 1),     // top
                        (x, y + 1),     // bottom
                        (x - 1, y),     // left
                        (x + 1, y),     // right
                        (x - 1, y - 1), // top-left
                        (x + 1, y - 1), // top-right
                        (x - 1, y + 1), // bottom-left
                        (x + 1, y + 1), // bottom-right
                    ]
                })
                // Neighbours that contain digits are not included tho
                .filter(|pos| !digit_positions.contains(pos))
                .collect::<HashSet<_>>();

            // For each neighbour of the number, if it happens to be an asterisk,
            // we add it as a gear candidate, keeping track of the number of
            // numbers it has seen, along with the actual numbers.
            for neighbour in neighbours {
                if asterisks.get(&neighbour).is_some() {
                    // Combine the list of digits into a full number
                    let number = number
                        .iter()
                        .map(|(_, digit)| digit)
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();

                    gear_candidates
                        .entry(neighbour)
                        .and_modify(|(count, num)| {
                            *count += 1;
                            *num *= number;
                        })
                        .or_insert((1, number));
                }
            }
        }

        // Any asterisks with >=2 numbers are gears, and we sum up their gear
        // ratios for the puzzle answer.
        let total = gear_candidates
            .iter()
            .filter(|(_, (count, _))| *count >= 2)
            .fold(0, |acc, (_, (_, number))| acc + number);

        Ok(total.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ok() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!("4361", part1::run(input).unwrap());
    }

    #[test]
    fn part2_ok() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!("467835", part2::run(input).unwrap());
    }
}
