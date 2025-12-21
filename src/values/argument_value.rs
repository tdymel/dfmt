use core::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};

use crate::values::Type;

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
    WidthOrPrecisionAmount(&'ct dyn WidthOrPrecisionAmount),
}

impl<'ct> ArgumentValue<'ct> {
    pub fn to_u16(&self) -> Option<u16> {
        match self {
            ArgumentValue::WidthOrPrecisionAmount(value) => Some(value.to_u16()),
            _ => None,
        }
    }

    pub fn to_type(&self) -> Type {
        match self {
            ArgumentValue::Display(_) => Type::Display,
            ArgumentValue::Debug(_) => Type::Debug,
            ArgumentValue::Binary(_) => Type::Binary,
            ArgumentValue::LowerExp(_) => Type::LowerExp,
            ArgumentValue::UpperExp(_) => Type::UpperExp,
            ArgumentValue::LowerHex(_) => Type::LowerHex,
            ArgumentValue::UpperHex(_) => Type::UpperHex,
            ArgumentValue::Octal(_) => Type::Octal,
            ArgumentValue::Pointer(_) => Type::Pointer,
            ArgumentValue::WidthOrPrecisionAmount(_) => Type::WidthOrPrecisionAmount,
        }
    }
}

impl<'ct> core::fmt::Display for ArgumentValue<'ct> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ArgumentValue::Display(display) => (*display).fmt(f),
            ArgumentValue::Debug(debug) => (*debug).fmt(f),
            ArgumentValue::Binary(binary) => (*binary).fmt(f),
            ArgumentValue::LowerExp(lower_exp) => (*lower_exp).fmt(f),
            ArgumentValue::UpperExp(upper_exp) => (*upper_exp).fmt(f),
            ArgumentValue::LowerHex(lower_hex) => (*lower_hex).fmt(f),
            ArgumentValue::UpperHex(upper_hex) => (*upper_hex).fmt(f),
            ArgumentValue::Octal(octal) => (*octal).fmt(f),
            ArgumentValue::Pointer(pointer) => (*pointer).fmt(f),
            ArgumentValue::WidthOrPrecisionAmount(_) => {
                unreachable!()
            }
        }
    }
}

pub trait WidthOrPrecisionAmount {
    fn to_u16(&self) -> u16;
}

macro_rules! impl_width_or_precision_amount {
    (true, $ty:ty) => {
        impl WidthOrPrecisionAmount for $ty {
            fn to_u16(&self) -> u16 {
                *self as u16
            }
        }
    };
    (false, $ty:ty) => {
        impl WidthOrPrecisionAmount for $ty {
            fn to_u16(&self) -> u16 {
                if *self < 0 { 0 } else { *self as u16 }
            }
        }
    };
}

impl_width_or_precision_amount!(false, i8);
impl_width_or_precision_amount!(true, u8);
impl_width_or_precision_amount!(false, i16);
impl_width_or_precision_amount!(true, u16);
impl_width_or_precision_amount!(false, i32);
impl_width_or_precision_amount!(true, u32);
impl_width_or_precision_amount!(false, i64);
impl_width_or_precision_amount!(true, u64);
impl_width_or_precision_amount!(false, i128);
impl_width_or_precision_amount!(true, u128);
