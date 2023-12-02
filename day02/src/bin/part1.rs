use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day02::part1::run(input)?;
    println!("Day 2 part 1: {}", result);

    Ok(())
}
