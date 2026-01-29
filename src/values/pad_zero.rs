#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PadZero {
    Deactivated,
    Activated,
}

pub trait ToPadZero {
    fn to_pad_zero(self) -> PadZero;
}

impl ToPadZero for bool {
    fn to_pad_zero(self) -> PadZero {
        PadZero::of(self)
    }
}

impl ToPadZero for PadZero {
    fn to_pad_zero(self) -> PadZero {
        self
    }
}

impl PadZero {
    pub fn of(bool: bool) -> Self {
        match bool {
            false => PadZero::Deactivated,
            true => PadZero::Activated,
        }
    }
}
