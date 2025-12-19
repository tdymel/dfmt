
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentKey {
    Index(usize),
    Name(String),
}

impl core::fmt::Display for ArgumentKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
