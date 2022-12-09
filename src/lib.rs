use std::io::BufReader;
use std::{fs::File, path::Path};

pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;

pub const INPUT_DIR: &str = "inputs";
pub type Lines = BufReader<File>;

pub fn input_file(name: &str) -> std::io::Result<File> {
	File::open(Path::new(INPUT_DIR).join(name))
}
