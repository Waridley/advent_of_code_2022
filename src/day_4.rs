use std::io::{BufReader, prelude::*};
use anyhow::Result;
use crate::input_file;

pub fn eval_part_1(file: &str) -> Result<u64> {
    let input = input_file(file)?;
    let input = BufReader::new(input);
    let mut sum = 0;
    for line in input.lines() {
        let line = line?;
        let (left, right) = line.split_once(',').unwrap();
        let (l_start, l_end) = left.split_once('-').unwrap();
        let (r_start, r_end) = right.split_once('-').unwrap();
        let left = l_start.parse::<u64>()?..=l_end.parse::<u64>()?;
        let right = r_start.parse::<u64>()?..=r_end.parse::<u64>()?;
        if (left.contains(right.start()) && left.contains(right.end()))
            || (right.contains(left.start()) && right.contains(left.end())) {
            sum += 1;
        }
    }
    Ok(sum)
}

pub fn eval_part_2(file: &str) -> Result<u64> {
    let input = input_file(file)?;
    let input = BufReader::new(input);
    let mut sum = 0;
    for line in input.lines() {
        let line = line?;
        let (left, right) = line.split_once(',').unwrap();
        let (l_start, l_end) = left.split_once('-').unwrap();
        let (r_start, r_end) = right.split_once('-').unwrap();
        let left = l_start.parse::<u64>()?..=l_end.parse::<u64>()?;
        let right = r_start.parse::<u64>()?..=r_end.parse::<u64>()?;
        if left.contains(right.start()) || left.contains(right.end())
            || right.contains(left.start()) || right.contains(left.end()) {
            sum += 1;
        }
    }

    Ok(sum)
}