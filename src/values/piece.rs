use core::fmt::Write;

use crate::{ArgumentKey, Error, values::Specifier};

#[derive(Debug, Clone)]
pub enum Piece {
    Literal(String),
    BracketOpen,
    BracketClose,
    Argument {
        key: ArgumentKey,
        specifier: Option<Specifier>,
    },
}

impl Piece {
    pub fn parse(input: &str) -> Result<Vec<Self>, Error> {
        let mut pieces: Vec<Piece> = Vec::with_capacity(10);

        let mut cursor = 0;
        let mut current_char = 0;
        let mut bracket = None;
        let mut separator = None;
        let mut internal_index = 0;

        let chars = input.as_bytes();
        while current_char < chars.len() {
            let char = chars[current_char];
            match char {
                b':' if bracket == Some(b'{') => {
                    separator = Some(current_char);
                }
                b'{' | b'}' => match (bracket, char) {
                    (None, _) => {
                        if cursor + 1 < current_char {
                            pieces.push(Piece::Literal(input[cursor..current_char].to_string()));
                        }
                        bracket = Some(char);
                        cursor = current_char;
                    }
                    (Some(b'{'), b'}') => {
                        let specifier = separator.map(|seperator_index| {
                            Specifier::parse(
                                &input[seperator_index + 1..current_char],
                                &mut internal_index,
                            )
                        });

                        let key = {
                            let (name_start, name_end) = match separator {
                                None => (cursor + 1, current_char),
                                Some(seperator_index) => (cursor + 1, seperator_index),
                            };

                            if name_start == name_end {
                                internal_index += 1;
                                ArgumentKey::Index(internal_index - 1)
                            } else {
                                match input[name_start..name_end].parse::<usize>() {
                                    Ok(arg_index) => ArgumentKey::Index(arg_index),
                                    Err(_) => {
                                        ArgumentKey::Name(input[name_start..name_end].to_string())
                                    }
                                }
                            }
                        };

                        pieces.push(Piece::Argument { key, specifier });

                        separator = None;
                        bracket = None;
                        cursor = current_char + 1;
                    }
                    (Some(b'{'), b'{') => {
                        pieces.push(Piece::BracketOpen);
                        bracket = None;
                        cursor = current_char + 1;
                    }
                    (Some(b'}'), b'}') => {
                        pieces.push(Piece::BracketClose);
                        bracket = None;
                        cursor = current_char + 1;
                    }
                    _ => {
                        return Err(Error::UnexpectedToken);
                    }
                },
                _ => {}
            }
            current_char += 1;
        }

        if cursor < current_char {
            pieces.push(Piece::Literal(input[cursor..current_char].to_string()));
        }

        Ok(pieces)
    }
}

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Piece::Literal(literal) => f.write_str(&literal),
            Piece::BracketOpen => f.write_str("{{"),
            Piece::BracketClose => f.write_str("}}"),
            Piece::Argument { key, specifier } => {
                f.write_char('{')?;
                write!(f, "{key}")?;
                if let Some(specifier) = specifier {
                    f.write_char(':')?;
                    write!(f, "{specifier}")?;
                }
                f.write_char('}')
            }
        }
    }
}
