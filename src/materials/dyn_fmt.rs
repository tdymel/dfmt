use crate::{ArgumentValue, Error, Template, ToArgumentKey};

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// Format extension using a list of key-value-pairs.
pub trait DynFmt {
    fn format(
        &self,
        argument_values: Vec<(&dyn ToArgumentKey, ArgumentValue<'_>)>,
    ) -> Result<String, Error>;

    fn format_unchecked(
        &self,
        argument_values: Vec<(&dyn ToArgumentKey, ArgumentValue<'_>)>,
    ) -> String;
}

impl DynFmt for Template {
    fn format(
        &self,
        argument_values: Vec<(&dyn ToArgumentKey, ArgumentValue<'_>)>,
    ) -> Result<String, Error> {
        let mut arguments = self.arguments();
        for (key, argument_value) in argument_values {
            arguments.add_argument_value(key.to_argument_key(), argument_value)?;
        }
        arguments.format()
    }

    fn format_unchecked(
        &self,
        argument_values: Vec<(&dyn ToArgumentKey, ArgumentValue<'_>)>,
    ) -> String {
        let mut arguments = self.arguments();
        for (key, argument_value) in argument_values {
            arguments.add_argument_value_unchecked(key.to_argument_key(), argument_value);
        }
        arguments.format().unwrap()
    }
}

impl DynFmt for &str {
    fn format(
        &self,
        argument_values: Vec<(&dyn ToArgumentKey, ArgumentValue<'_>)>,
    ) -> Result<String, Error> {
        Template::parse(self)?.format(argument_values)
    }

    fn format_unchecked(
        &self,
        argument_values: Vec<(&dyn ToArgumentKey, ArgumentValue<'_>)>,
    ) -> String {
        Template::parse(self)
            .unwrap()
            .format_unchecked(argument_values)
    }
}

impl DynFmt for String {
    fn format(
        &self,
        argument_values: Vec<(&dyn ToArgumentKey, ArgumentValue<'_>)>,
    ) -> Result<String, Error> {
        self.as_str().format(argument_values)
    }

    fn format_unchecked(
        &self,
        argument_values: Vec<(&dyn ToArgumentKey, ArgumentValue<'_>)>,
    ) -> String {
        self.as_str().format_unchecked(argument_values)
    }
}
