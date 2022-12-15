use crate::day_14::Path::{Floor, WallPath};
use crate::day_14::SandDropResult::{Blocked, CameToRest, FellIntoAbyss};
use crate::day_14::Space::{Empty, Sand, Wall};
use crate::{input_lines, IntoContiguous};
use anyhow::Result;
use std::fmt::{Debug, Formatter, Write};
use std::iter::repeat;
use std::{cmp::Ordering::*, io::BufRead};

pub fn eval_part_1(file: &str) -> Result<usize> {
	let lines = input_lines(file)?.lines();
	let paths = lines
		.map(|line| {
			line.unwrap()
				.split(" -> ")
				.map(|pair| pair.split_once(',').unwrap())
				.map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
				.collect::<Vec<_>>()
		})
		.map(|path| WallPath(path.into_iter()));
	let mut cave = Cave::from(paths);
	println!("{cave:?}");

	let mut grains_dropped = 0;
	while let CameToRest = cave.drop_grain_of_sand() {
		grains_dropped += 1
	}

	println!("{cave:?}");
	Ok(grains_dropped)
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let result = eval_part_1("day_14.example")?;
	assert_eq!(result, 24);
	Ok(())
}

pub fn eval_part_2(file: &str) -> Result<usize> {
	let lines = input_lines(file)?.lines();
	let paths = lines
		.map(|line| {
			line.unwrap()
				.split(" -> ")
				.map(|pair| pair.split_once(',').unwrap())
				.map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
				.collect::<Vec<_>>()
		})
		.map(|path| WallPath(path.into_iter()))
		.chain(std::iter::once(Floor));
	let mut cave = Cave::from(paths);
	println!("{cave:?}");

	let mut grains_dropped = 0;
	let mut result = CameToRest;
	while let CameToRest = result {
		result = cave.drop_grain_of_sand();
		grains_dropped += 1
	}
	assert!(matches!(result, Blocked));
	println!("{cave:?}");
	Ok(grains_dropped)
}

#[cfg(test)]
#[test]
fn part_2() -> Result<()> {
	let result = eval_part_2("day_14.example")?;
	assert_eq!(result, 93);
	Ok(())
}

struct Cave {
	spaces: Box<[Space]>,
	width: usize,
}

impl Cave {
	fn num_rows(&self) -> usize {
		self.spaces.len() / self.width
	}

	fn rows(&self) -> impl Iterator<Item = &[Space]> {
		self.spaces.chunks(self.width)
	}

	fn row_mut(&mut self, row: usize) -> Option<&mut [Space]> {
		let start = row * self.width;
		(row < self.num_rows()).then(|| &mut self.spaces[start..(start + self.width)])
	}

	fn drop_grain_of_sand(&mut self) -> SandDropResult {
		let (mut x, mut y) = (500, 0);
		loop {
			let Some(next_row) = self.row_mut(y + 1) else {
				return FellIntoAbyss
			};
			x = if next_row[x] == Empty {
				x
			} else if x == 0 {
				return FellIntoAbyss;
			} else if next_row[x - 1] == Empty {
				x - 1
			} else if x + 1 == next_row.len() {
				return FellIntoAbyss;
			} else if next_row[x + 1] == Empty {
				x + 1
			} else {
				self.row_mut(y).unwrap()[x] = Sand;
				if y == 0 {
					return Blocked;
				} else {
					return CameToRest;
				}
			};
			y += 1;
		}
	}
}

impl<Paths: Iterator<Item = Path<Iter>>, Iter: Iterator<Item = (usize, usize)>> From<Paths>
	for Cave
{
	fn from(paths: Paths) -> Self {
		let mut width = 512;
		let mut spaces = vec![vec![Empty; width]]; // will likely grow, but we know sand comes from 500

		let mut ensure_cap = |spaces: &mut Vec<Vec<Space>>, x, y| {
			// ensure there is space to fall to the floor on the right
			if x + 1 >= width {
				for row in spaces.iter_mut() {
					row.extend(repeat(Empty).take((x + 2) - width));
					assert_eq!(row.len(), x + 2)
				}
				width = x + 2;
			}
			let len = spaces.len();
			if y >= len {
				spaces.extend(std::iter::repeat_with(|| vec![Empty; width]).take((y + 1) - len));
				assert_eq!(spaces.len(), y + 1)
			}
		};

		for path in paths {
			let mut path = match path {
				WallPath(path) => path,
				Floor => {
					let new_width = spaces.first().unwrap().len() * 2; // Ensure enough room for pile on floor
					let new_height = spaces.len() + 1; // append 2 more lines
					(ensure_cap)(&mut spaces, new_width, new_height);
					spaces.last_mut().unwrap().fill(Wall);
					break;
				}
			};
			let Some(mut prev) = path.next() else { continue };
			(ensure_cap)(&mut spaces, prev.0, prev.1);
			for (x, y) in path {
				(ensure_cap)(&mut spaces, x, y);

				match (x.cmp(&prev.0), y.cmp(&prev.1)) {
					(Greater, Equal) => (prev.0..=x)
						.zip(repeat(y))
						.for_each(|(x, y)| spaces[y][x] = Wall),
					(Less, Equal) => (x..=prev.0)
						.zip(repeat(y))
						.for_each(|(x, y)| spaces[y][x] = Wall),
					(Equal, Greater) => repeat(x)
						.zip(prev.1..=y)
						.for_each(|(x, y)| spaces[y][x] = Wall),
					(Equal, Less) => repeat(x)
						.zip(y..=prev.1)
						.for_each(|(x, y)| spaces[y][x] = Wall),
					dir => panic!("unexpected direction: {dir:?}"),
				};

				prev = (x, y);
			}
		}
		Self {
			spaces: spaces.into_contiguous(width).into_boxed_slice(),
			width,
		}
	}
}

impl Debug for Cave {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for row in self.rows() {
			for space in row {
				match space {
					Empty => f.write_char(' ')?,
					Wall => f.write_char('#')?,
					Sand => f.write_char('.')?,
				}
			}
			f.write_char('\n')?
		}
		Ok(())
	}
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Space {
	#[default]
	Empty,
	Wall,
	Sand,
}

enum SandDropResult {
	CameToRest,
	FellIntoAbyss,
	Blocked,
}

enum Path<Iter: Iterator<Item = (usize, usize)> = std::iter::Empty<(usize, usize)>> {
	WallPath(Iter),
	Floor,
}
