use anyhow::Result;
use waridley_aoc_2022::day_08::eval_part_1;

fn main() -> Result<()> {
	let result = eval_part_1("day_8")?;
	println!("{result}");
	Ok(())
}
