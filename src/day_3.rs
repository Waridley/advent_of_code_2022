use crate::input_file;
use anyhow::Result;
use std::io::{BufReader, prelude::*};

pub fn eval_part_1(file: &str) -> Result<usize> {
    let input = input_file(file)?;
    let input = BufReader::new(input);
    let mut sum = 0;
    for line in input.lines() {
        let line = line?;
        let (left, right) = line.split_at(line.len() / 2);
        let l_contents = contents(left);
        for byte in right.bytes() {
            if l_contents & (1 << (byte - b'A')) != 0 {
                sum += priority(byte);
                break
            }
        }

    }
    Ok(sum)
}

pub fn eval_part_2(file: &str) -> Result<usize> {
    let input = input_file(file)?;
    let input = BufReader::new(input);
    let mut sum = 0;

    let mut lines = input.lines();
    loop {
        let lines = [
            if let Some(line) = lines.next() { line } else { break }?,
            lines.next().expect("multiple of 3 lines")?,
            lines.next().expect("multiple of 3 lines")?,
        ];
        let matching =contents(&lines[0]) & contents(&lines[1]) & contents(&lines[2]);
        let byte = matching.trailing_zeros() as u8 + b'A';
        sum += priority(byte);
    }

    Ok(sum)
}

fn contents(sack: &str) -> u64 {
    let mut contents = 0u64;
    for byte in sack.bytes() {
        contents |= 1 << (byte - b'A');
    }
    contents
}

fn priority(byte: u8) -> usize {
    let priority = (byte ^ 32) - (b'A' - 1);
    if priority > 26 { (priority - 6) as usize } else { priority as usize }
}