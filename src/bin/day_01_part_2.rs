use std::{path::Path, io::{BufReader, prelude::*}};
use anyhow::Result;
use waridley_aoc_2022::day_1::*;

fn main() -> Result<()> {
    let result = eval_part_2("day_1")?;
    println!("{result}");
    Ok(())
}
