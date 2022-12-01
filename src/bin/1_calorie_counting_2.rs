use std::{path::Path, io::{BufReader, prelude::*}};
use anyhow::Result;
use waridley_aoc_2022::INPUT_DIR;

//const INPUT_FILE: &str = "calorie_counting.example";
const INPUT_FILE: &str = "calorie_counting";

fn main() -> Result<()> {
    let input = std::fs::File::open(Path::new(INPUT_DIR).join(INPUT_FILE))?;
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
    println!("{top_3_sum}");
    Ok(())
}