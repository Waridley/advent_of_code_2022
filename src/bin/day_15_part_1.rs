use anyhow::Result;
use waridley_aoc_2022::day_15::eval_part_1;

fn main() -> Result<()> {
	let result = eval_part_1("day_15", 2_000_000)?;
	println!("{result}");
	Ok(())
}
