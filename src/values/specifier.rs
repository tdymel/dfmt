use core::fmt::Write;

use crate::values::{Alignment, Precision, Type, Width};

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

#[cfg(feature = "nightly_formatting_options")]
impl Specifier {
    pub fn formatting_options(&self) -> core::fmt::FormattingOptions {
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
