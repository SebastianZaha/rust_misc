use anyhow::Result;

pub mod day6;

fn main() -> Result<()> {
    day6::run_part1()?;
    day6::run_part2()?;

    Ok(())
}
