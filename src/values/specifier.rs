use core::fmt::Write;

use crate::{
    ArgumentKey, Error,
    values::{Alignment, Precision, Type, Width},
};

/// Argument formatting specifier.
/// ```rust
/// use dfmt::*;
///
/// Specifier::default()
///     .ty(Type::Debug)
///     .alternate_form(true)
///     .sign(false)
///     .pad_zero(false)
///     .fill_character('*')
///     .alignment(Alignment::Center)
///     .width(Width::Fixed(20))
///     .precision(Precision::Auto);
/// ```
#[derive(Debug, Clone)]
pub struct Specifier {
    pub(crate) ty: Type,
    pub(crate) alternate_form: bool,
    pub(crate) fill_character: char,
    pub(crate) alignment: Alignment,
    pub(crate) sign: bool,
    pub(crate) pad_zero: bool,
    pub(crate) width: Width,
    pub(crate) precision: Precision,
}

impl Specifier {
    /// Attempt to parse a specifier.
    pub fn parse(input: &str, internal_index: &mut usize) -> Result<Self, Error> {
        // Parsing the specifier first, because if it contains a precision .*
        // then the index of the precision argument is before the omitted argument index
        // Format: [argument_index][name][':' [fill][align][sign]['#']['0'][width]['.' precision][type]]
        let mut current_specifier_index = 0;
        let chars = input.as_bytes();

        let input_len = input.len();
        if input_len == 0 {
            return Ok(Specifier::default());
        }
        let remaining = |pos: usize| input_len - pos;

        let fill_character = if remaining(current_specifier_index) > 1 {
            match (
                chars[current_specifier_index],
                chars[current_specifier_index + 1],
            ) {
                (char, b'<') | (char, b'^') | (char, b'>') => {
                    current_specifier_index += 1;
                    char as char
                }
                _ => ' ',
            }
        } else {
            ' '
        };
        let alignment = if remaining(current_specifier_index) > 0 {
            match chars[current_specifier_index] {
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
            }
        } else {
            Alignment::Auto
        };
        let sign = remaining(current_specifier_index) > 0
            && match chars[current_specifier_index] {
                b'+' => {
                    current_specifier_index += 1;
                    true
                }
                _ => false,
            };
        let alternate_form = remaining(current_specifier_index) > 0
            && match chars[current_specifier_index] {
                b'#' => {
                    current_specifier_index += 1;
                    true
                }
                _ => false,
            };
        let pad_zero = remaining(current_specifier_index) > 0
            && match chars[current_specifier_index] {
                b'0' => {
                    current_specifier_index += 1;
                    true
                }
                _ => false,
            };
        let width = if remaining(current_specifier_index) > 0
            && chars[current_specifier_index] != b'.'
        {
            if (chars[current_specifier_index] as char).is_ascii_digit() {
                let mut until_index = current_specifier_index;
                while remaining(until_index + 1) > 0
                    && (chars[until_index + 1] as char).is_ascii_digit()
                {
                    until_index += 1;
                }
                until_index += 1;
                let amount_str = &input[current_specifier_index..until_index];
                current_specifier_index = until_index;
                Width::Fixed(amount_str.parse::<u16>().unwrap())
            } else if let Some(var_index) = input[current_specifier_index..].find('$') {
                let end_index = current_specifier_index + var_index;

                let key = ArgumentKey::Name(input[current_specifier_index..end_index].to_string());
                current_specifier_index = end_index + 1;
                Width::Dynamic(key)
            } else {
                Width::Fixed(0)
            }
        } else {
            Width::Fixed(0)
        };

        let precision = if remaining(current_specifier_index) > 0
            && chars[current_specifier_index] == b'.'
        {
            current_specifier_index += 1;
            if (chars[current_specifier_index] as char).is_ascii_digit() {
                let mut until_index = current_specifier_index;
                while remaining(until_index + 1) > 0
                    && (chars[until_index + 1] as char).is_ascii_digit()
                {
                    until_index += 1;
                }
                until_index += 1;
                let amount_str = &input[current_specifier_index..until_index];
                current_specifier_index = until_index;
                Precision::Fixed(amount_str.parse::<u16>().unwrap())
            } else if let Some(var_index) = input[current_specifier_index..].find('$') {
                let end_index = current_specifier_index + var_index;

                let key = ArgumentKey::Name(input[current_specifier_index..end_index].to_string());
                current_specifier_index = end_index + 1;
                Precision::Dynamic(key)
            } else if chars[current_specifier_index] == b'*' {
                *internal_index += 1;
                current_specifier_index += 1;
                Precision::Dynamic(ArgumentKey::Index(*internal_index - 1))
            } else {
                Precision::Auto
            }
        } else {
            Precision::Auto
        };

        let ty = if remaining(current_specifier_index) > 0 {
            match chars[current_specifier_index] {
                b'?' => Type::Debug,
                b'b' => Type::Binary,
                b'o' => Type::Octal,
                b'e' => Type::LowerExp,
                b'E' => Type::UpperExp,
                b'x' => Type::LowerHex,
                b'X' => Type::UpperHex,
                b'p' => Type::Pointer,
                _ => Type::Display,
            }
        } else {
            Type::Display
        };

        Ok(Specifier {
            ty,
            alternate_form,
            fill_character,
            alignment,
            sign,
            pad_zero,
            width,
            precision,
        })
    }

