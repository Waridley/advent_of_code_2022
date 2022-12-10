use std::io::BufRead;
use std::ops::ControlFlow;
use std::ops::ControlFlow::Continue;
use crate::{input_file, Lines};
use crate::day_10::Instruction::*;
use anyhow::Result;
use crate::day_10::ClockState::*;

pub fn eval_part_1(file: &str) -> Result<usize> {
	let lines = Lines::new(input_file(file)?);
	let mut instructions = lines.lines().map(|line| {
		match line.unwrap().split_once(' ') {
			Some(("addx", v)) => Addx(v.parse::<i64>().expect("value to add to X")),
			Some(("noop", "")) => Noop,
			other => panic!("Unknown instruction: {other:?}")
		}
	});
	let mut cpu = Cpu::new();
	let mut sum = 0;
	loop {
		if cpu.rising_edge(&mut instructions).is_break() { break }
		dbg!(&cpu);
		todo!();
		if cpu.falling_edge(&mut instructions).is_break() { break }
	}
	Ok(sum)
}

pub fn eval_part_2(file: &str) -> Result<usize> {
	todo!()
}

#[derive(Debug)]
pub struct Cpu {
	clock: ClockState,
	cycle: usize,
	x: i64,
	
}

impl Cpu {
	fn new() -> Self {
		Self {
			clock: Low,
			cycle: 0,
			x: 1,
		}
	}
	
	fn rising_edge(&mut self, instructions: &mut impl Iterator<Item = Instruction>)  -> ControlFlow<()> {
		assert_eq!(self.clock, Low);
		self.clock = High;
		self.cycle += 1;
		Continue(())
	}
	
	fn falling_edge(&mut self, instructions: &mut impl Iterator<Item = Instruction>) -> ControlFlow<()> {
		assert_eq!(self.clock, High);
		self.clock = Low;
		Continue(())
	}
}

#[derive(Debug)]
enum Instruction {
	Addx(i64),
	Noop,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClockState {
	#[default]
	Low,
	High,
}
