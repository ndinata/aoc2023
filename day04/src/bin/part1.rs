use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day04::part1::run(input)?;
    println!("Day 4 part 1: {}", result);

    Ok(())
}
