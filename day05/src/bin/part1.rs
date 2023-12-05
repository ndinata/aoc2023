use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day05::part1::run(input)?;
    println!("Day 5 part 1: {}", result);

    Ok(())
}
