use crate::input_file;
use anyhow::Result;
use std::io::prelude::*;

pub fn eval_part_1(file: &str) -> Result<usize> {
	let mut input = input_file(file)?;
	let mut buf = Vec::new();
	input.read_to_end(&mut buf)?;
	Ok(find_marker(&buf, 4))
}

#[cfg(test)]
#[test]
fn part_1() {
	assert_eq!(
		find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), 4),
		7
	);
	assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 4), 5);
	assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 4), 6);
	assert_eq!(
		find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 4),
		10
	);
	assert_eq!(
		find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 4),
		11
	);
}

pub fn eval_part_2(file: &str) -> Result<usize> {
	let mut input = input_file(file)?;
	let mut buf = Vec::new();
	input.read_to_end(&mut buf)?;
	Ok(find_marker(&buf, 14))
}

#[cfg(test)]
#[test]
fn part_2() {
	assert_eq!(
		find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), 14),
		19
	);
	assert_eq!(
		find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 14),
		23
	);
	assert_eq!(
		find_marker("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 14),
		23
	);
	assert_eq!(
		find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 14),
		29
	);
	assert_eq!(
		find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 14),
		26
	);
}

pub fn find_marker(stream: &[u8], unique: usize) -> usize {
	let mut i = unique - 1;
	for window in stream.windows(unique) {
		i += 1;
		let mut dup = false;
		for c in 0..unique {
			if window[0..c]
				.iter()
				.chain(window[(c + 1)..].iter())
				.any(|val| *val == window[c])
			{
				dup = true;
				break;
			}
		}
		if !dup {
			break;
		}
	}
	i
}
