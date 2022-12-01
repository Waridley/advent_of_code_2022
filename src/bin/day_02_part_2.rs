use anyhow::Result;

fn main() -> Result<()> {
    let result = eval_part_2("day_1")?;
    println!("{result}");
    Ok(())
}

pub fn eval_part_2(_file: &str) -> Result<String> {
    todo!()
}

#[cfg(test)]
#[test]
fn part_2() {
    let result = eval_part_2("day_1.example").unwrap();
    assert_eq!(result, "get example answer")
}
