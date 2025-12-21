use crate::values::Type;

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
    pub width_or_precision_amount: bool,
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
            width_or_precision_amount: Default::default(),
        }
    }
}

impl ArgumentTypeRequirements {
    pub fn add_requirement(&mut self, ty: Type) {
        match ty {
            Type::Binary => self.binary = true,
            Type::Octal => self.octal = true,
            Type::LowerHex => self.lower_hex = true,
            Type::UpperHex => self.upper_hex = true,
            Type::Pointer => self.pointer = true,
            Type::LowerExp => self.lower_exp = true,
            Type::UpperExp => self.upper_exp = true,
            Type::Debug => self.debug = true,
            Type::Display => self.display = true,
            Type::WidthOrPrecisionAmount => self.width_or_precision_amount = true,
        };
    }
}
