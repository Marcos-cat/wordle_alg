pub mod colors;
pub mod compare;

pub struct Word {
    letters: Vec<char>,
}

use colors::OutputColors;

impl Word {
    pub fn compare(
        self,
        input_word: Word,
    ) -> Result<OutputColors, compare::CompareError> {
        compare::compare(self, input_word)
    }
    pub fn from(letters: &str) -> Word {
        let letters = letters.trim().to_lowercase().chars().collect();

        Word { letters }
    }
}

use std::fmt;

impl fmt::Display for Word {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.pad(&self.letters.iter().collect::<String>().to_uppercase())
    }
}
