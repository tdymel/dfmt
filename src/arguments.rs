use crate::{
    ArgumentKey, ArgumentTypeRequirements, ArgumentValue, Error, ToArgumentKey,
    argument::TypedValue, template::*,
};
use core::fmt::Write;

use crate::template::Template;

pub struct Arguments<'ct> {
    pub template: &'ct Template,
    pub argument_values: Vec<(ArgumentKey, ArgumentValue<'ct>)>,
}

impl<'ct> Arguments<'ct> {
    pub fn new(template: &'ct Template) -> Self {
        Self {
            template,
            argument_values: Vec::with_capacity(template.pieces.len()),
        }
    }

    fn find_argument_value(&self, key: &ArgumentKey) -> Result<&ArgumentValue<'ct>, Error> {
        self.argument_values
            .iter()
            .find(|it| &it.0 == key)
            .map(|it| &it.1)
            .ok_or_else(|| Error::ArgumentNotFound(key.clone()))
    }

    pub fn format(&self) -> Result<String, Error> {
        let mut result = String::with_capacity(
            self.template
                .pieces
                .iter()
                .map(|piece| match piece {
                    Piece::Literal(literal) => literal.len(),
                    Piece::BracketOpen | Piece::BracketClose => 1,
                    _ => 20,
                })
                .sum(),
        );

        for piece in &self.template.pieces {
            match piece {
                Piece::Literal(literal) => result.push_str(&literal),
                Piece::BracketOpen => result.push('{'),
                Piece::BracketClose => result.push('}'),
                Piece::Argument { key, specifier } => {
                    let argument_value = self.find_argument_value(key)?;

                    let dynamic_width = if let Some(specifier) = specifier {
                        match &specifier.width {
                            Width::Dynamic(key) => self.find_argument_value(key)?.to_u16(),
                            Width::Fixed(amount) => Some(*amount),
                        }
                    } else {
                        None
                    };

                    let dynamic_precision = if let Some(specifier) = specifier {
                        match &specifier.precision {
                            Precision::Dynamic(key) => self.find_argument_value(key)?.to_u16(),
                            Precision::Fixed(amount) => Some(*amount),
                            Precision::Auto => None,
                        }
                    } else {
                        None
                    };

                    write_argument_value(
                        &mut result,
                        specifier.as_ref(),
                        &TypedValue {
                            argument_value,
                            ty: specifier.as_ref().map(|it| it.ty).unwrap_or(Type::Display),
                        },
                        dynamic_width,
                        dynamic_precision,
                    )
                    .map_err(|err| Error::Fmt(err))?;
                }
            }
        }

        Ok(result)
    }

    // Builder
    pub fn add_argument_value<K: ToArgumentKey>(
        &mut self,
        key: K,
        value: ArgumentValue<'ct>,
    ) -> Result<(), Error> {
        let argument_key = key.to_argument_key();
        if self
            .argument_values
            .iter()
            .any(|(key, _)| key == &argument_key)
        {
            return Err(Error::DuplicateArgument(argument_key));
        }

        // TODO: This Check is very expensive
        value
            .fullfills()
            .requires(self.template.argument_type_requirements(&argument_key)?)?;
        self.argument_values.push((argument_key, value));
        Ok(())
    }

    pub fn add_argument_value_unchecked<K: ToArgumentKey>(
        &mut self,
        key: K,
        value: ArgumentValue<'ct>,
    ) {
        self.argument_values.push((key.to_argument_key(), value));
    }

    pub fn builder(self) -> Result<Self, Error> {
        Ok(self)
    }

    pub fn builder_unchecked(self) -> Self {
        self
    }
}

