#![cfg_attr(feature = "nightly_formatting_options", feature(formatting_options))]

/*
TODO:
- Write documentation with examples
- Write Tests (Also test non ascii templates)
- Write Benchmarks
- Implement and test no_std support

- Impl Parser as impl of the individual values

- Add an API with the specialization feature
- Better handling of dynamic precision and width

- Finalize Readme
- Fix Contributing

- Create CI/CD
- Publish crate
*/

mod error;
mod macros;
mod materials;
mod parser;
mod values;

pub use error::Error;
pub use materials::{
    Arguments, ArgumentsBuilder, DynFmt, Template, ToTemplate, UncheckedArgumentsBuilder,
};
pub use values::{ArgumentKey, ArgumentTypeRequirements, ArgumentValue, ToArgumentKey};
