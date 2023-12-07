use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day08::part2::run(input)?;
    println!("Day 8 part 2: {}", result);

    Ok(())
}
