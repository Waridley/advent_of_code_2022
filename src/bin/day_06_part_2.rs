use waridley_aoc_2022::day_6::*;

pub fn main() {
	let result = eval_part_2("day_6").unwrap();
	println!("{result}");
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
