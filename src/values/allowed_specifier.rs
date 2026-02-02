use crate::{values::Sign, Alignment, AlternateForm, PadZero, Precision, Specifier, Type, Width};

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec, vec::Vec};

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
    pub types: u16,
    pub alternate_forms: u8,
    pub allowed_fill_characters: Option<String>,
    pub forbidden_fill_characters: Option<String>,
    pub alignments: u8,
    pub signs: u8,
    pub pad_zeros: u8,
    pub allowed_widths: Option<Vec<Width>>,
    pub forbidden_widths: Option<Vec<Width>>,
    pub allowed_precisions: Option<Vec<Precision>>,
    pub forbidden_precisions: Option<Vec<Precision>>,
}

impl AllowedSpecifier {
    pub fn all() -> Self {
        AllowedSpecifier {
            types: 0b0000_0011_1111_1111,
            alternate_forms: 0b0000_0011,
            allowed_fill_characters: None,
            forbidden_fill_characters: None,
            alignments: 0b0000_1111,
            signs: 0b0000_0011,
            pad_zeros: 0b0000_0011,
            allowed_widths: None,
            forbidden_widths: None,
            allowed_precisions: None,
            forbidden_precisions: None,
        }
    }

    pub fn none() -> Self {
        AllowedSpecifier {
            types: 0,
            alternate_forms: 0,
            allowed_fill_characters: Some(String::new()),
            forbidden_fill_characters: None,
            alignments: 0,
            signs: 0,
            pad_zeros: 0,
            allowed_widths: Some(Vec::new()),
            forbidden_widths: None,
            allowed_precisions: Some(Vec::new()),
            forbidden_precisions: None,
        }
    }

    pub fn is_within_constraints(&self, specifier: &Specifier) -> bool {
        (self.types & (1 << (specifier.ty as usize))) != 0
            && (self.alternate_forms & (1 << (specifier.alternate_form.clone() as usize))) != 0
            && self
                .allowed_fill_characters
                .as_ref()
                .map(|fc| fc.contains(specifier.fill_character))
                .unwrap_or(true)
            && self
                .forbidden_fill_characters
                .as_ref()
                .map(|fc| !fc.contains(specifier.fill_character))
                .unwrap_or(true)
            && (self.alignments & (1 << (specifier.alignment as usize))) != 0
            && (self.signs & (1 << (specifier.sign.clone() as usize))) != 0
            && (self.pad_zeros & (1 << (specifier.pad_zero.clone() as usize))) != 0
            && self
                .allowed_widths
                .as_ref()
                .map(|widths| widths.contains(&specifier.width))
                .unwrap_or(true)
            && self
                .forbidden_widths
                .as_ref()
                .map(|widths| !widths.contains(&specifier.width))
                .unwrap_or(true)
            && self
                .allowed_precisions
                .as_ref()
                .map(|precisions| precisions.contains(&specifier.precision))
                .unwrap_or(true)
            && self
                .forbidden_precisions
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
        self.types |= 1 << constraint as usize;
        self
    }

    fn forbid(mut self, constraint: Type) -> Self {
        self.types &= !(1 << constraint as u16);
        self
    }
}

impl AllowedSpecifierBuilder<Alignment> for AllowedSpecifier {
    fn allow(mut self, constraint: Alignment) -> Self {
        self.alignments |= 1 << constraint as usize;
        self
    }

    fn forbid(mut self, constraint: Alignment) -> Self {
        self.alignments &= !(1 << constraint as usize);
        self
    }
}

impl AllowedSpecifierBuilder<Option<String>> for AllowedSpecifier {
    fn allow(mut self, constraint: Option<String>) -> Self {
        self.allowed_fill_characters = constraint;
        self
    }

    fn forbid(mut self, constraint: Option<String>) -> Self {
        self.forbidden_fill_characters = constraint;
        self
    }
}

