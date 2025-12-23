use core::fmt::Write;

use crate::ArgumentKey;

/// Width variants of the specifier.
#[derive(Debug, Clone)]
pub enum Width {
    Dynamic(ArgumentKey),
    Fixed(u16),
}

impl core::fmt::Display for Width {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Width::Dynamic(argument_key) => {
                write!(f, "{}", argument_key)?;
                f.write_char('$')
            }
            Width::Fixed(amount) => write!(f, "{}", amount),
        }
    }
}
