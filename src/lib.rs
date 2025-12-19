#![cfg_attr(feature = "nightly_formatting_options", feature(formatting_options))]

/*
TODO:
- Write documentation with examples
- Write Tests (Also test non ascii templates)
- Write Benchmarks
- Implement and test no_std support

- Performance: The checking for add_argument_value adds overhead and the macro adds overhead by unwrapping these errors all the time.
  - Make a fully unchecked version of the macro that just panics but avoids these wrappings
- Clean up files and pull things apart

- Add an API with the specialization feature
- Better handling of dynamic precision and width

- Finalize Readme
- Fix Contributing

- Create CI/CD
- Publish crate
*/

mod argument;
mod arguments;
mod arguments_builder;
mod dyn_fmt;
mod error;
mod parser;
mod template;
mod macros;

pub use argument::{ArgumentKey, ArgumentTypeRequirements, ArgumentValue, ToArgumentKey};
pub use arguments::Arguments;
pub use arguments_builder::{ArgumentsBuilder, UncheckedArgumentsBuilder};
pub use dyn_fmt::DynFmt;
pub use error::Error;
pub use template::{Template, ToTemplate};
