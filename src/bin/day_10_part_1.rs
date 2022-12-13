use anyhow::Result;
use waridley_aoc_2022::day_10::eval_part_1;

fn main() -> Result<()> {
	let result = eval_part_1("day_10")?;
	println!("{result}");
	Ok(())
}
