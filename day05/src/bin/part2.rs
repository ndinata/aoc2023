use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day05::part2::run(input)?;
    println!("Day 5 part 2: {}", result);

    Ok(())
}
