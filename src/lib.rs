#![cfg_attr(feature = "nightly_formatting_options", feature(formatting_options))]
#![doc = include_str!("../README.md")]

/*
TODO:
- Write documentation with examples

- Create CI/CD
- Publish crate

- Test error cases
*/

mod error;
mod macros;
mod materials;
mod values;

pub use error::Error;
pub use materials::{
    Arguments, ArgumentsBuilder, DynFmt, Template, ToTemplate, UncheckedArgumentsBuilder,
};
pub use values::{
    ArgumentKey, ArgumentTypeRequirements, ArgumentValue, ToArgumentKey, WidthOrPrecisionAmount,
};
