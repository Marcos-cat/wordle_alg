#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Yellow,
    Green,
}

#[derive(Debug)]
pub struct OutputColors {
    colors: Vec<Color>,
}
impl OutputColors {
    pub fn new(colors: Vec<Color>) -> Self {
        OutputColors { colors }
    }
}

impl PartialEq for OutputColors {
    fn eq(&self, other: &Self) -> bool {
        self.colors.iter().eq(other.colors.iter())
    }
}

use std::fmt;

impl fmt::Display for OutputColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(
            &self
                .colors
                .iter()
                .map(|color| match color {
                    Color::Green => 'g',
                    Color::Yellow => 'y',
                    Color::Black => 'b',
                })
                .collect::<String>(),
        )
    }
}
