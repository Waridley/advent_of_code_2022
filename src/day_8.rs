use crate::input_file;
use anyhow::Result;
use std::ops::Index;
use std::{fs::File, io::prelude::*, slice::ChunksMut};

/// Height value mask
const VAL: u8 = 0b1111;

// Hidden marker bits
const LTR: u8 = 1 << 7;
const RTL: u8 = 1 << 6;
const TTB: u8 = 1 << 5;
const BTT: u8 = 1 << 4;

pub fn eval_part_1(file: &str) -> Result<usize> {
	let mut map = Map::build(input_file(file)?)?;
	map.check_visibility();
	let sum = map.map.iter().filter(|height| *height >> 4 != VAL).count();
	Ok(sum)
}

pub fn eval_part_2(file: &str) -> Result<u64> {
	let map = Map::build(input_file(file)?)?;
	Ok(map.max_view_dist())
}

#[derive(Debug)]
pub struct Map {
	row_width: usize,
	map: Box<[u8]>,
}

impl Map {
	fn build(input: File) -> Result<Self> {
		let mut map = vec![];
		let mut width = None;
		let mut count = 0;
		for byte in input.bytes() {
			let byte = byte?;

			if width.is_none() {
				if byte == b'\n' {
					width = Some(count);
				} else {
					count += 1;
				}
			}

			if byte != b'\n' {
				map.push(byte - b'0');
			}
		}
		Ok(Map {
			row_width: width.unwrap(),
			map: map.into_boxed_slice(),
		})
	}

	fn num_rows(&self) -> usize {
		self.map.len() / self.row_width
	}

	fn rows_mut(&mut self) -> ChunksMut<'_, u8> {
		self.map.chunks_mut(self.row_width)
	}

	fn check_visibility(&mut self) {
		for row in self.rows_mut() {
			let mut ltr = row.iter_mut();
			let mut tallest_so_far = *ltr.next().unwrap() & VAL;
			for height in ltr {
				if *height & VAL <= tallest_so_far {
					*height |= LTR
				} else {
					tallest_so_far = *height & VAL
				}
			}
			let mut rtl = row.iter_mut().rev();
			let mut tallest_so_far = *rtl.next().unwrap() & VAL;
			for height in rtl {
				if *height & VAL <= tallest_so_far {
					*height |= RTL
				} else {
					tallest_so_far = *height & VAL
				}
			}
		}

		for i in 0..self.row_width {
			let mut rows = self.rows_mut();
			let mut tallest_so_far = rows.next().unwrap()[i] & VAL;
			for row in rows {
				let height = &mut row[i];
				if *height & VAL <= tallest_so_far {
					*height |= TTB
				} else {
					tallest_so_far = *height & VAL
				}
			}
			let mut rows = self.rows_mut().rev();
			let mut tallest_so_far = rows.next().unwrap()[i] & VAL;
			for row in rows {
				let height = &mut row[i];
				if *height & VAL <= tallest_so_far {
					*height |= BTT
				} else {
					tallest_so_far = *height & VAL
				}
			}
		}
	}

	fn view_dist(&self, col: usize, row: usize) -> u64 {
		let h = self[(col, row)];
		let mut l = 0u64;
		for c in (0..col).rev() {
			l += 1;
			if self[(c, row)] >= h {
				break;
			}
		}

		let mut r = 0u64;
		for c in (col + 1)..self.row_width {
			r += 1;
			if self[(c, row)] >= h {
				break;
			}
		}

		let mut u = 0u64;
		for r in (0..row).rev() {
			u += 1;
			if self[(col, r)] >= h {
				break;
			}
		}

		let mut d = 0u64;
		for r in (row + 1)..self.num_rows() {
			d += 1;
			if self[(col, r)] >= h {
				break;
			}
		}

		l * r * u * d
	}

	fn max_view_dist(&self) -> u64 {
		let mut dist = 0;
		for i in 0..self.map.len() {
			let c = i % self.row_width;
			let r = i / self.row_width;
			let d = self.view_dist(c, r);
			if d > dist {
				dist = d
			}
		}
		dist
	}
}

impl Index<(usize, usize)> for Map {
	type Output = u8;

	fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
		&self.map[(row * self.row_width) + col]
	}
}
