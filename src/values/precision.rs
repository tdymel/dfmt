use core::fmt::Write;

use crate::ArgumentKey;

/// Precision variants of the specifier.
#[derive(Debug, Clone)]
pub enum Precision {
    Auto,
    Dynamic(ArgumentKey),
    Fixed(u16),
}

impl core::fmt::Display for Precision {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Precision::Dynamic(argument_key) => {
                f.write_char('.')?;
                write!(f, "{}", argument_key)?;
                f.write_char('$')
            }
            Precision::Fixed(amount) => {
                f.write_char('.')?;
                write!(f, "{}", amount)
            }
            Precision::Auto => Ok(()),
        }
    }
}
