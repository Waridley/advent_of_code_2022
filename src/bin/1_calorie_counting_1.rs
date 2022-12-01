use std::{path::Path, io::{BufReader, prelude::*}};
use anyhow::Result;
use waridley_aoc_2022::INPUT_DIR;

fn main() -> Result<()> {
    let input = std::fs::File::open(Path::new(INPUT_DIR).join("calorie_counting"))?;
    let input = BufReader::new(input);

    let mut elves = vec![(0u64, 0u64)];
    let mut fattest_elf_index = 0usize;

    for line in input.lines() {
        let line = line?;
        if line.is_empty() {
            elves.push((0, 0));
        } else {
            let fattest_elf_calories = elves[fattest_elf_index].1;
            let curr = elves.last_mut().unwrap();
            curr.0 += 1;
            curr.1 += line.parse::<u64>()?;
            if curr.1 > fattest_elf_calories {
                fattest_elf_index = elves.len() - 1;
            }
        }
    }
    println!("{}", elves[fattest_elf_index].1);
    Ok(())
}