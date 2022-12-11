use std::fmt::{Debug, Formatter};
use std::io::prelude::*;
use std::ops::{Add, Mul};
use anyhow::Result;
use crate::{input_lines};

pub fn eval_part_1(file: &str) -> Result<usize> {
	let mut lines = input_lines(file)?;
	let mut monkeys = Monkeys::deserialize(&mut lines)?;
	for _ in 0..20 {
		monkeys.round();
	}
	let (mut most, mut second_most) = (0, 0);
	for monkey in monkeys.0.iter() {
		let n = monkey.num_inspected;
		if n > most {
			second_most = most;
			most = n;
		} else if n > second_most {
			second_most = n;
		}
	}
	Ok(most * second_most)
}

type MonkeyId = usize;
type Worry = usize;

#[derive(Debug)]
struct Monkeys(&'static mut [Monkey]);

impl Monkeys {
	fn deserialize(stream: &mut impl BufRead) -> Result<Self> {
		let mut lines = stream.lines();
		let mut monkeys = Vec::new();
		loop {
			let Some(line) = lines.next() else { break };
			let line = line?;
			let line = line.strip_prefix("Monkey ")
				.unwrap_or_else(|| panic!("Expected new Monkey, got: {line}"));
			let id = line.strip_suffix(':')
				.unwrap_or_else(|| panic!("Unexpected end of line"))
				.parse::<MonkeyId>()?;
			assert_eq!(id, monkeys.len());
			
			let line = lines.next().expect("starting items")?;
			let line = line.strip_prefix("  Starting items: ")
				.unwrap_or_else(|| panic!("Expected starting items, got: {line}"));
			let mut holding = vec![];
			for item in line.split(", ") {
				holding.push(item.parse()?);
			}
			
			let line = lines.next().expect("operation")?;
			let line = line.strip_prefix("  Operation: new = old ")
				.unwrap_or_else(|| panic!("Expected operation, got: {line}"));
			let (op, arg) = line.split_once(' ').expect("operation arg");
			let op = match op {
				"*" => Worry::mul,
				"+" => Worry::add,
				other => panic!("Unknown operation: {other}")
			};
			let operation = match arg {
				"old" => Box::new(move |old| op(old, old)) as _,
				n => {
					let n = n.parse()?;
					Box::new(move |old| op(old, n)) as _
				},
			};
			
			let line = lines.next().expect("operation")?;
			let line = line.strip_prefix("  Test: divisible by ")
				.unwrap_or_else(|| panic!("Expected test condition, got: {line}"));
			let test = line.parse()?;
			
			let line = lines.next().expect("operation")?;
			let line = line.strip_prefix("    If true: throw to monkey ")
				.unwrap_or_else(|| panic!("Expected 'If true', got: {line}"));
			let if_true = line.parse()?;
			
			let line = lines.next().expect("operation")?;
			let line = line.strip_prefix("    If false: throw to monkey ")
				.unwrap_or_else(|| panic!("Expected 'If false', got: {line}"));
			let if_false = line.parse()?;
			
			monkeys.push(Monkey {
				holding,
				operation,
				test,
				if_true,
				if_false,
				num_inspected: 0,
			});
			
			let Some(line) = lines.next() else { break };
			assert!(line?.is_empty())
		}
		Ok(Self(Vec::leak(monkeys)))
	}
	
	fn round(&mut self) {
		let mut tmp = vec![];
		for i in 0..self.0.len() {
			tmp.clear();
			let Monkey {
				ref mut holding,
				ref operation,
				test,
				if_true,
				if_false,
				..
			} = self.0[i];
			tmp.extend(holding.drain(..).map(|item| (operation)(item) / 3));
			for item in tmp.drain(..) {
				self.0[i].num_inspected += 1;
				self.0[if item % test == 0 {
					if_true
				} else {
					if_false
				}].holding.push(item);
			}
		}
	}
}

struct Monkey {
	holding: Vec<Worry>,
	operation: Box<dyn  Fn(Worry) -> Worry>,
	test: usize,
	if_true: MonkeyId,
	if_false: MonkeyId,
	num_inspected: usize,
}

impl Debug for Monkey {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Monkey")
			.field("holding", &self.holding)
			.field("test", &format!("divisible by {}", self.test))
			.field("if_true", &format!("throw to monkey {}", self.test))
			.field("if_false", &format!("throw to monkey {}", self.test))
			.field("num_inspected", &self.num_inspected)
			.finish()
	}
}