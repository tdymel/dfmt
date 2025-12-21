use thiserror::Error;

use crate::{ArgumentKey, values::TypedArgumentKey};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Argument value for key '{0:#?}' not found")]
    ArgumentForTypeNotFound(TypedArgumentKey),
    #[error("Argument for key '{0:#?}' not found")]
    ArgumentNotFound(ArgumentKey),
    #[error("Duplicate argument value for key '{0:#?}'")]
    DuplicateArgument(TypedArgumentKey),
    #[error("Unexpeced argument value")]
    UnexpectedArgumentValue,
    #[error("Failed to format value")]
    Fmt(#[from] core::fmt::Error),
    #[error("Unexpected token encountered while parsing")]
    UnexpectedToken,
}
