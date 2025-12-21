use core::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    WidthOrPrecisionAmount,
}

impl core::fmt::Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Type::Binary => f.write_char('b'),
            Type::Octal => f.write_char('o'),
            Type::LowerHex => f.write_char('x'),
            Type::UpperHex => f.write_char('X'),
            Type::Pointer => f.write_char('p'),
            Type::LowerExp => f.write_char('e'),
            Type::UpperExp => f.write_char('E'),
            Type::Debug => f.write_char('?'),
            Type::Display | Type::WidthOrPrecisionAmount => Ok(()),
        }
    }
}
