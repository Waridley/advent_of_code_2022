use anyhow::Result;
use waridley_aoc_2022::day_10::eval_part_2;

fn main() -> Result<()> {
	eval_part_2("day_10", &mut std::io::stdout())
}

#[cfg(test)]
#[test]
fn part_1() -> Result<()> {
	let mut screen = vec![];
	let result = eval_part_2("day_10.example", &mut screen)?;
	let screen = String::from_utf8(screen)?;
	println!("{screen}");
	assert_eq!(screen,
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
	Ok(())
}
