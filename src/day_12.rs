use crate::input_lines;
use anyhow::Result;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter, Write};
use std::io::prelude::*;
use std::ops::{Index, IndexMut};

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn eval_part_1(file: &str) -> Result<usize> {
	let map = Map::deserialize(&mut input_lines(file)?)?;
	Ok(map.fewest_steps())
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let result = eval_part_1("day_12.example")?;
	assert_eq!(result, 31);
	Ok(())
}

pub fn eval_part_2(file: &str) -> Result<usize> {
	let map = Map::deserialize(&mut input_lines(file)?)?;
	Ok(map.closest_valley())
}

#[cfg(test)]
#[test]
fn part_2() -> Result<()> {
	let result = eval_part_2("day_12.example")?;
	assert_eq!(result, 29);
	Ok(())
}

struct Map {
	width: usize,
	start: (usize, usize),
	end: (usize, usize),
	heights: &'static [u8],
}

impl Map {
	fn deserialize(stream: &mut impl BufRead) -> Result<Self> {
		let lines = stream.lines();
		let (mut start, mut end) = (None, None);
		let mut width = 0;
		let mut heights = vec![];
		for (y, line) in lines.enumerate() {
			let mut line = line?.into_bytes();
			if width == 0 {
				width = line.len();
			} else {
				assert_eq!(width, line.len())
			}

			for (x, h) in line.iter_mut().enumerate() {
				if *h == b'S' {
					assert!(start.replace((x, y)).is_none());
					*h = b'a'
				} else if *h == b'E' {
					assert!(end.replace((x, y)).is_none());
					*h = b'z'
				}

				*h -= b'a'
			}
			heights.append(&mut line);
		}
		assert_ne!(width, 0);
		let (start, end) = (start.unwrap(), end.unwrap());

		Ok(Self {
			width,
			start,
			end,
			heights: heights.leak(),
		})
	}

	fn rows(&self) -> impl Iterator<Item = &[u8]> {
		self.heights.chunks(self.width)
	}

	fn num_rows(&self) -> usize {
		self.heights.len() / self.width
	}

	fn fewest_steps(&self) -> usize {
		let mut dists = Dists {
			dists: &mut vec![usize::MAX; self.heights.len()],
			width: self.width,
		};
		dists[self.end] = 0;

		let max_x = self.width;
		let max_y = self.num_rows();
		let neighbors = |(x, y): (usize, usize)| {
			NEIGHBORS
				.iter()
				.filter_map(move |&(nx, ny)| x.checked_add_signed(nx).zip(y.checked_add_signed(ny)))
				.filter(move |(nx, ny)| *nx < max_x && *ny < max_y)
		};

		let mut q = neighbors(self.end).collect::<VecDeque<_>>();

		loop {
			let Some((x, y)) = q.pop_front() else { println!("queue is empty"); break };
			let d = dists[(x, y)];
			let h = self[(x, y)];

			for (nx, ny) in neighbors((x, y)) {
				let nd = dists[(nx, ny)];
				let nh = self[(nx, ny)];

				if nd == usize::MAX {
					if !q.contains(&(nx, ny)) {
						q.push_back((nx, ny))
					}
				} else if nd + 1 < d && nh <= h + 1 {
					dists[(x, y)] = nd + 1;
				}
			}

			if dists[(x, y)] == usize::MAX {
				if !q.contains(&(x, y)) {
					// failed to find a suitable foothold, wait for neighbors to be calculated
					q.push_back((x, y))
				}
			} else if (x, y) == self.start {
				break;
			}
		}
		println!("{dists:?}");
		dists[self.start]
	}

	fn closest_valley(&self) -> usize {
		let mut dists = Dists {
			dists: &mut vec![usize::MAX; self.heights.len()],
			width: self.width,
		};
		dists[self.end] = 0;

		let max_x = self.width;
		let max_y = self.num_rows();
		let neighbors = |(x, y): (usize, usize)| {
			NEIGHBORS
				.iter()
				.filter_map(move |&(nx, ny)| x.checked_add_signed(nx).zip(y.checked_add_signed(ny)))
				.filter(move |(nx, ny)| *nx < max_x && *ny < max_y)
		};

		let mut q = neighbors(self.end).collect::<VecDeque<_>>();

		let mut closest_valley = self.start; // start has elevation `a`

		loop {
			let Some((x, y)) = q.pop_front() else { break };
			let d = dists[(x, y)];
			let h = self[(x, y)];

			for (nx, ny) in neighbors((x, y)) {
				let nd = dists[(nx, ny)];
				let nh = self[(nx, ny)];

				if nd == usize::MAX {
					if !q.contains(&(nx, ny)) {
						q.push_back((nx, ny))
					}
				} else if nd + 1 < d && nh <= h + 1 {
					dists[(x, y)] = nd + 1;
				}
			}

			let d = dists[(x, y)];

			if h == 0 && d < dists[closest_valley] {
				closest_valley = (x, y)
			}

			if d == usize::MAX {
				// failed to find a suitable foothold, wait for neighbors to be calculated
				q.push_back((x, y))
			} else if (x, y) == self.start {
				break;
			}
		}
		dists[closest_valley]
	}
}

impl Index<(usize, usize)> for Map {
	type Output = u8;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.heights[(index.1 * self.width) + index.0]
	}
}

impl Display for Map {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for row in self.rows() {
			for h in row {
				f.write_char((h + b'a') as char)?;
			}
			f.write_char('\n')?
		}
		Ok(())
	}
}

struct Dists<'a> {
	dists: &'a mut [usize],
	width: usize,
}

impl Debug for Dists<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		for row in self.dists.chunks(self.width) {
			for b in row {
				if *b == usize::MAX {
					f.write_str("    ")?
				} else {
					f.write_str(&format!("{b:4}"))?;
				}
			}
			f.write_char('\n')?
		}

		Ok(())
	}
}

impl Index<(usize, usize)> for Dists<'_> {
	type Output = usize;

	fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
		&self.dists[x + (y * self.width)]
	}
}

impl IndexMut<(usize, usize)> for Dists<'_> {
	fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
		&mut self.dists[x + (y * self.width)]
	}
}
