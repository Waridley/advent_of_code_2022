use crate::input_lines;
use anyhow::Result;
use std::io::prelude::*;

pub fn eval_part_1(file: &str) -> Result<String> {
	let input = input_lines(file)?;

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

pub fn eval_part_2(file: &str) -> Result<String> {
	let input = input_lines(file)?;

	let mut elves = vec![0u64];

	for line in input.lines() {
		let line = line?;
		if line.is_empty() {
			elves.push(0);
		} else {
			let curr = elves.last_mut().unwrap();
			*curr += line.parse::<u64>()?;
		}
	}
	elves.sort();
	let top_3_sum = elves
		.iter()
		.rev()
		.copied()
		.take(3)
		.reduce(|acc, calories| acc + calories)
		.unwrap_or(0);
	Ok(format!("{top_3_sum}"))
}

#[cfg(test)]
#[test]
fn part_2() {
	let result = eval_part_2("day_1.example").unwrap();
	assert_eq!(result, "45000")
}
