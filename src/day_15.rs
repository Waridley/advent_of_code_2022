use crate::input_lines;
use anyhow::Result;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::{io::prelude::*, str::FromStr};

pub fn eval_part_1(file: &str, row: isize) -> Result<usize> {
	let sensors = input_lines(file)?
		.lines()
		.map(|line| line.unwrap().parse::<Sensor>().unwrap())
		.collect::<Vec<_>>();

	let mut spans = spans_for_row(&sensors, row);

	let mut beacons_in_row = HashSet::new();
	for sensor in &sensors {
		let (bx, by) = sensor.closest_beacon;
		if by == row {
			beacons_in_row.insert(bx);
		}
	}
	dbg!(&beacons_in_row);
	Ok(spans.drain(..).flatten().count() - beacons_in_row.len())
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let result = eval_part_1("day_15.example", 10)?;
	assert_eq!(result, 26);
	Ok(())
}

pub fn eval_part_2(file: &str, range: isize) -> Result<isize> {
	let sensors = input_lines(file)?
		.lines()
		.map(|line| line.unwrap().parse::<Sensor>().unwrap())
		.collect::<Vec<_>>();

	let mut searcher = (0..=range)
		.map(|row| {
			spans_for_row(&sensors, row)
				.into_iter()
				.map(|span| *span.start().max(&0)..=*span.end().min(&range))
				.collect::<Vec<_>>()
		})
		.enumerate()
		.flat_map(|(row, spans)| {
			let mut i = 0;
			loop {
				let mut covered = false;
				for span in spans.iter() {
					if span.contains(&i) {
						i = *span.end() + 1;
						covered = true;
						break;
					}
				}
				if !covered {
					break Some((i, row as isize));
				} else if i >= range {
					break None;
				}
			}
		});
	let (x, y) = searcher.next().expect("didn't find position");
	println!("Beacon: {{ {x}, {y} }}");
	assert!(searcher.next().is_none());

	Ok(x * 4_000_000 + y)
}

#[cfg(test)]
#[test]
fn part_2() -> Result<()> {
	let result = eval_part_2("day_15.example", 20)?;
	assert_eq!(result, 56000011);
	Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Sensor {
	position: (isize, isize),
	closest_beacon: (isize, isize),
}

impl FromStr for Sensor {
	type Err = <isize as FromStr>::Err;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		let s = s.strip_prefix("Sensor at x=").unwrap();
		let (x, s) = s.split_once(", y=").unwrap();
		let x = x.parse()?;
		let (y, s) = s.split_once(": closest beacon is at x=").unwrap();
		let y = y.parse()?;
		let (bx, by) = s.split_once(", y=").unwrap();
		let bx = bx.parse()?;
		let by = by.parse()?;
		Ok(Self {
			position: (x, y),
			closest_beacon: (bx, by),
		})
	}
}

impl Sensor {
	fn manhattan_dist(&self) -> usize {
		(self.closest_beacon.0 - self.position.0).unsigned_abs()
			+ (self.closest_beacon.1 - self.position.1).unsigned_abs()
	}

	fn span_for_row(&self, row: isize) -> Option<RangeInclusive<isize>> {
		let vdist = (row - self.position.1).abs();
		let hdist = self.manhattan_dist() as isize - vdist;
		(hdist >= 0).then_some((self.position.0 - hdist)..=(self.position.0 + hdist))
	}
}

fn spans_for_row(sensors: &[Sensor], row: isize) -> Vec<RangeInclusive<isize>> {
	let mut spans = sensors
		.iter()
		.flat_map(|sensor| sensor.span_for_row(row))
		.collect::<Vec<_>>();

	let mut buf = Vec::new();
	while let Some(last) = spans.pop() {
		buf.push(last);
		let curr = buf.last_mut().unwrap();
		loop {
			let mut merged_some = false;
			for i in 0..spans.len() {
				let span = &spans[i];
				if curr.contains(span.start()) || curr.contains(span.end()) {
					let span = spans.remove(i);
					*curr = *curr.start().min(span.start())..=*curr.end().max(span.end());
					merged_some = true;
					break;
				}
			}
			if !merged_some {
				break;
			}
		}
	}
	buf
}
