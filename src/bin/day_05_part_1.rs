use waridley_aoc_2022::day_05::eval_part_1;

pub fn main() {
	let result = eval_part_1("day_5").unwrap();
	println!("{result}");
}

#[cfg(test)]
#[test]
fn part_1() {
	let result = eval_part_1("day_5.example").unwrap();
	assert_eq!(result, "CMZ")
}
