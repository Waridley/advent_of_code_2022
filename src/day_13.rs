use crate::day_13::Value::{Int, List};
use crate::{input_file, input_lines};
use anyhow::Result;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::{Display, Formatter, Write};
use std::io::prelude::*;

pub fn eval_part_1(file: &str) -> Result<usize> {
	let mut input = String::new();
	input_file(file)?.read_to_string(&mut input)?;
	let input = input.split("\n\n");
	let mut sum = 0;
	for (i, (left, right)) in input.map(|pair| pair.split_once('\n').unwrap()).enumerate() {
		let (left, rem) = list(left)?;
		assert!(rem.is_empty());
		let left = List(left);
		let (right, rem) = list(right)?;
		assert!(rem.is_empty());
		let right = List(right);

		if left <= right {
			sum += i + 1;
		}
	}
	Ok(sum)
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let result = eval_part_1("day_13.example")?;
	assert_eq!(result, 13);
	Ok(())
}

pub fn eval_part_2(file: &str) -> Result<usize> {
	let dividers = [
		List(vec![List(vec![Int(2)])]),
		List(vec![List(vec![Int(6)])]),
	];
	let mut packets = dividers
		.iter()
		.cloned()
		.chain(
			input_lines(file)?
				.lines()
				.map(Result::unwrap)
				.filter(|line| !line.is_empty())
				.map(|line| List(list(&line).unwrap().0)),
		)
		.collect::<Vec<_>>();
	packets.sort();
	let div_a = packets
		.iter()
		.enumerate()
		.find(|(_, val)| *val == &dividers[0])
		.unwrap()
		.0 + 1;
	let div_b = packets
		.iter()
		.enumerate()
		.find(|(_, val)| *val == &dividers[1])
		.unwrap()
		.0 + 1;
	Ok(div_a * div_b)
}

#[cfg(test)]
#[test]
fn part_2() -> Result<()> {
	let result = eval_part_2("day_13.example")?;
	assert_eq!(result, 140);
	Ok(())
}

fn list(input: &str) -> Result<(Vec<Value>, &str)> {
	let mut input = input.strip_prefix('[').unwrap();
	let mut vals = vec![];
	loop {
		if input.starts_with(',') {
			input = &input[1..];
		} else if input.starts_with('[') {
			let (list, rest) = list(input)?;
			vals.push(List(list));
			input = rest;
		} else if let Some(i) = input.find(|c| c == ',' || c == ']') {
			if i != 0 {
				vals.push(Int(input[0..i].parse()?));
			}
			input = &input[i..];
			let (found, rest) = input.split_at(1);
			input = rest;
			if found == "]" {
				break;
			}
		} else {
			panic!("lost track of nesting level: {input}")
		}
	}
	Ok((vals, input))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
	List(Vec<Value>),
	Int(u8),
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			List(vals) => {
				f.write_char('[')?;
				let l = vals
					.iter()
					.map(|val| format!("{val}"))
					.collect::<Vec<_>>()
					.join(",");
				f.write_str(&l)?;
				f.write_char(']')?;
			}
			Int(i) => write!(f, "{i}")?,
		}
		Ok(())
	}
}

impl Ord for Value {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Int(l), Int(r)) => l.cmp(r),
			(List(l), List(r)) => {
				let mut left = l.iter();
				let mut right = r.iter();

				loop {
					match (left.next(), right.next()) {
						(Some(l), Some(r)) => match l.cmp(r) {
							Greater => return Greater,
							Less => return Less,
							Equal => continue,
						},
						(Some(_), None) => return Greater,
						(None, Some(_)) => return Less,
						(None, None) => return Equal,
					}
				}
			}
			(l @ List(_), Int(r)) => l.cmp(&List(vec![Int(*r)])),
			(Int(l), r @ List(_)) => List(vec![Int(*l)]).cmp(r),
		}
	}
}

impl PartialOrd for Value {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
