use crate::input_lines;
use std::io::prelude::*;

pub fn eval_part_1(file: &str) -> String {
	let input = input_lines(file).unwrap();
	let score = input
		.lines()
		.into_iter()
		.map(|line| {
			let line = line.unwrap();
			let mut pair = line.split(' ');
			let opponent = pair.next().expect("line should not be empty");
			let you = pair.next().expect("missing your play");
			let mut score: i64 = match you {
				"X" => 1,
				"Y" => 2,
				"Z" => 3,
				other => panic!("Unexpected response: {other}"),
			};
			let opponent_score = match opponent {
				"A" => 1,
				"B" => 2,
				"C" => 3,
				other => panic!("Unexpected opponent input: {other}"),
			};
			score += ((score - opponent_score + 4) % 3) * 3;
			score
		})
		.sum::<i64>();
	format!("{score}")
}

#[cfg(test)]
#[test]
fn part_1() {
	let result = eval_part_1("day_2.example");
	assert_eq!(result, "15")
}

pub fn eval_part_2(file: &str) -> String {
	let input = input_lines(file).unwrap();
	let score = input
		.lines()
		.into_iter()
		.map(|line| {
			let line = line.unwrap();
			let mut pair = line.split(' ');
			let opponent = pair.next().expect("line should not be empty");
			let outcome = pair.next().expect("missing your play");
			let outcome = match outcome {
				"X" => Lose,
				"Y" => Draw,
				"Z" => Win,
				other => panic!("Unexpected response: {other}"),
			};
			let opponent = match opponent {
				"A" => Rock,
				"B" => Paper,
				"C" => Scissors,
				other => panic!("Unexpected opponent input: {other}"),
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
			outcome as i64 + you as i64
		})
		.sum::<i64>();
	format!("{score}")
}

#[derive(Clone, Copy)]
#[repr(i64)]
enum Outcome {
	Lose = 0,
	Draw = 3,
	Win = 6,
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
