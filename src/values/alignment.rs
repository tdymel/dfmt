
use core::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Auto,
}

impl core::fmt::Display for Alignment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Alignment::Left => f.write_char('<'),
            Alignment::Center => f.write_char('^'),
            Alignment::Right => f.write_char('>'),
            Alignment::Auto => Ok(()),
        }
    }
}