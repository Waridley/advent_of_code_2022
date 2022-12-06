use anyhow::Result;
use std::io::{prelude::*, BufReader};
use waridley_aoc_2022::input_file;

fn main() -> Result<()> {
	let result = eval_part_1("day_1")?;
	println!("{result}");
	Ok(())
}

pub fn eval_part_1(file: &str) -> Result<String> {
	let input = input_file(file)?;
	let input = BufReader::new(input);

	let mut elves = vec![(0u64, 0u64)];
	let mut fattest_elf_index = 0usize;

	for line in input.lines() {
		let line = line?;
		if line.is_empty() {
			elves.push((0, 0));
		} else {
			let fattest_elf_calories = elves[fattest_elf_index].1;
			let curr = elves.last_mut().unwrap();
			curr.0 += 1;
			curr.1 += line.parse::<u64>()?;
			if curr.1 > fattest_elf_calories {
				fattest_elf_index = elves.len() - 1;
			}
		}
	}
	Ok(format!("{}", elves[fattest_elf_index].1))
}

#[cfg(test)]
#[test]
fn part_1() {
	let result = eval_part_1("day_1.example").unwrap();
	assert_eq!(result, "24000")
}
