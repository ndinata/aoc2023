use anyhow::Result;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::{digit1, space1, u64};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        // Naive way — iterating through each possibility one by one, filtering
        // the ones that win.
        // return (0..=self.time)
        //     .filter(|hold_duration| {
        //         (hold_duration * (self.time - hold_duration)) > self.dist
        //     })
        //     .count() as u64;

        let mut counter = 0;

        // Better way is to cut down the search space by half before we begin
        // finding winning numbers, since we can see that the results are
        // symmetric about halfway. Example with time 7:
        // time    : 0 1  2  3  4  5 6 7
        // distance: 0 6 10 12 12 10 7 0
        //                   |  |
        //                 mid-point
        let mid = self.time / 2;

        // This loop short-circuits as soon as a "losing" time is found because
        // we iterate from the middle down to 0 ("winning" numbers at the front
        // of the queue).
        for time in (0..=mid).rev() {
            if (time * (self.time - time)) > self.dist {
                counter += 2;
            } else {
                break;
            }
        }

        // If the time of the race is even, the middle (`time / 2`) stands on
        // its own when split by half, so we minus 1 (since we `counter += 2`
        // each time previously).
        if self.time % 2 == 0 {
            counter - 1
        } else {
            counter
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let (_, races) = parse_races(input).unwrap();

        Ok(races
            .iter()
            .map(|race| race.ways_to_win())
            .product::<u64>()
            .to_string())
    }

    /// Parses list of races.
    pub(super) fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
        // Parse list of race times
        let (input, times) = preceded(
            preceded(preceded(take_until1(":"), tag(":")), space1),
            separated_list1(space1, u64),
        )(input)?;

        // Parse list of race distances
        let (input, distances) = preceded(
            preceded(preceded(take_until1(":"), tag(":")), space1),
            separated_list1(space1, u64),
        )(input)?;

        Ok((
            input,
            times
                .into_iter()
                .zip(distances)
                .map(|(time, dist)| Race { time, dist })
                .collect(),
        ))
    }
}

pub mod part2 {
    use super::*;

    pub fn run(input: &str) -> Result<String> {
        let (_, race) = parse_race(input).unwrap();

        Ok(race.ways_to_win().to_string())
    }

    /// Parses the single race.
    pub(super) fn parse_race(input: &str) -> IResult<&str, Race> {
        // Parse the merged race time
        let (input, time) = preceded(
            preceded(preceded(take_until1(":"), tag(":")), space1),
            separated_list1(space1, digit1),
        )(input)
        .map(|(input, times)| (input, times.concat().parse().unwrap()))?;

        // Parse the merged race distance
        let (input, dist) = preceded(
            preceded(preceded(take_until1(":"), tag(":")), space1),
            separated_list1(space1, digit1),
        )(input)
        .map(|(input, dists)| (input, dists.concat().parse().unwrap()))?;

        Ok((input, Race { time, dist }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ok() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!("288", part1::run(input).unwrap());
    }

    #[test]
    fn part1_parse_races_ok() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let races = Vec::from([
            Race { time: 7, dist: 9 },
            Race { time: 15, dist: 40 },
            Race {
                time: 30,
                dist: 200,
            },
        ]);
        assert_eq!(races, part1::parse_races(input).unwrap().1);
    }

    #[test]
    fn part2_ok() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!("71503", part2::run(input).unwrap());
    }
}
