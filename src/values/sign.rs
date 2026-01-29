#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    None,
    Plus,
}

pub trait ToSign {
    fn to_sign(self) -> Sign;
}

impl ToSign for bool {
    fn to_sign(self) -> Sign {
        Sign::of(self)
    }
}

impl ToSign for Sign {
    fn to_sign(self) -> Sign {
        self
    }
}

impl Sign {
    pub fn of(bool: bool) -> Self {
        match bool {
            false => Sign::None,
            true => Sign::Plus,
        }
    }
}
