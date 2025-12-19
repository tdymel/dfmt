use core::fmt::Write;

use crate::{Arguments, argument::ArgumentKey, error::Error, parser::parse_pieces};

#[derive(Debug, Clone)]
pub struct Template {
    pub template: String,
    pub pieces: Vec<Piece>,
}

impl Template {
    pub fn parse(template: String) -> Result<Self, Error> {
        let pieces = parse_pieces(&template)?;
        Ok(Self { template, pieces })
    }

    pub fn parse_str(template: &str) -> Result<Self, Error> {
        Template::parse(template.to_owned())
    }

    pub fn arguments(&self) -> Arguments<'_> {
        Arguments::new(self)
    }

    pub fn to_template(&self) -> &Template {
        self
    }
}

impl core::fmt::Display for Template {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for piece in &self.pieces {
            match piece {
                Piece::Literal { start, end } => f.write_str(&self.template[*start..*end])?,
                Piece::BracketOpen => f.write_str("{{")?,
                Piece::BracketClose => f.write_str("}}")?,
                Piece::Argument { key, specifier } => {
                    f.write_char('{')?;
                    write!(f, "{key}")?;
                    if let Some(specifier) = specifier {
                        f.write_char(':')?;
                        write!(f, "{specifier}")?;
                    }
                    f.write_char('}')?;
                }
            };
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Piece {
    Literal {
        start: usize,
        end: usize,
    },
    BracketOpen,
    BracketClose,
    Argument {
        key: ArgumentKey,
        specifier: Option<Specifier>,
    },
}

pub trait ToTemplate {
    fn to_template(self) -> Template;
}

impl ToTemplate for Template {
    fn to_template(self) -> Template {
        self
    }
}

impl ToTemplate for &str {
    fn to_template(self) -> Template {
        Template::parse_str(self).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Binary,
    Octal,
    LowerHex,
    UpperHex,
    Pointer,
    LowerExp,
    UpperExp,
    Debug,
    Display,
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Binary => f.write_char('b'),
            Type::Octal => f.write_char('o'),
            Type::LowerHex => f.write_char('x'),
            Type::UpperHex => f.write_char('X'),
            Type::Pointer => f.write_char('p'),
            Type::LowerExp => f.write_char('e'),
            Type::UpperExp => f.write_char('E'),
            Type::Debug => f.write_char('?'),
            Type::Display => Ok(()),
        }
    }
}

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

#[derive(Debug, Clone)]
pub enum Width {
    Dynamic(ArgumentKey), // something$
    Fixed(u16),
}

impl core::fmt::Display for Width {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Width::Dynamic(argument_key) => {
                write!(f, "{argument_key}")?;
                f.write_char('$')
            }
            Width::Fixed(amount) => write!(f, "{amount}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Precision {
    Auto,
    Dynamic(ArgumentKey), // .something$ or *
    Fixed(u16),
}

impl core::fmt::Display for Precision {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Precision::Dynamic(argument_key) => {
                f.write_char('.')?;
                write!(f, "{argument_key}")?;
                f.write_char('$')
            }
            Precision::Fixed(amount) => {
                f.write_char('.')?;
                write!(f, "{amount}")
            }
            Precision::Auto => Ok(()),
        }
    }
}

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
