use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day03::part1::run(input)?;
    println!("Day 3 part 1: {}", result);

    Ok(())
}
