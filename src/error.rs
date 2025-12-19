use thiserror::Error;

use crate::{ArgumentKey, ArgumentTypeRequirements};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Argument value for key '{0:#?}' not found")]
    ArgumentNotFound(ArgumentKey),
    #[error("Duplicate argument value for key '{0:#?}'")]
    DuplicateArgument(ArgumentKey),
    #[error("Argument does not meet requirements: Needed: {0:#?}; Given: {0:#?}")]
    ArgumentDoesNotMeetRequirements(ArgumentTypeRequirements, ArgumentTypeRequirements),
    #[error("Unexpeced argument value")]
    UnexpectedArgumentValue,
    #[error("Failed to format value")]
    Fmt(#[from] core::fmt::Error),
    #[error("Unexpected token encountered while parsing")]
    UnexpectedToken,
}
