use anyhow::Result;
use waridley_aoc_2022::day_09::eval_part_2;

fn main() -> Result<()> {
	let result = eval_part_2("day_9")?;
	println!("{result}");
	Ok(())
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let result = eval_part_2("day_9.example_2")?;
	assert_eq!(result, 36);
	Ok(())
}
