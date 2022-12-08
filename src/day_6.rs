use crate::input_file;
use anyhow::Result;
use std::io::prelude::*;

pub fn eval_part_1(file: &str) -> Result<usize> {
    let mut input = input_file(file)?;
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    Ok(find_marker(&*buf, 4))
}

pub fn eval_part_2(file: &str) -> Result<usize> {
    let mut input = input_file(file)?;
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    Ok(find_marker(&*buf, 14))
}

pub fn find_marker(stream: &[u8], unique: usize) -> usize {
    let mut i = unique - 1;
    for window in stream.windows(unique) {
        i += 1;
        let mut dup = false;
        for c in 0..unique {
            if window[0..c].iter().chain(window[(c + 1)..].iter()).find(|val| **val == window[c]).is_some() {
                dup = true;
                break
            }
        }
        if !dup { break }
    }
    i
}
