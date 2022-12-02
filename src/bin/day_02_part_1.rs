use anyhow::Result;
use waridley_aoc_2022::input_file;
use std::{
    cmp::Ordering,
    io::{BufReader, prelude::*}
};

fn main() -> Result<()> {
    let result = eval_part_1("day_1");
    println!("{result}");
    Ok(())
}

pub fn eval_part_1(file: &str) -> String {
    let input = input_file(file).unwrap();
    let input = BufReader::new(input);
    let score = input.lines().into_iter()
    .map(|line| {
        let line = line.unwrap();
        let mut pair = line.split(' ');
        let opponent = pair.next().expect("line should not be empty");
        let you = pair.next().expect("missing your play");
        let mut score: u64 = match you {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            other => panic!("Unexpected response: {other}")
        };
        let opponent_score = match opponent {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            other => panic!("Unexpected opponent input: {other}")
        };
        score += match score.cmp(&opponent_score) {
            Ordering::Greater => 6,
            Ordering::Equal => 3,
            Ordering::Less => 0,
        };
        score
    })
    .sum::<u64>();
    format!("{score}")
}

#[cfg(test)]
#[test]
fn part_1() {
    let result = eval_part_1("day_2.example");
    assert_eq!(result, "15")
}
