use anyhow::Result;
use waridley_aoc_2022::day_13::eval_part_2;

fn main() -> Result<()> {
	let result = eval_part_2("day_13")?;
	println!("{result}");
	Ok(())
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let result = eval_part_2("day_13.example")?;
	assert_eq!(result, 140);
	Ok(())
}
