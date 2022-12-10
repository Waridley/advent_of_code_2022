use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};
use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};
use std::str::FromStr;
use crate::{input_file, Lines};
use crate::day_10::Instruction::*;
use anyhow::Result;
use crate::day_10::ClockState::*;

pub fn eval_part_1(file: &str) -> Result<isize> {
	let lines = Lines::new(input_file(file)?);
	let mut instructions = lines.lines()
		.map(|line| line.unwrap().parse().unwrap());
	let mut cpu = Cpu::new();
	let mut sum = 0;
	loop {
		if cpu.rising_edge(&mut instructions).is_break() { break }
		if (cpu.cycle + 20) % 40 == 0 && cpu.cycle <= 220 {
			let signal = cpu.cycle as isize * cpu.registers.x;
			sum += signal;
		}
		if cpu.falling_edge().is_break() { break }
	}
	Ok(sum)
}

pub fn eval_part_2(file: &str, screen: &mut impl Write) -> Result<()> {
	let lines = Lines::new(input_file(file)?);
	let mut instructions = lines.lines()
		.map(|line| line.unwrap().parse().unwrap());
	let mut cpu = Cpu::new();
	let mut crt = Crt::new();
	loop {
		if cpu.rising_edge(&mut instructions).is_break() { break }
		crt.update(cpu.registers.x);
		if cpu.falling_edge().is_break() { break }
	}
	crt.render(screen)
}

#[derive(Debug)]
pub struct Cpu {
	clock: ClockState,
	cycle: usize,
	curr_op: Option<Op>,
	registers: Registers,
}

impl Cpu {
	fn new() -> Self {
		Self {
			clock: Low,
			cycle: 0,
			curr_op: None,
			registers: Registers::new(),
		}
	}
	
	fn rising_edge(&mut self, instructions: &mut impl Iterator<Item = Instruction>)  -> ControlFlow<()> {
		assert_eq!(self.clock, Low);
		self.clock = High;
		self.cycle += 1;
		if self.curr_op.is_none() {
			let Some(instruction) = instructions.next() else { return Break(()) };
			let cycles_remaining = instruction.cycles();
			self.curr_op = Some(Op {
				instruction,
				cycles_remaining,
			});
		}
		Continue(())
	}
	
	fn falling_edge(&mut self) -> ControlFlow<()> {
		assert_eq!(self.clock, High);
		self.clock = Low;
		if let Some(mut op) = self.curr_op.take() {
			op.cycles_remaining -= 1;
			if op.cycles_remaining == 0 {
				op.instruction.eval(&mut self.registers)
			} else {
				self.curr_op = Some(op);
			}
		}
		Continue(())
	}
}

#[derive(Debug)]
enum Instruction {
	Addx(isize),
	Noop,
}

impl Instruction {
	fn cycles(&self) -> usize {
		match self {
			Addx(_) => 2,
			Noop => 1,
		}
	}
	
	fn eval(self, registers: &mut Registers) {
		match self {
			Addx(v) => registers.x += v,
			Noop => {}
		}
	}
}

#[derive(Debug)]
enum InstructionErr {
	UnknownInstruction(String),
	MissingArg,
	ParseError(<isize as FromStr>::Err),
	Empty,
}

impl Display for InstructionErr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

impl FromStr for Instruction {
	type Err = InstructionErr;
	
	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		use InstructionErr::*;
		let mut split = s.split(' ');
		match split.next() {
			Some("addx") => {
				let Some(v) = split.next() else { return Err(MissingArg) };
				match v.parse() {
					Ok(v) => Ok(Addx(v)),
					Err(e) => Err(ParseError(e)),
				}
			},
			Some("noop") => {
				Ok(Noop)
			},
			Some(other) => Err(UnknownInstruction(other.into())),
			None => Err(Empty),
		}
	}
}

#[derive(Debug)]
struct Op {
	instruction: Instruction,
	cycles_remaining: usize,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
enum ClockState {
	#[default]
	Low,
	High,
}

#[derive(Debug)]
struct Registers {
	x: isize,
}

impl Registers {
	fn new() -> Self {
		Self {
			x: 1,
		}
	}
}

#[derive(Debug)]
pub struct Crt {
	pixels: [[u8; 40]; 6],
	cycle: usize,
}

impl Crt {
	fn new() -> Self {
		Self {
			pixels: [[b'.'; 40]; 6],
			cycle: 0,
		}
	}
	
	fn update(&mut self, x: isize) {
		let v = self.cycle / 40;
		let h = self.cycle % 40;
		self.pixels[v][h] = if ((x - 1)..=(x + 1))
			.contains(&(h as isize))
		{
			b'#'
		} else {
			b'.'
		};
		self.cycle = (self.cycle + 1) % (40 * 6);
	}
	
	fn render(&self, screen: &mut impl Write) -> Result<()> {
		for scanline in &self.pixels {
			assert_eq!(screen.write(scanline)?, 40);
			screen.write(&[b'\n'])?;
		}
		Ok(())
	}
}
