use crate::input_file;
use anyhow::Result;
use std::io::{prelude::*, BufReader};

pub fn eval_part_1(file: &str) -> Result<String> {
	let input = input_file(file)?;
	let input = BufReader::new(input);
	let mut lines = input.lines();
	let mut stacks = build_stacks(&mut lines)?;
	let line = lines.next().expect("instructions separator")?;
	debug_assert!(line.is_empty());
	for line in lines {
		let Instruction {
			n,
			from_col,
			to_col,
		} = parse_instruction(&line?)?;
		let (from, to) = stacks.get_2_mut(from_col, to_col);
		for _ in 0..n {
			to.push(from.pop().expect("stack shouldn't be empty"));
		}
	}
	unsafe {
		Ok(String::from_utf8_unchecked(
			stacks
				.iter()
				.map(|stack| *stack.last().unwrap_or(&b' '))
				.collect::<Vec<u8>>(),
		))
	}
}

#[cfg(test)]
#[test]
fn part_1() {
	let result = eval_part_1("day_5.example").unwrap();
	assert_eq!(result, "CMZ")
}

pub fn eval_part_2(file: &str) -> Result<String> {
	let input = input_file(file)?;
	let input = BufReader::new(input);
	let mut lines = input.lines();
	let mut stacks = build_stacks(&mut lines)?;
	let line = lines.next().expect("instructions separator")?;
	debug_assert!(line.is_empty());
	for line in lines {
		let Instruction {
			n,
			from_col,
			to_col,
		} = parse_instruction(&line?)?;
		let start = stacks[from_col].len() - n;
		let (from, to) = stacks.get_2_mut(from_col, to_col);
		to.extend(from.drain(start..))
	}
	unsafe {
		Ok(String::from_utf8_unchecked(
			stacks
				.iter()
				.map(|stack| *stack.last().unwrap_or(&b' '))
				.collect::<Vec<u8>>(),
		))
	}
}

#[cfg(test)]
#[test]
fn part_2() {
	let result = eval_part_2("day_5.example").unwrap();
	assert_eq!(result, "MCD")
}

trait Get2Mut {
	type Item;
	fn get_2_mut(&mut self, a: usize, b: usize) -> (&mut Self::Item, &mut Self::Item);
}

impl<T> Get2Mut for Vec<T> {
	type Item = T;
	fn get_2_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T) {
		assert_ne!(a, b);
		assert!(a < self.len());
		assert!(b < self.len());
		let this = self.as_mut_ptr();
		unsafe { (&mut *this.add(a), &mut *this.add(b)) }
	}
}

struct Instruction {
	n: usize,
	from_col: usize,
	to_col: usize,
}

fn build_stacks(lines: &mut impl Iterator<Item = std::io::Result<String>>) -> Result<Vec<Vec<u8>>> {
	let mut line = lines.next().expect("input")?;
	let mut stacks = std::iter::repeat(Vec::new())
		.take((line.len() / 4) + 1)
		.collect::<Vec<Vec<u8>>>();
	loop {
		let bytes = line.as_bytes();
		if bytes[1] == b'1' {
			break;
		}
		for (col, i) in (1..line.len()).step_by(4).enumerate() {
			let val = bytes[i];
			if val != b' ' {
				stacks[col].insert(0, val)
			}
		}
		line = lines.next().expect("column labels")?;
	}
	Ok(stacks)
}

fn parse_instruction(line: &str) -> Result<Instruction> {
	let line = line.strip_prefix("move ").expect("move command");
	let (n, line) = line.split_once(' ').expect("number to move");
	let n = n.parse::<usize>()?;
	let line = line.strip_prefix("from ").expect("from indicator");
	let (from_col, line) = line.split_once(' ').expect("from column");
	let from_col = from_col.parse::<usize>()? - 1;
	let line = line.strip_prefix("to ").expect("to indicator");
	let to_col = line.parse::<usize>()? - 1;
	Ok(Instruction {
		n,
		from_col,
		to_col,
	})
}
