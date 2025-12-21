use core::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};

use crate::{ArgumentTypeRequirements, values::Type};

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

    DynPointer(&'ct dyn DynPointer),

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
        let mut requirements = ArgumentTypeRequirements::default();
        match self {
            ArgumentValue::Display(_) => requirements.add_requirement(Type::Display),
            ArgumentValue::Debug(_) => requirements.add_requirement(Type::Debug),
            ArgumentValue::Binary(_) => requirements.add_requirement(Type::Binary),
            ArgumentValue::LowerExp(_) => requirements.add_requirement(Type::LowerExp),
            ArgumentValue::UpperExp(_) => requirements.add_requirement(Type::UpperExp),
            ArgumentValue::LowerHex(_) => requirements.add_requirement(Type::LowerHex),
            ArgumentValue::UpperHex(_) => requirements.add_requirement(Type::UpperHex),
            ArgumentValue::Octal(_) => requirements.add_requirement(Type::Octal),
            ArgumentValue::Pointer(_) => requirements.add_requirement(Type::Pointer),
            ArgumentValue::DynPointer(_) => requirements.add_requirement(Type::Pointer),
            ArgumentValue::DisplayAndDebug(_) => {
                requirements.add_requirement(Type::Display);
                requirements.add_requirement(Type::Debug);
            }
            ArgumentValue::IntegerLike(_) => {
                requirements.add_requirement(Type::Display);
                requirements.add_requirement(Type::Debug);
                requirements.add_requirement(Type::Binary);
                requirements.add_requirement(Type::Octal);
                requirements.add_requirement(Type::LowerExp);
                requirements.add_requirement(Type::UpperExp);
                requirements.add_requirement(Type::LowerHex);
                requirements.add_requirement(Type::UpperHex);
            }
            ArgumentValue::FloatLike(_) => {
                requirements.add_requirement(Type::Display);
                requirements.add_requirement(Type::Debug);
                requirements.add_requirement(Type::LowerExp);
                requirements.add_requirement(Type::UpperExp);
            }
        };
        requirements
    }
}

pub trait DynPointer: Pointer {
    fn dyn_fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}
impl<T> DynPointer for T where T: Pointer {
    fn dyn_fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
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
