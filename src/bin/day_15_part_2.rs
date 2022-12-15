use anyhow::Result;
use waridley_aoc_2022::day_15::eval_part_2;

fn main() -> Result<()> {
	let result = eval_part_2("day_15", 4_000_000)?;
	println!("{result}");
	Ok(())
}
