use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day01::part2::run(input)?;
    println!("Day 1 part 2: {}", result);

    Ok(())
}
