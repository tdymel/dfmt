use crate::{ArgumentValue, values::Type};

pub struct TypedValue<'ct> {
    pub argument_value: &'ct ArgumentValue<'ct>,
    pub ty: Type,
}

impl<'ct> core::fmt::Display for TypedValue<'ct> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
