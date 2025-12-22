#![cfg_attr(feature = "nightly_formatting_options", feature(formatting_options))]

/*
TODO:
- Write documentation with examples

- Create CI/CD
- Publish crate

- Find out where the dformat_unchecked overhead comes from
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
