mod word;

use rayon::prelude::*;
use std::fs;
use std::io;
use word::Word;

fn main() -> Result<(), io::Error> {
    let contents = fs::read_to_string("words.txt")?;
    let lines: Vec<&str> = contents.lines().collect();

    lines.par_iter().for_each(|&secret_word| {
        lines.par_iter().for_each(|&input_word| {
            let _ = Word::from(secret_word).compare(Word::from(input_word));
        });
    });

    Ok(())
}
