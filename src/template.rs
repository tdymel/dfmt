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
        todo!()
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

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub enum Width {
    Dynamic(ArgumentKey), // something$
    Fixed(usize),
}

#[derive(Debug, Clone)]
pub enum Precision {
    Dynamic(ArgumentKey), // .something$ or *
    Fixed(usize),
}

#[derive(Debug, Clone)]
pub struct Specifier {
    pub ty: Type,
    pub alternate_form: bool,
    pub fill_character: Option<char>, // Default: Space
    pub alignment: Option<Alignment>,
    pub sign: bool,
    pub pad_zero: bool,
    pub width: Width,
    pub precision: Option<Precision>,
}

#[cfg(feature = "nightly_formatting_options")]
impl Specifier {
    pub fn formatting_options(&self) -> core::fmt::FormattingOptions {
        let mut options = core::fmt::FormattingOptions::new();
        options
            .fill(self.fill_character.unwrap_or(' '))
            .align(self.alignment.map(|it| match it {
                Alignment::Left => core::fmt::Alignment::Left,
                Alignment::Right => core::fmt::Alignment::Right,
                Alignment::Center => core::fmt::Alignment::Center,
            }))
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
                Some(Precision::Dynamic(_)) | None => None,
                Some(Precision::Fixed(amount)) => Some(amount as u16),
            });

        options
    }
}

impl<'a> Default for Specifier {
    fn default() -> Self {
        Self {
            ty: Type::Display,
            alternate_form: Default::default(),
            fill_character: Default::default(),
            alignment: Default::default(),
            sign: Default::default(),
            pad_zero: Default::default(),
            width: Width::Fixed(0),
            precision: Default::default(),
        }
    }
}
