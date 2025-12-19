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

    pub fn argument_type_requirements(
        &self,
        argument_key: &ArgumentKey,
    ) -> ArgumentTypeRequirements {
        self.template
            .pieces
            .iter()
            .filter(|it| match it {
                Piece::Argument { key, .. } => key == argument_key,
                _ => false,
            })
            .fold(ArgumentTypeRequirements::default(), |mut atr, arg| {
                // TODO: Width and Precision
                if let Piece::Argument { specifier, .. } = arg
                    && let Some(specifier) = specifier
                {
                    match specifier.ty {
                        Type::Binary => atr.binary = true,
                        Type::LowerHex => atr.lower_hex = true,
                        Type::UpperHex => atr.upper_hex = true,
                        Type::Octal => atr.octal = true,
                        Type::LowerExp => atr.lower_exp = true,
                        Type::UpperExp => atr.upper_exp = true,
                        Type::Debug => atr.debug = true,
                        Type::Display => atr.display = true,
                        Type::Pointer => atr.pointer = true,
                    }
                }

                atr
            })
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
            self.template.template.len() + self.template.pieces.iter().count() * 20,
        );

        for piece in &self.template.pieces {
            match piece {
                Piece::Literal { start, end } => {
                    result.push_str(&self.template.template[*start..*end])
                }
                Piece::BracketOpen => result.push('{'),
                Piece::BracketClose => result.push('}'),
                Piece::Argument { key, specifier } => {
                    let argument_value = self.find_argument_value(key)?;

                    let dynamic_width = if let Some(specifier) = specifier {
                        match &specifier.width {
                            Width::Dynamic(key) => self.find_argument_value(key)?.to_usize(),
                            Width::Fixed(amount) => *amount,
                        }
                    } else {
                        0
                    };

                    let dynamic_precision = if let Some(specifier) = specifier
                        && let Some(precision) = &specifier.precision
                    {
                        match precision {
                            Precision::Dynamic(key) => self.find_argument_value(key)?.to_usize(),
                            Precision::Fixed(amount) => *amount,
                        }
                    } else {
                        0
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
        if self.find_argument_value(&argument_key).is_ok() {
            return Err(Error::DuplicateArgument(argument_key));
        }

        // This Check is very expensive and redundant for the macro case
        value
            .fullfills()
            .requires(&self.argument_type_requirements(&argument_key))?;
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
            // TODO
            // .field("args", &self.argument_values)
            .finish()
    }
}

#[cfg(feature = "nightly_formatting_options")]
fn write_argument_value(
    output: &mut String,
    specifier: Option<&Specifier>,
    value: &TypedValue<'_>,
    width: usize,
    precision: usize,
) -> core::fmt::Result {
    if let Some(specifier) = specifier {
        let mut formatter = specifier
            .formatting_options()
            .width(Some(width as u16))
            .precision(Some(precision as u16))
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
    width: usize,
    precision: usize,
) -> core::fmt::Result {
    if let Some(specifier) = specifier {
        let result = match (
            specifier.alignment,
            specifier.sign,
            specifier.alternate_form,
            specifier.pad_zero,
            &specifier.precision,
        ) {
            (None, true, true, true, None) => write!(output, "{:+#0w$}", value, w = width),
            (None, true, true, true, Some(_)) => {
                write!(output, "{:+#0w$.p$}", value, w = width, p = precision)
            }
            (None, true, true, false, None) => write!(output, "{:+#w$}", value, w = width),
            (None, true, true, false, Some(_)) => {
                write!(output, "{:+#w$.p$}", value, w = width, p = precision)
            }
            (None, true, false, true, None) => write!(output, "{:+0w$}", value, w = width),
            (None, true, false, true, Some(_)) => {
                write!(output, "{:+0w$.p$}", value, w = width, p = precision)
            }
            (None, true, false, false, None) => write!(output, "{:+#w$}", value, w = width),
            (None, true, false, false, Some(_)) => {
                write!(output, "{:+w$.p$}", value, w = width, p = precision)
            }
            (None, false, true, true, None) => write!(output, "{:#0w$}", value, w = width),
            (None, false, true, true, Some(_)) => {
                write!(output, "{:#0w$.p$}", value, w = width, p = precision)
            }
            (None, false, true, false, None) => write!(output, "{:#w$}", value, w = width),
            (None, false, true, false, Some(_)) => {
                write!(output, "{:#w$.p$}", value, w = width, p = precision)
            }
            (None, false, false, true, None) => write!(output, "{:0w$}", value, w = width),
            (None, false, false, true, Some(_)) => {
                write!(output, "{:0w$.p$}", value, w = width, p = precision)
            }
            (None, false, false, false, None) => write!(output, "{:w$}", value, w = width),
            (None, false, false, false, Some(_)) => {
                write!(output, "{:w$.p$}", value, w = width, p = precision)
            }

            (Some(Alignment::Left), true, true, true, None) => {
                write!(output, "{:ꙮ<+#0w$}", value, w = width)
            }
            (Some(Alignment::Left), true, true, true, Some(_)) => {
                write!(output, "{:ꙮ<+#0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Left), true, true, false, None) => {
                write!(output, "{:ꙮ<+#w$}", value, w = width)
            }
            (Some(Alignment::Left), true, true, false, Some(_)) => {
                write!(output, "{:ꙮ<+#w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Left), true, false, true, None) => {
                write!(output, "{:ꙮ<+0w$}", value, w = width)
            }
            (Some(Alignment::Left), true, false, true, Some(_)) => {
                write!(output, "{:ꙮ<+0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Left), true, false, false, None) => {
                write!(output, "{:ꙮ<+#w$}", value, w = width)
            }
            (Some(Alignment::Left), true, false, false, Some(_)) => {
                write!(output, "{:ꙮ<+w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Left), false, true, true, None) => {
                write!(output, "{:ꙮ<#0w$}", value, w = width)
            }
            (Some(Alignment::Left), false, true, true, Some(_)) => {
                write!(output, "{:ꙮ<#0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Left), false, true, false, None) => {
                write!(output, "{:ꙮ<#w$}", value, w = width)
            }
            (Some(Alignment::Left), false, true, false, Some(_)) => {
                write!(output, "{:ꙮ<#w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Left), false, false, true, None) => {
                write!(output, "{:ꙮ<0w$}", value, w = width)
            }
            (Some(Alignment::Left), false, false, true, Some(_)) => {
                write!(output, "{:ꙮ<0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Left), false, false, false, None) => {
                write!(output, "{:ꙮ<w$}", value, w = width)
            }
            (Some(Alignment::Left), false, false, false, Some(_)) => {
                write!(output, "{:ꙮ<w$.p$}", value, w = width, p = precision)
            }

            (Some(Alignment::Center), true, true, true, None) => {
                write!(output, "{:ꙮ^+#0w$}", value, w = width)
            }
            (Some(Alignment::Center), true, true, true, Some(_)) => {
                write!(output, "{:ꙮ^+#0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Center), true, true, false, None) => {
                write!(output, "{:ꙮ^+#w$}", value, w = width)
            }
            (Some(Alignment::Center), true, true, false, Some(_)) => {
                write!(output, "{:ꙮ^+#w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Center), true, false, true, None) => {
                write!(output, "{:ꙮ^+0w$}", value, w = width)
            }
            (Some(Alignment::Center), true, false, true, Some(_)) => {
                write!(output, "{:ꙮ^+0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Center), true, false, false, None) => {
                write!(output, "{:ꙮ^+#w$}", value, w = width)
            }
            (Some(Alignment::Center), true, false, false, Some(_)) => {
                write!(output, "{:ꙮ^+w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Center), false, true, true, None) => {
                write!(output, "{:ꙮ^#0w$}", value, w = width)
            }
            (Some(Alignment::Center), false, true, true, Some(_)) => {
                write!(output, "{:ꙮ^#0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Center), false, true, false, None) => {
                write!(output, "{:ꙮ^#w$}", value, w = width)
            }
            (Some(Alignment::Center), false, true, false, Some(_)) => {
                write!(output, "{:ꙮ^#w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Center), false, false, true, None) => {
                write!(output, "{:ꙮ^0w$}", value, w = width)
            }
            (Some(Alignment::Center), false, false, true, Some(_)) => {
                write!(output, "{:ꙮ^0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Center), false, false, false, None) => {
                write!(output, "{:ꙮ^w$}", value, w = width)
            }
            (Some(Alignment::Center), false, false, false, Some(_)) => {
                write!(output, "{:ꙮ^w$.p$}", value, w = width, p = precision)
            }

            (Some(Alignment::Right), true, true, true, None) => {
                write!(output, "{:ꙮ>+#0w$}", value, w = width)
            }
            (Some(Alignment::Right), true, true, true, Some(_)) => {
                write!(output, "{:ꙮ>+#0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Right), true, true, false, None) => {
                write!(output, "{:ꙮ>+#w$}", value, w = width)
            }
            (Some(Alignment::Right), true, true, false, Some(_)) => {
                write!(output, "{:ꙮ>+#w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Right), true, false, true, None) => {
                write!(output, "{:ꙮ>+0w$}", value, w = width)
            }
            (Some(Alignment::Right), true, false, true, Some(_)) => {
                write!(output, "{:ꙮ>+0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Right), true, false, false, None) => {
                write!(output, "{:ꙮ>+#w$}", value, w = width)
            }
            (Some(Alignment::Right), true, false, false, Some(_)) => {
                write!(output, "{:ꙮ>+w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Right), false, true, true, None) => {
                write!(output, "{:ꙮ>#0w$}", value, w = width)
            }
            (Some(Alignment::Right), false, true, true, Some(_)) => {
                write!(output, "{:ꙮ>#0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Right), false, true, false, None) => {
                write!(output, "{:ꙮ>#w$}", value, w = width)
            }
            (Some(Alignment::Right), false, true, false, Some(_)) => {
                write!(output, "{:ꙮ>#w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Right), false, false, true, None) => {
                write!(output, "{:ꙮ>0w$}", value, w = width)
            }
            (Some(Alignment::Right), false, false, true, Some(_)) => {
                write!(output, "{:ꙮ>0w$.p$}", value, w = width, p = precision)
            }
            (Some(Alignment::Right), false, false, false, None) => {
                write!(output, "{:ꙮ>w$}", value, w = width)
            }
            (Some(Alignment::Right), false, false, false, Some(_)) => {
                write!(output, "{:ꙮ>w$.p$}", value, w = width, p = precision)
            }
        };

        if specifier.alignment.is_some() && output.contains('ꙮ') {
            *output = output.replace('ꙮ', &specifier.fill_character.unwrap_or(' ').to_string());
        }

        result
    } else {
        write!(output, "{}", value)
    }
}
