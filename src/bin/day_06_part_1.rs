use waridley_aoc_2022::day_06::*;

pub fn main() {
	let result = eval_part_1("day_6").unwrap();
	println!("{result}");
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
