use anyhow::Result;
use num_bigint::BigUint;
use waridley_aoc_2022::day_11::eval_part_2;

fn main() -> Result<()> {
	let result = eval_part_2("day_11")?;
	println!("{result}");
	Ok(())
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let result = eval_part_2("day_11.example")?;
	assert_eq!(result, BigUint::from(2713310158u128));
	Ok(())
}
