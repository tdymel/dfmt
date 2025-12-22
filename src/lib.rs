#![cfg_attr(feature = "nightly_formatting_options", feature(formatting_options))]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(not(feature = "std"))]
extern crate alloc;

mod error;
mod macros;
mod materials;
mod values;

pub use error::Error;
pub use materials::{
    Arguments, ArgumentsBuilder, DynFmt, Template, ToTemplate, UncheckedArgumentsBuilder,
};
pub use values::{
    Alignment, ArgumentKey, ArgumentTypeRequirements, ArgumentValue, Precision, Specifier,
    ToArgumentKey, Type, TypedArgumentKey, Width, WidthOrPrecisionAmount,
};
