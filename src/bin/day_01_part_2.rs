use std::{path::Path, io::{BufReader, prelude::*}};
use anyhow::Result;
use waridley_aoc_2022::INPUT_DIR;

fn main() -> Result<()> {
    let result = eval_part_2("day_1")?;
    println!("{result}");
    Ok(())
}

pub fn eval_part_2(file: &str) -> Result<String> {
    let input = std::fs::File::open(Path::new(INPUT_DIR).join(file))?;
    let input = BufReader::new(input);

    let mut elves = vec![0u64];

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            elves.push(0);
        } else {
            let curr = elves.last_mut().unwrap();
            *curr += line.parse::<u64>()?;
        }
    }
    elves.sort();
    let top_3_sum = elves.iter().rev().copied().take(3).reduce(|acc, calories| acc + calories).unwrap_or(0);
    Ok(format!("{top_3_sum}"))
}

#[cfg(test)]
#[test]
fn part_2() {
    let result = eval_part_2("day_1.example").unwrap();
    assert_eq!(result, "45000")
}
