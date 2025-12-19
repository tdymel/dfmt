use core::fmt::Write;

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