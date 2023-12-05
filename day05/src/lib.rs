use anyhow::Result;
use nom::bytes::complete::{tag, take_till1, take_until1};
use nom::character::complete::{line_ending, multispace1, space1, u64};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

struct RangeMap {
    src_start: u64,
    dest_start: u64,
    range_len: u64,
}

impl RangeMap {
    /// Returns the destination version of `num` if in range, None otherwise.
    fn map(&self, num: u64) -> Option<u64> {
        #[allow(clippy::unnecessary_lazy_evaluations)]
        (self.src_start..(self.src_start + self.range_len))
            .contains(&num)
            // NOTE: we DO NOT wanna use `then_some` as per clippy's suggestions
            // because then the term `num - self.src_start` gets evaluated even
            // when `num` isn't in the range, potentially leading to overflow!
            .then(|| self.dest_start + (num - self.src_start))
    }
}

/// The naive brute-force solution to part 1.
///
/// Each seed (in order) is passed through the map pipeline one by one until its
/// location number is found, filling a list of location numbers. The smallest
/// number in the list is then used as the answer.
///
/// As you can tell, the more seeds there are, the longer the calculation will
/// take (each seed needs to go through all range maps). This becomes infeasible
/// for part 2 where the number of seeds to consider is so much larger.
pub mod part1 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let (input, seeds) =
            terminated(parse_seeds, multispace1)(input).unwrap();

        let (_, min_location) = parse_min_location(input, &seeds).unwrap();

        Ok(min_location.to_string())
    }

    /// Parses the list of seeds.
    ///
    /// `"seeds: 79 14 55 13"` -> `[79, 14, 55, 13]`
    fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
        preceded(
            preceded(take_until1(": "), tag(": ")),
            separated_list1(space1, u64),
        )(input)
    }

    /// Parses the lowest location number for the given list of seeds.
    fn parse_min_location<'a>(
        input: &'a str,
        seeds: &[u64],
    ) -> IResult<&'a str, u64> {
        let (input, map_sections) =
            separated_list1(tag("\n\n"), parse_map)(input)?;

        // Gather the location numbers of all seeds, then find the smallest one
        let min_location_num = seeds
            .iter()
            .map(|seed| {
                let mut num = *seed;

                // For each seed, we pass it through the map pipeline one by one
                // till the last one to obtain the location number.
                for map_section in &map_sections {
                    num = *map_section
                        .iter()
                        .filter_map(|range_map| range_map.map(num))
                        .collect::<Vec<_>>()
                        // If `num` is in some mapped range, use the map.
                        // Otherwise, source num == destination num.
                        .first()
                        .unwrap_or(&num);
                }

                num
            })
            .min()
            .unwrap();

        Ok((input, min_location_num))
    }

    /// Parses each map section into a list of range maps.
    ///
    /// Example:
    /// ```text
    /// seed-to-soil map:
    /// 50 98 2
    /// 52 50 48
    /// ```
    /// becomes `[RangeMap {98, 50, 2}, RangeMap {50, 52, 48}]`.
    fn parse_map(input: &str) -> IResult<&str, Vec<RangeMap>> {
        // Ignore the first line of the section, e.g. "seed-to-soil map:"
        let (input, _) =
            preceded(take_till1(|c| c == '\n'), line_ending)(input)?;

        separated_list1(
            line_ending,
            map(separated_list1(space1, u64), |nums| RangeMap {
                src_start: nums[1],
                dest_start: nums[0],
                range_len: nums[2],
            }),
        )(input)
    }
}

/// A smarter implementation for part 2 compared to part 1's naive solution.
///
/// The essence is that we flip the direction of the map pipeline: instead of
/// processing each seed top (`seed`) to bottom (`location`), we process potential
/// location numbers in ascending order up the pipeline (`location` -> `seed`).
/// We're trying to find the smallest location number anyways, so the first one
/// that falls in a seed range will automatically be the answer.
///
/// This is still kinda brute-forcing, but instead of the cost scaling with the
/// number of seeds (which is... gigantic for part 2), apparently the search
/// space is smaller. Credit goes to some of the comments at the AOC subreddit
/// for the idea :)
pub mod part2 {
    use super::*;

    type Range = std::ops::Range<u64>;

    pub fn run(input: &str) -> Result<String> {
        let (input, seed_ranges) =
            terminated(parse_seed_ranges, multispace1)(input).unwrap();

        let (_, min_location) =
            parse_min_location(input, &seed_ranges).unwrap();

        Ok(min_location.to_string())
    }

    /// Parses the list of seed ranges.
    ///
    /// `"seeds: 79 14 55 13"` -> `[79..93, 55..68]`
    fn parse_seed_ranges(input: &str) -> IResult<&str, Vec<Range>> {
        preceded(
            preceded(take_until1(": "), tag(": ")),
            separated_list1(space1, separated_pair(u64, space1, u64)),
        )(input)
        .map(|(input, seeds)| {
            (
                input,
                seeds
                    .iter()
                    .map(|&(start, len)| Range {
                        start,
                        end: start + len,
                    })
                    .collect(),
            )
        })
    }

    /// Parses the lowest location number for the given list of seed ranges.
    fn parse_min_location<'a>(
        input: &'a str,
        seed_ranges: &'a [Range],
    ) -> IResult<&'a str, u64> {
        let (input, mut map_sections) =
            separated_list1(tag("\n\n"), parse_map_reversed)(input)?;

        // We go through the pipeline backwards/upwards: from `location` back up
        // to the `seed` ranges.
        map_sections.reverse();

        // We now iterate through all possible location numbers (ascending order),
        // and the first one that falls in a seed range is the answer.
        let min_location = (0..=u64::MAX)
            .find(|location| {
                let mut seed_equivalent = *location;

                // For each location number, we pass it through the map pipeline
                // upwards till the last one (the seed one).
                for map_section in &map_sections {
                    seed_equivalent = *map_section
                        .iter()
                        .filter_map(|range| range.map(seed_equivalent))
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap_or(&seed_equivalent);
                }

                // If this location's "seed equivalent" number falls in any seed
                // range, it is the answer.
                seed_ranges
                    .iter()
                    .any(|seed_range| seed_range.contains(&seed_equivalent))
            })
            .unwrap();

        Ok((input, min_location))
    }

    /// Parses each map section into a list of range maps.
    ///
    /// Different to part 1, we're considering the first number to be the source
    /// instead, and the second number the destination start. This is because
    /// we're going to be mapping the sections backwards/upwards
    /// (`location` -> `seed`).
    ///
    /// Example:
    /// ```text
    /// seed-to-soil map:
    /// 50 98 2
    /// 52 50 48
    /// ```
    /// becomes `[RangeMap {50, 98, 2}, RangeMap {52, 50, 48}]`.
    fn parse_map_reversed(input: &str) -> IResult<&str, Vec<RangeMap>> {
        // Ignore the first line of the map section, e.g. "seed-to-soil map:"
        let (input, _) =
            preceded(take_till1(|c| c == '\n'), line_ending)(input)?;

        separated_list1(
            line_ending,
            map(separated_list1(space1, u64), |nums| RangeMap {
                // First number is source; second destination
                src_start: nums[0],
                dest_start: nums[1],
                range_len: nums[2],
            }),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ok() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!("35", part1::run(input).unwrap());
    }

    #[test]
    fn part2_ok() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!("46", part2::run(input).unwrap());
    }
}
