use anyhow::Result;
use waridley_aoc_2022::input_file;

fn main() -> Result<()> {
    let result = eval_part_1("day_1")?;
    println!("{result}");
    Ok(())
}

pub fn eval_part_1(file: &str) -> Result<String> {
    let _input = input_file(file)?;
    todo!()
}

#[cfg(test)]
#[test]
fn part_1() {
    let result = eval_part_1("day_1.example").unwrap();
    assert_eq!(result, "get example answer")
}
