use std::io::BufReader;
use std::{fs::File, path::Path};

pub mod day_01;
pub mod day_02;
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
pub mod day_14;
pub mod day_15;

pub const INPUT_DIR: &str = "inputs";
pub type Lines = BufReader<File>;

pub fn input_file(name: &str) -> std::io::Result<File> {
	File::open(Path::new(INPUT_DIR).join(name))
}

pub fn input_lines(name: &str) -> std::io::Result<Lines> {
	Ok(Lines::new(input_file(name)?))
}

pub trait IntoContiguous<T> {
	type Out;
	fn into_contiguous(self, width: usize) -> Self::Out
	where
		T: Default;
}

impl<T> IntoContiguous<T> for Vec<Vec<T>> {
	type Out = Vec<T>;
	fn into_contiguous(self, width: usize) -> Self::Out
	where
		T: Default,
	{
		let mut out = Vec::with_capacity(self.len() * width);
		for mut row in self {
			assert_eq!(row.len(), width);
			out.append(&mut row);
		}
		out
	}
}