impl AllowedSpecifierBuilder<Option<Vec<Width>>> for AllowedSpecifier {
    fn allow(mut self, constraint: Option<Vec<Width>>) -> Self {
        self.allowed_widths = constraint;
        self
    }

    fn forbid(mut self, constraint: Option<Vec<Width>>) -> Self {
        self.forbidden_widths = constraint;
        self
    }
}

impl AllowedSpecifierBuilder<Width> for AllowedSpecifier {
    fn allow(mut self, constraint: Width) -> Self {
        if let Some(widths) = &mut self.forbidden_widths {
            if let Some(index) = widths.iter().position(|width| width == &constraint) {
                widths.remove(index);
            }
        }

        if self.allowed_widths.is_none() {
            self.allowed_widths = Some(vec![constraint]);
        } else if let Some(widths) = &mut self.allowed_widths {
            widths.push(constraint);
        }

        self
    }

    fn forbid(mut self, constraint: Width) -> Self {
        if let Some(widths) = &mut self.allowed_widths {
            if let Some(index) = widths.iter().position(|width| width == &constraint) {
                widths.remove(index);
            }
        }

        if self.forbidden_widths.is_none() {
            self.forbidden_widths = Some(vec![constraint]);
        } else if let Some(widths) = &mut self.forbidden_widths {
            widths.push(constraint);
        }

        self
    }
}

impl AllowedSpecifierBuilder<Option<Vec<Precision>>> for AllowedSpecifier {
    fn allow(mut self, constraint: Option<Vec<Precision>>) -> Self {
        self.allowed_precisions = constraint;
        self
    }

    fn forbid(mut self, constraint: Option<Vec<Precision>>) -> Self {
        self.forbidden_precisions = constraint;
        self
    }
}

impl AllowedSpecifierBuilder<Precision> for AllowedSpecifier {
    fn allow(mut self, constraint: Precision) -> Self {
        if let Some(precisions) = &mut self.forbidden_precisions {
            if let Some(index) = precisions
                .iter()
                .position(|precision| precision == &constraint)
            {
                precisions.remove(index);
            }
        }

        if self.allowed_precisions.is_none() {
            self.allowed_precisions = Some(vec![constraint]);
        } else if let Some(precisions) = &mut self.allowed_precisions {
            precisions.push(constraint);
        }

        self
    }

    fn forbid(mut self, constraint: Precision) -> Self {
        if let Some(precisions) = &mut self.allowed_precisions {
            if let Some(index) = precisions
                .iter()
                .position(|precision| precision == &constraint)
            {
                precisions.remove(index);
            }
        }

        if self.forbidden_precisions.is_none() {
            self.forbidden_precisions = Some(vec![constraint]);
        } else if let Some(precisions) = &mut self.forbidden_precisions {
            precisions.push(constraint);
        }

        self
    }
}

impl AllowedSpecifierBuilder<Sign> for AllowedSpecifier {
    fn allow(mut self, constraint: Sign) -> Self {
        self.signs |= 1 << constraint as usize;
        self
    }

    fn forbid(mut self, constraint: Sign) -> Self {
        self.signs &= !(1 << constraint as usize);
        self
    }
}

impl AllowedSpecifierBuilder<PadZero> for AllowedSpecifier {
    fn allow(mut self, constraint: PadZero) -> Self {
        self.pad_zeros |= 1 << constraint as usize;
        self
    }

    fn forbid(mut self, constraint: PadZero) -> Self {
        self.pad_zeros &= !(1 << constraint as usize);
        self
    }
}

impl AllowedSpecifierBuilder<AlternateForm> for AllowedSpecifier {
    fn allow(mut self, constraint: AlternateForm) -> Self {
        self.alternate_forms |= 1 << constraint as usize;
        self
    }

    fn forbid(mut self, constraint: AlternateForm) -> Self {
        self.alternate_forms &= !(1 << constraint as usize);
        self
    }
}
