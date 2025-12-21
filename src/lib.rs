#![cfg_attr(feature = "nightly_formatting_options", feature(formatting_options))]

/*
TODO:
- Write documentation with examples
- Write Tests (Also test non ascii templates)
- Write Benchmarks

- Finalize Readme
- Fix Contributing

- Create CI/CD
- Publish crate

- The specifier Parser is not very robust yet

- Find out where the dformat_unchecked overhead comes from

# Long Shots
- Implement and test no_std support
- Find out how to write an api like format([k,v]) where k is any key and v is any value without wrapping it in ArgumentValue
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
