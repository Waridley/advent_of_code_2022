use waridley_aoc_2022::input_file;
use std::io::{BufReader, prelude::*};

fn main() {
    let result = eval_part_2("day_2");
    println!("{result}");
}

pub fn eval_part_2(file: &str) -> String {
    let input = input_file(file).unwrap();
    let input = BufReader::new(input);
    let score = input.lines().into_iter()
    .map(|line| {
        let line = line.unwrap();
        let mut pair = line.split(' ');
        let opponent = pair.next().expect("line should not be empty");
        let outcome = pair.next().expect("missing your play");
        let outcome = match outcome {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            other => panic!("Unexpected response: {other}")
        };
        let opponent = match opponent {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            other => panic!("Unexpected opponent input: {other}")
        };
        let you = match (opponent, outcome) {
            (Rock, Lose) => Scissors,
            (Rock, Win) => Paper,
            (Paper, Lose) => Rock,
            (Paper, Win) => Scissors,
            (Scissors, Lose) => Paper,
            (Scissors, Win) => Rock,
            (opponent, Draw) => opponent,
        };
        (outcome as i64 * 3 + 3) + you as i64
    })
    .sum::<i64>();
    format!("{score}")
}

#[derive(Clone, Copy)]
#[repr(i64)]
enum Outcome {
    Lose = -1,
    Draw = 0,
    Win = 1,
}
use Outcome::*;

#[derive(Clone, Copy)]
#[repr(i64)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
use Shape::*;

#[cfg(test)]
#[test]
fn part_2() {
    let result = eval_part_2("day_2.example");
    assert_eq!(result, "12")
}
