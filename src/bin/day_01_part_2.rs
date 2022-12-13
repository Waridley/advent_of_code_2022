use anyhow::Result;
use waridley_aoc_2022::day_01::eval_part_2;

fn main() -> Result<()> {
	let result = eval_part_2("day_1")?;
	println!("{result}");
	Ok(())
}
