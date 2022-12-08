use std::{
	borrow::Cow,
	io::{BufReader, prelude::*}
};
use std::collections::BTreeMap;
use anyhow::Result;
use crate::input_file;

type Lines = BufReader<std::fs::File>;

pub fn eval_part_1(file: &str) -> Result<usize> {
	let input = input_file(file)?;
	let lines = BufReader::new(input);
	let root = init(lines)?;
	Ok(root.sum_dirs_lt_or_eq_to(100_000))
}

pub fn eval_part_2(file: &str) -> Result<usize> {
	let input = input_file(file)?;
	let lines = BufReader::new(input);
	let root = init(lines)?;
	let avail = 70_000_000 - root.total_size;
	let needed = 30_000_000 - avail;
	Ok(root.best_dir_to_delete(needed).total_size)
}

fn init(mut lines: Lines) -> Result<Dir<'static>> {
	let mut line = String::new();
	lines.read_line(&mut line)?;
	let line = line.strip_prefix("$ ").expect("input should start with a command");
	let line = line.strip_prefix("cd ").expect("first command should be cd");
	assert_eq!(line, "/\n");
	let mut root = Dir::default();
	root.cd(&mut lines)?;
	Ok(root)
}

#[derive(Default, Debug)]
struct Dir<'a> {
	subdirs: BTreeMap<Cow<'a, str>, Dir<'a>>,
	files: BTreeMap<Cow<'a, str>, File>,
	total_size: usize,
}

#[derive(Default, Debug)]
struct File {
	size: usize,
}

impl File {
	fn new(size: usize) -> Self {
		Self {
			size
		}
	}
}

impl Dir<'_> {
	fn cd(&mut self, lines: &mut Lines) -> Result<()> {
		let mut buf = String::new();
		if lines.read_line(&mut buf)? == 0 {
			return Ok(())
		}
		let mut line = Cow::from(buf);
		loop {
			if line.is_empty() {
				return Ok(())
			} else if line == "$ ls\n" {
				line = self.parse_ls_output(lines)?.into();
			} else if let Some(name) = line.strip_prefix("$ cd ") {
				let name = name.strip_suffix('\n').unwrap();
				if name == ".." {
					return Ok(())
				} else {
					let subdir = self.subdirs.get_mut(name).expect(&*format!("Directory {name} not found"));
					subdir.cd(lines)?;
					self.total_size += subdir.total_size;
					let mut buf = String::new();
					if lines.read_line(&mut buf)? == 0 {
						return Ok(())
					}
					line = Cow::from(buf);
				}
			} else {
				panic!("Unexpected input: {line}")
			}
		}
	}
	
	fn parse_ls_output(&mut self, lines: &mut Lines) -> Result<String> {
		loop {
			let mut line = String::new();
			let bytes = lines.read_line(&mut line)?;
			if bytes == 0 {
				return Ok("".into())
			}
			if line.starts_with('$') {
				return Ok(line)
			}
			
			let (dir_or_size, name) = line.split_once(' ').expect(&*format!("Unexpected ls output: {line}"));
			let name = if let Some(name) = name.strip_suffix("\n") { name } else { name };
			if dir_or_size == "dir" {
				let existing = self.subdirs.insert(name.to_owned().into(), Dir::default());
				if let Some(existing) = existing {
					eprintln!("Warning: Directory already existed: {name}: {existing:?}");
				}
			} else {
				let size = dir_or_size.parse::<usize>()?;
				let existing = self.files.insert(name.to_owned().into(), File::new(size));
				if let Some(existing) = existing {
					eprintln!("Warning: File already existed: {name}: {existing:?}");
				}
				self.total_size += size;
			}
		}
	}
	
	fn sum_dirs_lt_or_eq_to(&self, size: usize) -> usize {
		let mut sum = 0;
		if self.total_size <= size {
			sum += self.total_size
		}
		for subdir in self.subdirs.values() {
			sum += subdir.sum_dirs_lt_or_eq_to(size)
		}
		sum
	}
	
	fn best_dir_to_delete(&self, needed: usize) -> &Dir {
		let mut curr = self;
		for dir in self.subdirs.values() {
			let dir = dir.best_dir_to_delete(needed);
			if dir.total_size >= needed && dir.total_size < curr.total_size {
				curr = dir
			}
		}
		curr
	}
}
