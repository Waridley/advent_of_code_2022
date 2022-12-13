use crate::{input_file, Lines};
use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

pub fn eval_part_1(file: &str) -> Result<usize> {
	let lines = Lines::new(input_file(file)?);
	let mut rope = Rope::<2>::new();
	for line in lines.lines() {
		let line = line?;
		rope.command(line)?;
	}
	Ok(rope.tail_visited.len())
}

pub fn eval_part_2(file: &str) -> Result<usize> {
	let lines = Lines::new(input_file(file)?);
	let mut rope = Rope::<10>::new();
	for line in lines.lines() {
		let line = line?;
		rope.command(line)?;
	}
	Ok(rope.tail_visited.len())
}

struct Rope<const KNOTS: usize> {
	knots: [(i32, i32); KNOTS],
	tail_visited: HashSet<(i32, i32)>,
}

impl<const KNOTS: usize> Rope<KNOTS> {
	fn new() -> Self {
		Self {
			knots: [(0, 0); KNOTS],
			tail_visited: HashSet::default(),
		}
	}

	fn head_mut(&mut self) -> &mut (i32, i32) {
		&mut self.knots[0]
	}

	fn tail_mut(&mut self) -> &mut (i32, i32) {
		self.knots.last_mut().unwrap()
	}

	fn command(&mut self, cmd: impl AsRef<str>) -> Result<()> {
		let cmd = cmd.as_ref();
		let (dir, dist) = cmd
			.split_once(' ')
			.unwrap_or_else(|| panic!("invalid command: {cmd}"));
		let dist = dist.parse::<u32>()?;
		match dir {
			"L" => (0..dist).for_each(|_| {
				self.head_mut().0 -= 1;
				self.pull_tails();
			}),
			"R" => (0..dist).for_each(|_| {
				self.head_mut().0 += 1;
				self.pull_tails();
			}),
			"U" => (0..dist).for_each(|_| {
				self.head_mut().1 -= 1;
				self.pull_tails();
			}),
			"D" => (0..dist).for_each(|_| {
				self.head_mut().1 += 1;
				self.pull_tails();
			}),
			cmd => panic!("invalid command: {cmd}"),
		}
		Ok(())
	}

	fn pull_tails(&mut self) {
		for i in 1..KNOTS {
			self.pull_tail(i);
		}
		let pos = *self.tail_mut();
		self.tail_visited.insert(pos);
	}

	fn pull_tail(&mut self, i: usize) {
		let [head, tail] = &mut self.knots[(i - 1)..=i] else { unreachable!("{i}")};

		let y_dist = head.1 - tail.1;
		let x_dist = head.0 - tail.0;

		match (x_dist.abs(), y_dist.abs()) {
			(x, y) if x > 2 || y > 2 => {
				panic!("tail got too far away")
			}
			(x, y) if x < 2 && y < 2 => {}
			(2, y) => {
				tail.0 += x_dist.signum();
				tail.1 += i32::min(y, 1) * y_dist.signum()
			}
			(x, 2) => {
				tail.1 += y_dist.signum();
				tail.0 += i32::min(x, 1) * x_dist.signum()
			}
			pair => unreachable!("{:?}", pair),
		}
	}
}
