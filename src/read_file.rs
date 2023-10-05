use std::fs;
use std::io;

pub fn get_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn count_words(content: &str) -> usize {
    content.lines().count()
}
