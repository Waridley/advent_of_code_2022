use std::io::BufReader;
use std::{fs::File, path::Path};

pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

pub const INPUT_DIR: &str = "inputs";
pub type Lines = BufReader<File>;

pub fn input_file(name: &str) -> std::io::Result<File> {
	File::open(Path::new(INPUT_DIR).join(name))
}

pub fn input_lines(name: &str) -> std::io::Result<Lines> {
	Ok(Lines::new(input_file(name)?))
}