    #[cfg(feature = "nightly_formatting_options")]
    pub(crate) fn formatting_options(&self) -> core::fmt::FormattingOptions {
        let mut options = core::fmt::FormattingOptions::new();
        options
            .fill(self.fill_character)
            .align(match self.alignment {
                Alignment::Left => Some(core::fmt::Alignment::Left),
                Alignment::Right => Some(core::fmt::Alignment::Right),
                Alignment::Center => Some(core::fmt::Alignment::Center),
                Alignment::Auto => None,
            })
            .sign(match self.sign {
                true => Some(core::fmt::Sign::Plus),
                false => None,
            })
            .sign_aware_zero_pad(self.pad_zero)
            .alternate(self.alternate_form)
            .width(match self.width {
                Width::Dynamic(_) => None,
                Width::Fixed(amount) => Some(amount as u16),
            })
            .precision(match self.precision {
                Precision::Dynamic(_) | Precision::Auto => None,
                Precision::Fixed(amount) => Some(amount as u16),
            });

        options
    }

    /// Builder to specify the type.
    pub fn ty(mut self, ty: Type) -> Self {
        self.ty = ty;
        self
    }

    /// Builder to set the alternate form.
    pub fn alternate_form(mut self, alternate_form: bool) -> Self {
        self.alternate_form = alternate_form;
        self
    }

    /// Builder to specify the fill character
    pub fn fill_character(mut self, fill_character: char) -> Self {
        self.fill_character = fill_character;
        self
    }

    /// Builder to specify the alignment.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Builder to set the sign mode.
    pub fn sign(mut self, sign: bool) -> Self {
        self.sign = sign;
        self
    }

    /// Builder to set the pad zero mode.
    pub fn pad_zero(mut self, pad_zero: bool) -> Self {
        self.pad_zero = pad_zero;
        self
    }

    /// Builder to specify the width.
    pub fn width(mut self, width: Width) -> Self {
        self.width = width;
        self
    }

    /// Builder to specify the precision.
    pub fn precision(mut self, precision: Precision) -> Self {
        self.precision = precision;
        self
    }
}

impl<'a> Default for Specifier {
    fn default() -> Self {
        Self {
            ty: Type::Display,
            alternate_form: false,
            fill_character: ' ',
            alignment: Alignment::Auto,
            sign: false,
            pad_zero: false,
            width: Width::Fixed(0),
            precision: Precision::Auto,
        }
    }
}

impl core::fmt::Display for Specifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.fill_character)?;
        write!(f, "{}", self.alignment)?;
        if self.sign {
            f.write_char('+')?;
        }
        if self.alternate_form {
            f.write_char('#')?;
        }
        if self.pad_zero {
            f.write_char('0')?;
        }
        write!(f, "{}", self.width)?;
        write!(f, "{}", self.precision)?;
        write!(f, "{}", self.ty)?;

        Ok(())
    }
}
