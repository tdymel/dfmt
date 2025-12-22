use crate::{values::TypedArgumentKey, ArgumentKey};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ArgumentForTypeNotFound(TypedArgumentKey),
    ArgumentNotFound(ArgumentKey),
    DuplicateArgument(TypedArgumentKey),
    UnexpectedArgumentValue,
    Fmt(core::fmt::Error),
    UnexpectedToken,
}

impl core::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::ArgumentForTypeNotFound(typed_argument_key) => write!(
                f,
                "Argument value for key '{0:#?}' not found",
                typed_argument_key,
            ),
            Error::ArgumentNotFound(argument_key) => {
                write!(f, "Argument for key '{0:#?}' not found", argument_key)
            }
            Error::DuplicateArgument(typed_argument_key) => write!(
                f,
                "Duplicate argument value for key '{0:#?}'",
                typed_argument_key
            ),
            Error::UnexpectedArgumentValue => write!(f, "Unexpeced argument value"),
            Error::Fmt(error) => core::fmt::Display::fmt(&error, f),
            Error::UnexpectedToken => write!(f, "Unexpected token encountered while parsing"),
        }
    }
}
