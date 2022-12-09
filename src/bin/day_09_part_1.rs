use anyhow::Result;
use waridley_aoc_2022::day_9::eval_part_1;

fn main() -> Result<()> {
	let result = eval_part_1("day_9")?;
	println!("{result}");
	Ok(())
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let result = eval_part_1("day_9.example")?;
	assert_eq!(result, 13);
	Ok(())
}