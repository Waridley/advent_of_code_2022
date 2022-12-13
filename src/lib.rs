use std::io::BufReader;
use std::{fs::File, path::Path};

pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;

pub const INPUT_DIR: &str = "inputs";
pub type Lines = BufReader<File>;

pub fn input_file(name: &str) -> std::io::Result<File> {
	File::open(Path::new(INPUT_DIR).join(name))
}

pub fn input_lines(name: &str) -> std::io::Result<Lines> {
	Ok(Lines::new(input_file(name)?))
}
