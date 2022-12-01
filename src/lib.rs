use std::{fs::File, path::Path};

pub const INPUT_DIR: &str = "inputs";

pub fn input_file(name: &str) -> std::io::Result<File> {
    File::open(Path::new(INPUT_DIR).join(name))
}
