use core::fmt::Write;

use crate::{
    values::{Alignment, Precision, Type, Width},
    ArgumentKey, Error,
};

#[cfg(not(feature = "std"))]
use alloc::string::ToString;

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
    pub ty: Type,
    pub alternate_form: bool,
    pub fill_character: char,
    pub alignment: Alignment,
    pub sign: bool,
    pub pad_zero: bool,
    pub width: Width,
    pub precision: Precision,
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
        let mut specifier = Specifier::default();

        if input_len == 0 {
            return Ok(specifier);
        }

        if let Some(fill_character) = parse_fill_character(&chars[current_specifier_index..]) {
            current_specifier_index += 1;
            specifier.fill_character = fill_character;
        }

        // Check if len > 0 ?
        if let Some(alignment) = parse_alignment(&chars[current_specifier_index..]) {
            current_specifier_index += 1;
            specifier.alignment = alignment;
        }

        if let Some(sign) = parse_sign(&chars[current_specifier_index..]) {
            current_specifier_index += 1;
            specifier.sign = sign;
        }

        if let Some(alternate_form) = parse_alternate_form(&chars[current_specifier_index..]) {
            current_specifier_index += 1;
            specifier.alternate_form = alternate_form;
        }

        if let Some(pad_zero) = parse_pad_zero(&chars[current_specifier_index..]) {
            current_specifier_index += 1;
            specifier.pad_zero = pad_zero;
        }

        if let Some((width, incr_index)) = parse_width(
            &chars[current_specifier_index..],
            &input[current_specifier_index..],
        ) {
            current_specifier_index += incr_index;
            specifier.width = width;
        }

        if let Some((precision, incr_index)) = parse_precision(
            &chars[current_specifier_index..],
            &input[current_specifier_index..],
            internal_index,
        ) {
            current_specifier_index += incr_index;
            specifier.precision = precision;
        }

        if let Some(ty) = parse_ty(&chars[current_specifier_index..]) {
            current_specifier_index += 1;
            specifier.ty = ty;
        }

        if current_specifier_index < input.len() {
            Err(Error::UnexpectedToken)
        } else {
            Ok(specifier)
        }
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

impl Default for Specifier {
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

fn parse_fill_character(chars: &[u8]) -> Option<char> {
    match (chars.first(), chars.get(1)) {
        (Some(chr), Some(b'<') | Some(b'^') | Some(b'>')) => Some(*chr as char),
        _ => None,
    }
}

fn parse_alignment(chars: &[u8]) -> Option<Alignment> {
    match chars.first() {
        Some(b'<') => Some(Alignment::Left),
        Some(b'^') => Some(Alignment::Center),
        Some(b'>') => Some(Alignment::Right),
        _ => None,
    }
}

fn parse_sign(chars: &[u8]) -> Option<bool> {
    match chars.first() {
        Some(b'+') => Some(true),
        Some(b'-') => Some(false),
        _ => None,
    }
}

fn parse_alternate_form(chars: &[u8]) -> Option<bool> {
    match chars.first() {
        Some(b'#') => Some(true),
        _ => None,
    }
}

fn parse_pad_zero(chars: &[u8]) -> Option<bool> {
    match chars.first() {
        Some(b'0') => Some(true),
        _ => None,
    }
}

fn parse_width(chars: &[u8], input: &str) -> Option<(Width, usize)> {
    match chars.first() {
        Some(b'.') => None,
        Some(&chr) => {
            if (chr as char).is_ascii_digit() {
                let mut until_index = 0;
                while let Some(&add_char) = chars.get(until_index + 1) {
                    if !add_char.is_ascii_digit() {
                        break;
                    }
                    until_index += 1;
                }
                until_index += 1;
                Some((
                    Width::Fixed(input[..until_index].parse::<u16>().unwrap()),
                    until_index,
                ))
            } else {
                input.find('$').map(|var_index| {
                    (
                        Width::Dynamic(ArgumentKey::Name(input[..var_index].to_string())),
                        var_index + 1,
                    )
                })
            }
        }
        None => None,
    }
}

fn parse_precision(
    chars: &[u8],
    input: &str,
    internal_index: &mut usize,
) -> Option<(Precision, usize)> {
    match chars.first() {
        Some(b'.') => match chars.get(1) {
            Some(b'*') => {
                *internal_index += 1;
                Some((
                    Precision::Dynamic(ArgumentKey::Index(*internal_index - 1)),
                    2,
                ))
            }
            Some(&chr) => {
                if (chr as char).is_ascii_digit() {
                    let mut until_index = 1;
                    while let Some(&add_char) = chars.get(until_index + 1) {
                        if !add_char.is_ascii_digit() {
                            break;
                        }
                        until_index += 1;
                    }
                    until_index += 1;
                    Some((
                        Precision::Fixed(input[1..until_index].parse::<u16>().unwrap()),
                        until_index,
                    ))
                } else {
                    input.find('$').map(|var_index| {
                        (
                            Precision::Dynamic(ArgumentKey::Name(input[1..var_index].to_string())),
                            var_index + 1,
                        )
                    })
                }
            }
            None => None,
        },
        _ => None,
    }
}

fn parse_ty(chars: &[u8]) -> Option<Type> {
    match chars.first() {
        Some(b'?') => Some(Type::Debug),
        Some(b'b') => Some(Type::Binary),
        Some(b'o') => Some(Type::Octal),
        Some(b'e') => Some(Type::LowerExp),
        Some(b'E') => Some(Type::UpperExp),
        Some(b'x') => Some(Type::LowerHex),
        Some(b'X') => Some(Type::UpperHex),
        Some(b'p') => Some(Type::Pointer),
        _ => None,
    }
}
