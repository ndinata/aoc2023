use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day07::part2::run(input)?;
    println!("Day 7 part 2: {}", result);

    Ok(())
}
