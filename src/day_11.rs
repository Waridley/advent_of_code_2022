use crate::input_lines;
use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::io::prelude::*;

pub fn eval_part_1(file: &str) -> Result<Worry> {
	let mut lines = input_lines(file)?;
	let mut monkeys = Monkeys::deserialize(&mut lines)?;
	Ok(monkeys.monkey_business(20, |worry| worry / 3))
}

pub fn eval_part_2(file: &str) -> Result<Worry> {
	let mut lines = input_lines(file)?;
	let mut monkeys = Monkeys::deserialize(&mut lines)?;
	Ok(monkeys.monkey_business(10_000, |worry| worry))
}

type MonkeyId = usize;
pub type Worry = u64;

#[derive(Debug)]
struct Monkeys(&'static mut [Monkey]);

impl Monkeys {
	fn deserialize(stream: &mut impl BufRead) -> Result<Self> {
		let mut lines = stream.lines();
		let mut monkeys = Vec::new();
		loop {
			let Some(line) = lines.next() else { break };
			let line = line?;
			let line = line
				.strip_prefix("Monkey ")
				.unwrap_or_else(|| panic!("Expected new Monkey, got: {line}"));
			let id = line
				.strip_suffix(':')
				.unwrap_or_else(|| panic!("Unexpected end of line"))
				.parse::<MonkeyId>()?;
			assert_eq!(id, monkeys.len());

			let line = lines.next().expect("starting items")?;
			let line = line
				.strip_prefix("  Starting items: ")
				.unwrap_or_else(|| panic!("Expected starting items, got: {line}"));
			let mut holding = vec![];
			for item in line.split(", ") {
				holding.push(item.parse()?);
			}

			let line = lines.next().expect("operation")?;
			let line = line
				.strip_prefix("  Operation: new = old ")
				.unwrap_or_else(|| panic!("Expected operation, got: {line}"));
			let (op, arg) = line.split_once(' ').expect("operation arg");
			let operation = match (op, arg) {
				("+", "old") => Op::Double,
				("*", "old") => Op::Square,
				("+", n) => Op::Add(n.parse()?),
				("*", n) => Op::Mul(n.parse()?),
				_ => panic!("Unknown operation: {line}"),
			};

			let line = lines.next().expect("operation")?;
			let line = line
				.strip_prefix("  Test: divisible by ")
				.unwrap_or_else(|| panic!("Expected test condition, got: {line}"));
			let test = line.parse()?;

			let line = lines.next().expect("operation")?;
			let line = line
				.strip_prefix("    If true: throw to monkey ")
				.unwrap_or_else(|| panic!("Expected 'If true', got: {line}"));
			let if_true = line.parse()?;

			let line = lines.next().expect("operation")?;
			let line = line
				.strip_prefix("    If false: throw to monkey ")
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

	fn round(&mut self, manage_worry: &impl Fn(Worry) -> Worry) {
		let mut tmp = vec![];
		for i in 0..self.0.len() {
			tmp.clear();
			{
				let Monkey {
					ref mut holding,
					ref operation,
					ref mut num_inspected,
					..
				} = self.0[i];
				*num_inspected += holding.len() as u64;
				tmp.extend(
					holding
						.drain(..)
						.map(|item| match operation {
							Op::Add(n) => item + n,
							Op::Double => item + item,
							Op::Mul(n) => item * n,
							Op::Square => item * item,
						})
						.map(manage_worry),
				);
			}
			for item in tmp.drain(..) {
				let Monkey {
					ref test,
					if_true,
					if_false,
					..
				} = self.0[i];
				let to = if item % test == 0 { if_true } else { if_false };
				let to = &mut self.0[to];
				to.holding.push(item);
			}
		}
	}

	fn monkey_business(&mut self, rounds: usize, manage_worry: impl Fn(Worry) -> Worry) -> Worry {
		let common_factor = self
			.0
			.iter()
			.map(|monkey| monkey.test)
			.fold(3, |acc, test| acc * test);
		let manage_worry = move |worry| (manage_worry)(worry) % common_factor;
		for _ in 0..rounds {
			self.round(&manage_worry);
		}
		let (mut most, mut second_most) = (0, 0);
		for monkey in self.0.iter() {
			let n = monkey.num_inspected;
			if n > most {
				second_most = most;
				most = n;
			} else if n > second_most {
				second_most = n;
			}
		}
		most * second_most
	}
}

struct Monkey {
	holding: Vec<Worry>,
	operation: Op,
	test: Worry,
	if_true: MonkeyId,
	if_false: MonkeyId,
	num_inspected: Worry,
}

enum Op {
	Add(Worry),
	Double,
	Mul(Worry),
	Square,
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
