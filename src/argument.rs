use core::fmt::{
    Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex,
};

use crate::{Error, template::Type};

pub enum ArgumentValue<'ct> {
    Display(&'ct dyn Display),
    Debug(&'ct dyn Debug),
    Binary(&'ct dyn Binary),
    LowerExp(&'ct dyn LowerExp),
    UpperExp(&'ct dyn UpperExp),
    LowerHex(&'ct dyn LowerHex),
    UpperHex(&'ct dyn UpperHex),
    Octal(&'ct dyn Octal),
    Pointer(&'ct dyn Pointer),

    DisplayAndDebug(&'ct dyn DisplayAndDebug),
    IntegerLike(&'ct dyn IntegerLike),
    FloatLike(&'ct dyn FloatLike),
}

impl<'ct> ArgumentValue<'ct> {
    pub fn to_u16(&self) -> Option<u16> {
        match self {
            ArgumentValue::Display(value) => value.to_string().parse::<u16>().ok(),
            ArgumentValue::DisplayAndDebug(value) => value.to_string().parse::<u16>().ok(),
            ArgumentValue::IntegerLike(value) => value.to_string().parse::<u16>().ok(),
            _ => None,
        }
    }

    pub fn fullfills(&self) -> ArgumentTypeRequirements {
        let requirements = ArgumentTypeRequirements::default();
        match self {
            ArgumentValue::Display(_) => requirements.with_display(),
            ArgumentValue::Debug(_) => requirements.with_debug(),
            ArgumentValue::Binary(_) => requirements.with_binary(),
            ArgumentValue::LowerExp(_) => requirements.with_lower_exp(),
            ArgumentValue::UpperExp(_) => requirements.with_upper_exp(),
            ArgumentValue::LowerHex(_) => requirements.with_lower_hex(),
            ArgumentValue::UpperHex(_) => requirements.with_upper_hex(),
            ArgumentValue::Octal(_) => requirements.with_octal(),
            ArgumentValue::Pointer(_) => requirements.with_pointer(),
            ArgumentValue::DisplayAndDebug(_) => requirements.with_display().with_debug(),
            ArgumentValue::IntegerLike(_) => requirements
                .with_display()
                .with_debug()
                .with_binary()
                .with_octal()
                .with_lower_exp()
                .with_upper_exp()
                .with_lower_hex()
                .with_upper_hex(),
            ArgumentValue::FloatLike(_) => requirements
                .with_display()
                .with_debug()
                .with_lower_exp()
                .with_upper_exp(),
        }
    }
}

pub struct TypedValue<'ct> {
    pub argument_value: &'ct ArgumentValue<'ct>,
    pub ty: Type,
}

impl<'ct> Display for TypedValue<'ct> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        match (self.argument_value, &self.ty) {
            (ArgumentValue::Display(value), Type::Display) => {
                core::fmt::Display::fmt(value, formatter)
            }
            (ArgumentValue::Debug(value), Type::Debug) => core::fmt::Debug::fmt(value, formatter),
            (ArgumentValue::Binary(value), Type::Binary) => {
                core::fmt::Binary::fmt(value, formatter)
            }
            (ArgumentValue::Octal(value), Type::Octal) => core::fmt::Octal::fmt(value, formatter),
            (ArgumentValue::Pointer(value), Type::Pointer) => {
                core::fmt::Pointer::fmt(value, formatter)
            }
            (ArgumentValue::LowerExp(value), Type::LowerExp) => {
                core::fmt::LowerExp::fmt(value, formatter)
            }
            (ArgumentValue::UpperExp(value), Type::UpperExp) => {
                core::fmt::UpperExp::fmt(value, formatter)
            }
            (ArgumentValue::LowerHex(value), Type::LowerHex) => {
                core::fmt::LowerHex::fmt(value, formatter)
            }
            (ArgumentValue::UpperHex(value), Type::UpperHex) => {
                core::fmt::UpperHex::fmt(value, formatter)
            }
            (ArgumentValue::DisplayAndDebug(value), Type::Display) => {
                core::fmt::Display::fmt(value, formatter)
            }
            (ArgumentValue::DisplayAndDebug(value), Type::Debug) => {
                core::fmt::Debug::fmt(value, formatter)
            }
            (ArgumentValue::IntegerLike(value), Type::Display) => {
                core::fmt::Display::fmt(value, formatter)
            }
            (ArgumentValue::IntegerLike(value), Type::Debug) => {
                core::fmt::Debug::fmt(value, formatter)
            }
            (ArgumentValue::IntegerLike(value), Type::Octal) => {
                core::fmt::Octal::fmt(value, formatter)
            }
            (ArgumentValue::IntegerLike(value), Type::Binary) => {
                core::fmt::Binary::fmt(value, formatter)
            }
            (ArgumentValue::IntegerLike(value), Type::LowerExp) => {
                core::fmt::LowerExp::fmt(value, formatter)
            }
            (ArgumentValue::IntegerLike(value), Type::UpperExp) => {
                core::fmt::UpperExp::fmt(value, formatter)
            }
            (ArgumentValue::IntegerLike(value), Type::LowerHex) => {
                core::fmt::LowerHex::fmt(value, formatter)
            }
            (ArgumentValue::IntegerLike(value), Type::UpperHex) => {
                core::fmt::UpperHex::fmt(value, formatter)
            }
            (ArgumentValue::FloatLike(value), Type::Display) => {
                core::fmt::Display::fmt(value, formatter)
            }
            (ArgumentValue::FloatLike(value), Type::Debug) => {
                core::fmt::Debug::fmt(value, formatter)
            }
            (ArgumentValue::FloatLike(value), Type::LowerExp) => {
                core::fmt::LowerExp::fmt(value, formatter)
            }
            (ArgumentValue::FloatLike(value), Type::UpperExp) => {
                core::fmt::UpperExp::fmt(value, formatter)
            }
            _ => unreachable!(),
        }
    }
}

pub trait DisplayAndDebug: Display + Debug {}
impl<T> DisplayAndDebug for T where T: Display + Debug {}
pub trait IntegerLike:
    Display + Debug + LowerExp + UpperExp + LowerHex + UpperHex + Binary + Octal
{
}
impl<T> IntegerLike for T where
    T: Display + Debug + LowerExp + UpperExp + LowerHex + UpperHex + Binary + Octal
{
}

pub trait FloatLike: Display + Debug + LowerExp + UpperExp {}
impl<T> FloatLike for T where T: Display + Debug + LowerExp + UpperExp {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentKey {
    Index(usize),
    Name(String),
}

impl Display for ArgumentKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ArgumentKey::Index(index) => write!(f, "{index}"),
            ArgumentKey::Name(name) => write!(f, "{name}"),
        }
    }
}

pub trait ToArgumentKey {
    fn to_argument_key(&self) -> ArgumentKey;
}

impl ToArgumentKey for usize {
    fn to_argument_key(&self) -> ArgumentKey {
        ArgumentKey::Index(*self)
    }
}

impl ToArgumentKey for &str {
    fn to_argument_key(&self) -> ArgumentKey {
        ArgumentKey::Name(self.to_string())
    }
}

impl ToArgumentKey for ArgumentKey {
    fn to_argument_key(&self) -> ArgumentKey {
        self.clone()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ArgumentTypeRequirements {
    pub display: bool,
    pub debug: bool,
    pub lower_exp: bool,
    pub upper_exp: bool,
    pub lower_hex: bool,
    pub upper_hex: bool,
    pub binary: bool,
    pub pointer: bool,
    pub octal: bool,
}

impl Default for ArgumentTypeRequirements {
    fn default() -> Self {
        Self {
            display: Default::default(),
            debug: Default::default(),
            lower_exp: Default::default(),
            upper_exp: Default::default(),
            lower_hex: Default::default(),
            upper_hex: Default::default(),
            binary: Default::default(),
            pointer: Default::default(),
            octal: Default::default(),
        }
    }
}

impl ArgumentTypeRequirements {
    pub fn with_display(mut self) -> Self {
        self.display = true;
        self
    }

    pub fn with_debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn with_lower_exp(mut self) -> Self {
        self.lower_exp = true;
        self
    }

    pub fn with_upper_exp(mut self) -> Self {
        self.upper_exp = true;
        self
    }

    pub fn with_lower_hex(mut self) -> Self {
        self.lower_hex = true;
        self
    }

    pub fn with_upper_hex(mut self) -> Self {
        self.upper_hex = true;
        self
    }

    pub fn with_binary(mut self) -> Self {
        self.binary = true;
        self
    }

    pub fn with_pointer(mut self) -> Self {
        self.pointer = true;
        self
    }

    pub fn with_octal(mut self) -> Self {
        self.octal = true;
        self
    }

    pub fn requires(&self, other: &Self) -> Result<(), Error> {
        if (!other.display || self.display)
            && (!other.debug || self.debug)
            && (!other.lower_exp || self.lower_exp)
            && (!other.upper_exp || self.upper_exp)
            && (!other.lower_hex || self.lower_hex)
            && (!other.upper_hex || self.upper_hex)
            && (!other.binary || self.binary)
            && (!other.pointer || self.pointer)
            && (!other.octal || self.octal)
        {
            return Ok(());
        }
        Err(Error::ArgumentDoesNotMeetRequirements(*other, *self))
    }
}
