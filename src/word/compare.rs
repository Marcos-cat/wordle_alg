use super::colors::Color;
use super::*;

#[derive(Debug)]
pub enum CompareError {
    WordLengthsDiffer,
}

pub fn compare(
    secret_word: Word,
    input_word: Word,
) -> Result<OutputColors, CompareError> {
    if secret_word.letters.len() != input_word.letters.len() {
        return Err(CompareError::WordLengthsDiffer);
    }

    let mut secret_diagram = WordDiagram::new(&secret_word);
    let mut input_diagram = WordDiagram::new(&input_word);

    Ok(secret_diagram.colorize(&mut input_diagram))
}

struct WordDiagram {
    chars: Vec<DiagramLetter>,
}

impl WordDiagram {
    fn colorize(&mut self, input_diagram: &mut WordDiagram) -> OutputColors {
        self.make_connections(input_diagram);

        let output_colors = input_diagram
            .chars
            .iter()
            .map(|letter| match letter.connection {
                Connection::Alone => Color::Black,
                Connection::ConnectedOther => Color::Yellow,
                Connection::ConnectedInplace => Color::Green,
            })
            .collect();

        OutputColors::new(output_colors)
    }
    fn make_connections(&mut self, input_diagram: &mut WordDiagram) {
        for (secret_index, secret_letter) in self.chars.iter_mut().enumerate() {
            let connected_letter =
                input_diagram.chars.iter_mut().enumerate().find(
                    |(_, input_letter)| {
                        input_letter.char == secret_letter.char
                            && input_letter.connection == Connection::Alone
                    },
                );

            match connected_letter {
                None => continue,
                Some((connected_index, connected_letter)) => {
                    let is_inplace = secret_index == connected_index;

                    connected_letter.connect(is_inplace);
                    secret_letter.connect(is_inplace);
                }
            }
        }
    }
    fn new(word: &Word) -> Self {
        let chars = word
            .letters
            .iter()
            .map(|&c| DiagramLetter::new(c))
            .collect();

        WordDiagram { chars }
    }
}

struct DiagramLetter {
    char: char,
    connection: Connection,
}

impl DiagramLetter {
    fn connect(&mut self, is_inplace: bool) {
        self.connection = match is_inplace {
            true => Connection::ConnectedInplace,
            false => Connection::ConnectedOther,
        };
    }

    fn new(c: char) -> Self {
        DiagramLetter {
            char: c,
            connection: Connection::Alone,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Connection {
    ConnectedOther,
    ConnectedInplace,
    Alone,
}
