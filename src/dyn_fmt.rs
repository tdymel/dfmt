use crate::{ArgumentValue, Error, Template, ToArgumentKey};

pub trait DynFmt<K: ToArgumentKey> {
    fn format(&self, argument_values: Vec<(K, ArgumentValue<'_>)>) -> Result<String, Error>;

    fn format_unchecked(&self, argument_values: Vec<(K, ArgumentValue<'_>)>) -> String;
}

impl<K: ToArgumentKey> DynFmt<K> for Template {
    fn format(&self, argument_values: Vec<(K, ArgumentValue<'_>)>) -> Result<String, Error> {
        let mut arguments = self.arguments();
        for (key, argument_value) in argument_values {
            arguments.add_argument_value(key, argument_value)?;
        }
        arguments.format()
    }

    fn format_unchecked(&self, argument_values: Vec<(K, ArgumentValue<'_>)>) -> String {
        let mut arguments = self.arguments();
        for (key, argument_value) in argument_values {
            arguments.add_argument_value_unchecked(key, argument_value);
        }
        arguments.format().unwrap()
    }
}

impl<K: ToArgumentKey> DynFmt<K> for &str {
    fn format(&self, argument_values: Vec<(K, ArgumentValue<'_>)>) -> Result<String, Error> {
        Template::parse_str(self)?.format(argument_values)
    }

    fn format_unchecked(&self, argument_values: Vec<(K, ArgumentValue<'_>)>) -> String {
        Template::parse_str(self)
            .unwrap()
            .format_unchecked(argument_values)
    }
}

impl<K: ToArgumentKey> DynFmt<K> for String {
    fn format(&self, argument_values: Vec<(K, ArgumentValue<'_>)>) -> Result<String, Error> {
        self.as_str().format(argument_values)
    }

    fn format_unchecked(&self, argument_values: Vec<(K, ArgumentValue<'_>)>) -> String {
        self.as_str().format_unchecked(argument_values)
    }
}
