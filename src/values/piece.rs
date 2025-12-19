use core::fmt::Write;

use crate::{ArgumentKey, values::Specifier};

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

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
