use crate::{values::*, ArgumentKey, ArgumentValue, Error, Template, ToArgumentKey};
use core::fmt::Write;

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// Main structure to enrich the template with values and format the template to the end result.
pub struct Arguments<'ct> {
    pub template: &'ct Template,
    pub(crate) argument_values: Vec<(ArgumentKey, ArgumentValue<'ct>)>,
}

impl<'ct> Arguments<'ct> {
    /// Create a new container from a precompiled template.
    pub fn new(template: &'ct Template) -> Self {
        Self {
            template,
            argument_values: Vec::with_capacity(template.pieces.len()),
        }
    }

    /// Clear the argument values again
    pub fn clear(&mut self) {
        self.argument_values.clear();
    }

    fn find_argument_value(
        &self,
        key: &ArgumentKey,
        ty: &Type,
    ) -> Result<&ArgumentValue<'ct>, Error> {
        self.argument_values
            .iter()
            .find(|it| &it.0 == key && &it.1.to_type() == ty)
            .map(|it| &it.1)
            .ok_or_else(|| Error::ArgumentForTypeNotFound(TypedArgumentKey::new(key.clone(), *ty)))
    }

    /// Attempt to format the template with the provided values.
    pub fn format(&self) -> Result<String, Error> {
        let mut result = String::with_capacity(
            self.template
                .pieces
                .iter()
                .map(|piece| match piece {
                    Piece::Literal(literal) => literal.len(),
                    Piece::BracketOpen | Piece::BracketClose => 1,
                    _ => 40,
                })
                .sum(),
        );

        for piece in &self.template.pieces {
            match piece {
                Piece::Literal(literal) => result.push_str(literal),
                Piece::BracketOpen => result.push('{'),
                Piece::BracketClose => result.push('}'),
                Piece::Argument { key, specifier } => {
                    let ty = specifier.as_ref().map(|it| it.ty).unwrap_or(Type::Display);
                    let argument_value = self.find_argument_value(key, &ty)?;

                    let dynamic_width = if let Some(specifier) = specifier {
                        match &specifier.width {
                            Width::Dynamic(key) => self
                                .find_argument_value(key, &Type::WidthOrPrecisionAmount)?
                                .to_u16(),
                            Width::Fixed(amount) => Some(*amount),
                        }
                    } else {
                        None
                    };

                    let dynamic_precision = if let Some(specifier) = specifier {
                        match &specifier.precision {
                            Precision::Dynamic(key) => self
                                .find_argument_value(key, &Type::WidthOrPrecisionAmount)?
                                .to_u16(),
                            Precision::Fixed(amount) => Some(*amount),
                            Precision::Auto => None,
                        }
                    } else {
                        None
                    };

                    write_argument_value(
                        &mut result,
                        specifier.as_ref(),
                        argument_value,
                        dynamic_width,
                        dynamic_precision,
                    )
                    .map_err(Error::Fmt)?;
                }
            }
        }

        Ok(result)
    }

    // Builder
    /// Attempts to add an argument and checks for duplicate argument values.
    pub fn add_argument_value<K: ToArgumentKey>(
        &mut self,
        key: K,
        value: ArgumentValue<'ct>,
    ) -> Result<(), Error> {
        let argument_key = key.to_argument_key();
        let ty = value.to_type();
        if self
            .argument_values
            .iter()
            .any(|(key, val)| key == &argument_key && val.to_type() == ty)
        {
            return Err(Error::DuplicateArgument(TypedArgumentKey::new(
                argument_key,
                ty,
            )));
        }

        self.argument_values.push((argument_key, value));
        Ok(())
    }

    /// Adds an argument but does not execute any checks.
    pub fn add_argument_value_unchecked<K: ToArgumentKey>(
        &mut self,
        key: K,
        value: ArgumentValue<'ct>,
    ) {
        self.argument_values.push((key.to_argument_key(), value));
    }

    /// Transitions into the checked [`ArgumentsBuilder`][$crate::ArgumentsBuilder].
    pub fn builder(self) -> Result<Self, Error> {
        Ok(self)
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
                    .map(|it| (&it.0, it.1.to_type()))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

#[cfg(feature = "nightly_formatting_options")]
fn write_argument_value(
    output: &mut String,
    specifier: Option<&Specifier>,
    value: &ArgumentValue<'_>,
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
    value: &ArgumentValue<'_>,
    width: Option<u16>,
    precision: Option<u16>,
) -> core::fmt::Result {
    if let Some(specifier) = specifier {
        let result = match (
            specifier.alignment,
            &specifier.sign,
            &specifier.alternate_form,
            &specifier.pad_zero,
            &specifier.precision,
        ) {
            (
                Alignment::Auto,
                Sign::Plus,
                AlternateForm::Activated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:+#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, Sign::Plus, AlternateForm::Activated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:+#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Auto,
                Sign::Plus,
                AlternateForm::Activated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, Sign::Plus, AlternateForm::Activated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:+#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Auto,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:+0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, Sign::Plus, AlternateForm::Deactivated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:+0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Auto,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, Sign::Plus, AlternateForm::Deactivated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:+w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Auto,
                Sign::None,
                AlternateForm::Activated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, Sign::None, AlternateForm::Activated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Auto,
                Sign::None,
                AlternateForm::Activated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, Sign::None, AlternateForm::Activated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Auto,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, Sign::None, AlternateForm::Deactivated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Auto,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Auto, Sign::None, AlternateForm::Deactivated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }

            (
                Alignment::Left,
                Sign::Plus,
                AlternateForm::Activated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ<+#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, Sign::Plus, AlternateForm::Activated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ<+#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Left,
                Sign::Plus,
                AlternateForm::Activated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ<+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, Sign::Plus, AlternateForm::Activated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ<+#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Left,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ<+0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, Sign::Plus, AlternateForm::Deactivated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ<+0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Left,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ<+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, Sign::Plus, AlternateForm::Deactivated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ<+w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Left,
                Sign::None,
                AlternateForm::Activated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ<#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, Sign::None, AlternateForm::Activated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ<#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Left,
                Sign::None,
                AlternateForm::Activated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ<#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, Sign::None, AlternateForm::Activated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ<#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Left,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ<0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, Sign::None, AlternateForm::Deactivated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ<0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Left,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ<w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Left, Sign::None, AlternateForm::Deactivated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ<w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }

            (
                Alignment::Center,
                Sign::Plus,
                AlternateForm::Activated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ^+#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, Sign::Plus, AlternateForm::Activated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ^+#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Center,
                Sign::Plus,
                AlternateForm::Activated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ^+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, Sign::Plus, AlternateForm::Activated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ^+#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Center,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ^+0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, Sign::Plus, AlternateForm::Deactivated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ^+0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Center,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ^+#w$}", value, w = width.unwrap() as usize)
            }
            (
                Alignment::Center,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                _,
            ) => {
                write!(
                    output,
                    "{:ꙮ^+w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Center,
                Sign::None,
                AlternateForm::Activated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ^#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, Sign::None, AlternateForm::Activated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ^#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Center,
                Sign::None,
                AlternateForm::Activated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ^#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, Sign::None, AlternateForm::Activated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ^#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Center,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ^0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Center, Sign::None, AlternateForm::Deactivated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ^0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Center,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ^w$}", value, w = width.unwrap() as usize)
            }
            (
                Alignment::Center,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                _,
            ) => {
                write!(
                    output,
                    "{:ꙮ^w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }

            (
                Alignment::Right,
                Sign::Plus,
                AlternateForm::Activated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ>+#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, Sign::Plus, AlternateForm::Activated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ>+#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Right,
                Sign::Plus,
                AlternateForm::Activated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ>+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, Sign::Plus, AlternateForm::Activated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ>+#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Right,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ>+0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, Sign::Plus, AlternateForm::Deactivated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ>+0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Right,
                Sign::Plus,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ>+#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, Sign::Plus, AlternateForm::Deactivated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ>+w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Right,
                Sign::None,
                AlternateForm::Activated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ>#0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, Sign::None, AlternateForm::Activated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ>#0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Right,
                Sign::None,
                AlternateForm::Activated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ>#w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, Sign::None, AlternateForm::Activated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ>#w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Right,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Activated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ>0w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, Sign::None, AlternateForm::Deactivated, PadZero::Activated, _) => {
                write!(
                    output,
                    "{:ꙮ>0w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
            (
                Alignment::Right,
                Sign::None,
                AlternateForm::Deactivated,
                PadZero::Deactivated,
                Precision::Auto,
            ) => {
                write!(output, "{:ꙮ>w$}", value, w = width.unwrap() as usize)
            }
            (Alignment::Right, Sign::None, AlternateForm::Deactivated, PadZero::Deactivated, _) => {
                write!(
                    output,
                    "{:ꙮ>w$.p$}",
                    value,
                    w = width.unwrap() as usize,
                    p = precision.unwrap() as usize
                )
            }
        };

        if let Some(width) = width {
            if specifier.alignment != Alignment::Auto && width > 0 && output.contains('ꙮ') {
                *output = output.replace('ꙮ', &specifier.fill_character.to_string());
            }
        }

        result
    } else {
        write!(output, "{}", value)
    }
}
