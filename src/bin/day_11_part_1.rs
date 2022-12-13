use anyhow::Result;
use waridley_aoc_2022::day_11::eval_part_1;

fn main() -> Result<()> {
	let result = eval_part_1("day_11")?;
	println!("{result}");
	Ok(())
}
