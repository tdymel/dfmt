use crate::{Alignment, Precision, Specifier, Type, Width};

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// Helper structure to specify the constraints of an expected argument.
/// ```rust
/// use dfmt::*;
///
/// let allowed_specifier =
///     AllowedSpecifier::all()
///         .forbid(Type::Display);
///
/// assert!(Template::parse("Hello, {world}!").unwrap()
///     .expect_argument("world", &allowed_specifier)
///     .is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedSpecifier {
    pub types: [bool; 10],
    pub alternate_form: [bool; 2],
    pub fill_characters: Option<String>,
    pub alignments: [bool; 3],
    pub sign: [bool; 2],
    pub pad_zero: [bool; 2],
    pub widths: Option<Vec<Width>>,
    pub precisions: Option<Vec<Precision>>,
}

impl AllowedSpecifier {
    pub fn all() -> Self {
        AllowedSpecifier {
            types: [true; 10],
            alternate_form: [true; 2],
            fill_characters: None,
            alignments: [true; 3],
            sign: [true; 2],
            pad_zero: [true; 2],
            widths: None,
            precisions: None,
        }
    }

    pub fn none() -> Self {
        AllowedSpecifier {
            types: [false; 10],
            alternate_form: [false; 2],
            fill_characters: Some("".to_string()),
            alignments: [false; 3],
            sign: [false; 2],
            pad_zero: [false; 2],
            widths: Some(Vec::new()),
            precisions: Some(Vec::new()),
        }
    }

    pub fn is_within_constraints(&self, specifier: &Specifier) -> bool {
        self.types[specifier.ty as usize]
            && ((!specifier.alternate_form && self.alternate_form[0])
                || (specifier.alternate_form && self.alternate_form[1]))
            && self
                .fill_characters
                .as_ref()
                .map(|fc| fc.contains(specifier.fill_character))
                .unwrap_or(true)
            && self.alignments[specifier.alignment as usize]
            && ((!specifier.sign && self.sign[0]) || (specifier.sign && self.sign[1]))
            && ((!specifier.pad_zero && self.pad_zero[0])
                || (specifier.pad_zero && self.pad_zero[1]))
            && self
                .widths
                .as_ref()
                .map(|widths| widths.contains(&specifier.width))
                .unwrap_or(true)
            && self
                .precisions
                .as_ref()
                .map(|precisions| precisions.contains(&specifier.precision))
                .unwrap_or(true)
    }
}

pub trait AllowedSpecifierBuilder<T> {
    fn allow(self, constraint: T) -> Self;
    fn forbid(self, constraint: T) -> Self;
}

impl AllowedSpecifierBuilder<Type> for AllowedSpecifier {
    fn allow(mut self, constraint: Type) -> Self {
        self.types[constraint as usize] = true;
        self
    }

    fn forbid(mut self, constraint: Type) -> Self {
        self.types[constraint as usize] = false;
        self
    }
}

impl AllowedSpecifierBuilder<Alignment> for AllowedSpecifier {
    fn allow(mut self, constraint: Alignment) -> Self {
        self.alignments[constraint as usize] = true;
        self
    }

    fn forbid(mut self, constraint: Alignment) -> Self {
        self.alignments[constraint as usize] = true;
        self
    }
}
