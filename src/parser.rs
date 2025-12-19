use crate::{
    ArgumentKey,
    error::Error,
    values::{Alignment, Piece, Precision, Specifier, Type, Width},
};


/*
Performance: Depending on the complexity of the fmt string it takes about 25 ns per arg
*/
pub fn parse_pieces(input: &str) -> Result<Vec<Piece>, Error> {
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
                    // Parsing the specifier first, because if it contains a precision .*
                    // then the index of the precision argument is before the omitted argument index
                    // Format: [argument_index][name][':' [fill][align][sign]['#']['0'][width]['.' precision][type]]

                    let specifier = separator.map(|seperator_index| {
                        let mut current_specifier_index = seperator_index + 1;
                        let fill_character = match (
                            chars[current_specifier_index],
                            chars[current_specifier_index + 1],
                        ) {
                            (char, b'<') | (char, b'^') | (char, b'>') => {
                                current_specifier_index += 1;
                                char as char
                            }
                            _ => ' ',
                        };
                        let alignment = match chars[current_specifier_index] {
                            b'<' => {
                                current_specifier_index += 1;
                                Alignment::Left
                            }
                            b'>' => {
                                current_specifier_index += 1;
                                Alignment::Right
                            }
                            b'^' => {
                                current_specifier_index += 1;
                                Alignment::Center
                            }
                            _ => Alignment::Auto,
                        };
                        let sign = match chars[current_specifier_index] {
                            b'+' => {
                                current_specifier_index += 1;
                                true
                            }
                            _ => false,
                        };
                        let alternate_form = match chars[current_specifier_index] {
                            b'#' => {
                                current_specifier_index += 1;
                                true
                            }
                            _ => false,
                        };
                        let pad_zero = match chars[current_specifier_index] {
                            b'0' => {
                                current_specifier_index += 1;
                                true
                            }
                            _ => false,
                        };
                        let width = {
                            if (chars[current_specifier_index] as char).is_digit(10) {
                                let mut until_index = current_specifier_index;
                                while (chars[until_index] as char).is_digit(10) {
                                    until_index += 1;
                                }
                                let amount_str = &input[current_specifier_index..until_index];
                                current_specifier_index = until_index;
                                Width::Fixed(amount_str.parse::<u16>().unwrap())
                            } else if chars[current_specifier_index].is_ascii_alphabetic() {
                                let end_index = current_specifier_index
                                    + input[current_specifier_index..current_char]
                                        .find('$')
                                        .unwrap();

                                let key = ArgumentKey::Name(
                                    input[current_specifier_index..end_index].to_string(),
                                );
                                current_specifier_index = end_index + 1;
                                Width::Dynamic(key)
                            } else {
                                Width::Fixed(0)
                            }
                        };

                        let precision = if chars[current_specifier_index] == b'.' {
                            current_specifier_index += 1;
                            if (chars[current_specifier_index] as char).is_digit(10) {
                                let mut until_index = current_specifier_index;
                                while (chars[until_index] as char).is_digit(10) {
                                    until_index += 1;
                                }
                                let amount_str = &input[current_specifier_index..until_index];
                                current_specifier_index = until_index;
                                Precision::Fixed(amount_str.parse::<u16>().unwrap())
                            } else if chars[current_specifier_index].is_ascii_alphabetic() {
                                let end_index = current_specifier_index
                                    + input[current_specifier_index..current_char]
                                        .find('$')
                                        .unwrap();

                                let key = ArgumentKey::Name(
                                    input[current_specifier_index..end_index].to_string(),
                                );
                                current_specifier_index = end_index + 1;
                                Precision::Dynamic(key)
                            } else if chars[current_specifier_index] == b'*' {
                                internal_index += 1;
                                Precision::Dynamic(ArgumentKey::Index(internal_index - 1))
                            } else {
                                unreachable!()
                            }
                        } else {
                            Precision::Auto
                        };
                        let ty = match chars[current_specifier_index] {
                            b'?' => Type::Debug,
                            b'b' => Type::Binary,
                            b'o' => Type::Octal,
                            b'e' => Type::LowerExp,
                            b'E' => Type::UpperExp,
                            b'x' => Type::LowerHex,
                            b'X' => Type::UpperHex,
                            b'p' => Type::Pointer,
                            _ => Type::Display,
                        };

                        Specifier {
                            ty: ty,
                            alternate_form: alternate_form,
                            fill_character: fill_character,
                            alignment: alignment,
                            sign: sign,
                            pad_zero: pad_zero,
                            width: width,
                            precision: precision,
                        }
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
                                // Err(_) => ArgumentKey::Name {
                                //     start: name_start,
                                //     end: name_end,
                                // },
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
                    return Err(Error::InvalidFormat);
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
