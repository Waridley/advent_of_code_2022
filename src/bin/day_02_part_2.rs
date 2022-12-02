use anyhow::Result;
use waridley_aoc_2022::input_file;

fn main() -> Result<()> {
    let result = eval_part_2("day_2")?;
    println!("{result}");
    Ok(())
}

pub fn eval_part_2(file: &str) -> Result<String> {
    let _input = input_file(file)?;
    todo!()
}

#[cfg(test)]
#[test]
fn part_2() {
    let result = eval_part_2("day_2.example").unwrap();
    assert_eq!(result, "get example answer")
}
