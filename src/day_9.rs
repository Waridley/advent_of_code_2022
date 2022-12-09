use std::collections::HashSet;
use std::io::BufRead;
use crate::{input_file, Lines};
use anyhow::Result;

pub fn eval_part_1(file: &str) -> Result<usize> {
	let lines = Lines::new(input_file(file)?);
	let mut rope = Rope::default();
	for line in lines.lines() {
		let line = line?;
		rope.command(line)?;
	}
	for pos in rope.tail_visited.iter() {
		println!("{pos:?}")
	}
	Ok(rope.tail_visited.len())
}

pub fn eval_part_2(file: &str) -> Result<usize> {
	todo!()
}

#[derive(Default)]
struct Rope {
	head: (i32, i32),
	tail: (i32, i32),
	tail_visited: HashSet<(i32, i32)>,
}

impl Rope {
	fn command(&mut self, cmd: impl AsRef<str>) -> Result<()> {
		let cmd = cmd.as_ref();
		let (dir, dist) = cmd.split_once(' ').unwrap_or_else(|| panic!("invalid command: {cmd}"));
		let dist = dist.parse::<u32>()?;
		match dir {
			"L" => (0..dist).for_each(|_| {
				self.head.0 -= 1;
				self.pull_tail();
			}),
			"R" => (0..dist).for_each(|_| {
				self.head.0 += 1;
				self.pull_tail();
			}),
			"U" => (0..dist).for_each(|_| {
				self.head.1 -= 1;
				self.pull_tail();
			}),
			"D" => (0..dist).for_each(|_| {
				self.head.1 += 1;
				self.pull_tail();
			}),
			cmd => panic!("invalid command: {cmd}")
		}
		Ok(())
	}

	fn pull_tail(&mut self) {
		let x_dist = self.head.0 - self.tail.0;
		let y_dist = self.head.1 - self.tail.1;

		match (x_dist.abs(), y_dist.abs()) {
			(x, y) if x > 2 || y > 2 => {
				panic!("tail got too far away")
			}
			(x, y) if x < 2 && y < 2 => {}
			(2, y) => {
				self.tail.0 += x_dist.signum();
				self.tail.1 += i32::min(y, 1) * y_dist.signum()
			}
			(x, 2) => {
				self.tail.1 += y_dist.signum();
				self.tail.0 += i32::min(x, 1) * x_dist.signum()
			}
			pair => unreachable!("{:?}", pair),
		}
		self.tail_visited.insert(self.tail);
	}
}
