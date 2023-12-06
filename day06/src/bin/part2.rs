use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");

    let result = day06::part2::run(input)?;
    println!("Day 6 part 2: {}", result);

    Ok(())
}
