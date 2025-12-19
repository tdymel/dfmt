#![cfg_attr(feature = "nightly_formatting_options", feature(formatting_options))]

/*
TODO:
- Write documentation with examples
- Write Tests
- Implement ToTemplate String function via Display probably
- Implement and test no_std support

- Add API to create Template directly and sidestep string based constructor and parser
  - Guess we need another builder

- Performance: The checking for add_argument_value adds overhead and the macro adds overhead by unwrapping these errors all the time.
  - Make a fully unchecked version of the macro that just panics but avoids these wrappings
- Clean up files and pull things apart
- Rename internal macros, such that they are not easily found

- Add an API with the specialization feature

- Rename the crate to dfmt
*/

mod argument;
mod arguments;
mod arguments_builder;
mod black_magic;
mod dyn_fmt;
mod error;
mod parser;
mod template;

pub use argument::{ArgumentKey, ArgumentTypeRequirements, ArgumentValue, ToArgumentKey};
pub use arguments::Arguments;
pub use arguments_builder::{ArgumentsBuilder, UncheckedArgumentsBuilder};
pub use dyn_fmt::DynFmt;
pub use error::Error;
pub use template::{Template, ToTemplate};