impl<'ct> core::fmt::Debug for Arguments<'ct> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Arguments")
            .field("template", &self.template)
            .field(
                "args",
                &self
                    .argument_values
                    .iter()
                    .map(|it| &it.0)
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

#[cfg(feature = "nightly_formatting_options")]
fn write_argument_value(
    output: &mut String,
    specifier: Option<&Specifier>,
    value: &TypedValue<'_>,
    width: Option<u16>,
    precision: Option<u16>,
) -> core::fmt::Result {
    if let Some(specifier) = specifier {
        let mut formatter = specifier
            .formatting_options()
            .width(width)
            .precision(precision)
            .create_formatter(output);
        core::fmt::Display::fmt(value, &mut formatter)
    } else {
        write!(output, "{}", value)
    }
}

#[cfg(not(feature = "nightly_formatting_options"))]
fn write_argument_value(
    output: &mut String,
    specifier: Option<&Specifier>,
    value: &TypedValue<'_>,
    width: Option<u16>,
    precision: Option<u16>,
) -> core::fmt::Result {
    if let Some(specifier) = specifier {
        let result = match (
            specifier.alignment,
            specifier.sign,
            specifier.alternate_form,
            specifier.pad_zero,
            &specifier.precision,
        ) {
            (Alignment::Auto, true, true, true, Precision::Auto) => {
                write!(output, "{:+#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, true, true, true, _) => {
                write!(
                    output,
                    "{:+#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Auto, true, true, false, Precision::Auto) => {
                write!(output, "{:+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, true, true, false, _) => {
                write!(
                    output,
                    "{:+#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Auto, true, false, true, Precision::Auto) => {
                write!(output, "{:+0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, true, false, true, _) => {
                write!(
                    output,
                    "{:+0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Auto, true, false, false, Precision::Auto) => {
                write!(output, "{:+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, true, false, false, _) => {
                write!(
                    output,
                    "{:+w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Auto, false, true, true, Precision::Auto) => {
                write!(output, "{:#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, false, true, true, _) => {
                write!(
                    output,
                    "{:#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Auto, false, true, false, Precision::Auto) => {
                write!(output, "{:#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, false, true, false, _) => {
                write!(
                    output,
                    "{:#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Auto, false, false, true, Precision::Auto) => {
                write!(output, "{:0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, false, false, true, _) => {
                write!(
                    output,
                    "{:0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Auto, false, false, false, Precision::Auto) => {
                write!(output, "{:w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, false, false, false, _) => {
                write!(
                    output,
                    "{:w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }

            (Alignment::Left, true, true, true, Precision::Auto) => {
                write!(output, "{:ꙮ<+#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, true, true, true, _) => {
                write!(
                    output,
                    "{:ꙮ<+#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Left, true, true, false, Precision::Auto) => {
                write!(output, "{:ꙮ<+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, true, true, false, _) => {
                write!(
                    output,
                    "{:ꙮ<+#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Left, true, false, true, Precision::Auto) => {
                write!(output, "{:ꙮ<+0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, true, false, true, _) => {
                write!(
                    output,
                    "{:ꙮ<+0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Left, true, false, false, Precision::Auto) => {
                write!(output, "{:ꙮ<+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, true, false, false, _) => {
                write!(
                    output,
                    "{:ꙮ<+w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Left, false, true, true, Precision::Auto) => {
                write!(output, "{:ꙮ<#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, false, true, true, _) => {
                write!(
                    output,
                    "{:ꙮ<#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Left, false, true, false, Precision::Auto) => {
                write!(output, "{:ꙮ<#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, false, true, false, _) => {
                write!(
                    output,
                    "{:ꙮ<#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Left, false, false, true, Precision::Auto) => {
                write!(output, "{:ꙮ<0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, false, false, true, _) => {
                write!(
                    output,
                    "{:ꙮ<0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Left, false, false, false, Precision::Auto) => {
                write!(output, "{:ꙮ<w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, false, false, false, _) => {
                write!(
                    output,
                    "{:ꙮ<w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }

            (Alignment::Center, true, true, true, Precision::Auto) => {
                write!(output, "{:ꙮ^+#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, true, true, true, _) => {
                write!(
                    output,
                    "{:ꙮ^+#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Center, true, true, false, Precision::Auto) => {
                write!(output, "{:ꙮ^+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, true, true, false, _) => {
                write!(
                    output,
                    "{:ꙮ^+#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Center, true, false, true, Precision::Auto) => {
                write!(output, "{:ꙮ^+0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, true, false, true, _) => {
                write!(
                    output,
                    "{:ꙮ^+0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Center, true, false, false, Precision::Auto) => {
                write!(output, "{:ꙮ^+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, true, false, false, _) => {
                write!(
                    output,
                    "{:ꙮ^+w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Center, false, true, true, Precision::Auto) => {
                write!(output, "{:ꙮ^#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, false, true, true, _) => {
                write!(
                    output,
                    "{:ꙮ^#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Center, false, true, false, Precision::Auto) => {
                write!(output, "{:ꙮ^#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, false, true, false, _) => {
                write!(
                    output,
                    "{:ꙮ^#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Center, false, false, true, Precision::Auto) => {
                write!(output, "{:ꙮ^0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, false, false, true, _) => {
                write!(
                    output,
                    "{:ꙮ^0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Center, false, false, false, Precision::Auto) => {
                write!(output, "{:ꙮ^w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, false, false, false, _) => {
                write!(
                    output,
                    "{:ꙮ^w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }

            (Alignment::Right, true, true, true, Precision::Auto) => {
                write!(output, "{:ꙮ>+#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, true, true, true, _) => {
                write!(
                    output,
                    "{:ꙮ>+#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Right, true, true, false, Precision::Auto) => {
                write!(output, "{:ꙮ>+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, true, true, false, _) => {
                write!(
                    output,
                    "{:ꙮ>+#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Right, true, false, true, Precision::Auto) => {
                write!(output, "{:ꙮ>+0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, true, false, true, _) => {
                write!(
                    output,
                    "{:ꙮ>+0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Right, true, false, false, Precision::Auto) => {
                write!(output, "{:ꙮ>+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, true, false, false, _) => {
                write!(
                    output,
                    "{:ꙮ>+w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Right, false, true, true, Precision::Auto) => {
                write!(output, "{:ꙮ>#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, false, true, true, _) => {
                write!(
                    output,
                    "{:ꙮ>#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Right, false, true, false, Precision::Auto) => {
                write!(output, "{:ꙮ>#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, false, true, false, _) => {
                write!(
                    output,
                    "{:ꙮ>#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Right, false, false, true, Precision::Auto) => {
                write!(output, "{:ꙮ>0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, false, false, true, _) => {
                write!(
                    output,
                    "{:ꙮ>0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (Alignment::Right, false, false, false, Precision::Auto) => {
                write!(output, "{:ꙮ>w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, false, false, false, _) => {
                write!(
                    output,
                    "{:ꙮ>w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
        };

        if specifier.alignment != Alignment::Auto && output.contains('ꙮ') {
            *output = output.replace('ꙮ', &specifier.fill_character.to_string());
        }

        result
    } else {
        write!(output, "{}", value)
    }
}
