use crate::{
    error::Error,
    values::{Piece, Precision, Specifier, Type, Width},
    ArgumentKey, ArgumentTypeRequirements, Arguments, ToArgumentKey,
};

#[cfg(not(feature = "std"))]
use alloc::{string::ToString, vec::Vec};

/// Precompiled version of the string template.
#[derive(Debug, Clone, Default)]
pub struct Template {
    pub(crate) pieces: Vec<Piece>,
    pub(crate) requirements: Vec<(ArgumentKey, ArgumentTypeRequirements)>,
}

impl Template {
    /// Create an empty template, which can be filled using the builders.
    /// ```rust
    /// use dfmt::*;
    /// let formatted_string = Template::new()
    ///     .literal("Hello, ")
    ///     .specified_argument(0, Specifier::default())
    ///     .literal("!")
    ///     .arguments()
    ///     .builder()
    ///     .display(0, &"World")
    ///     .format()
    ///     .unwrap();
    /// println!("{}", formatted_string);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses a string template.
    pub fn parse(template: &str) -> Result<Self, Error> {
        let pieces = Piece::parse(template)?;

        let mut requirements = Vec::with_capacity(pieces.len());
        pieces.iter().for_each(|piece| {
            if let Piece::Argument { key, specifier } = piece {
                if let Some(specifier) = specifier {
                    Template::add_requirement(&mut requirements, key, specifier.ty);
                    if let Precision::Dynamic(precision_key) = &specifier.precision {
                        Template::add_requirement(
                            &mut requirements,
                            precision_key,
                            Type::WidthOrPrecisionAmount,
                        );
                    }
                    if let Width::Dynamic(width_key) = &specifier.width {
                        Template::add_requirement(
                            &mut requirements,
                            width_key,
                            Type::WidthOrPrecisionAmount,
                        );
                    }
                } else {
                    Template::add_requirement(&mut requirements, key, Type::Display);
                }
            }
        });

        Ok(Self {
            pieces,
            requirements,
        })
    }

    /// Transition into [`Arguments`][$crate::Arguments] for convinience.
    pub fn arguments(&self) -> Arguments<'_> {
        Arguments::new(self)
    }

    #[doc(hidden)]
    pub fn to_template(&self) -> Result<&Self, Error> {
        Ok(self)
    }

    /// Builder to add a literal piece.
    pub fn literal<V: ToString>(mut self, literal: V) -> Self {
        self.pieces.push(Piece::Literal(literal.to_string()));
        self
    }

    /// Builder to add an argument with a specifier.
    pub fn specified_argument<K: ToArgumentKey>(mut self, key: K, specifier: Specifier) -> Self {
        let argument_key = key.to_argument_key();
        Template::add_requirement(&mut self.requirements, &argument_key, specifier.ty);
        if let Precision::Dynamic(precision_key) = &specifier.precision {
            Template::add_requirement(
                &mut self.requirements,
                precision_key,
                Type::WidthOrPrecisionAmount,
            );
        }
        if let Width::Dynamic(width_key) = &specifier.width {
            Template::add_requirement(
                &mut self.requirements,
                width_key,
                Type::WidthOrPrecisionAmount,
            );
        }
        self.pieces.push(Piece::Argument {
            key: argument_key,
            specifier: Some(specifier),
        });
        self
    }

    /// Builder to add an argument without a specifier.
    pub fn argument<K: ToArgumentKey>(mut self, key: K) -> Self {
        let argument_key = key.to_argument_key();
        Template::add_requirement(&mut self.requirements, &argument_key, Type::Display);
        self.pieces.push(Piece::Argument {
            key: argument_key,
            specifier: None,
        });
        self
    }

    pub fn argument_type_requirements(
        &self,
        argument_key: &ArgumentKey,
    ) -> Result<&ArgumentTypeRequirements, Error> {
        self.requirements
            .iter()
            .find(|it| &it.0 == argument_key)
            .map(|it| &it.1)
            .ok_or_else(|| Error::ArgumentNotFound(argument_key.clone()))
    }

    fn add_requirement(
        requirements: &mut Vec<(ArgumentKey, ArgumentTypeRequirements)>,
        argument_key: &ArgumentKey,
        ty: Type,
    ) {
        if let Some((_, requirements)) =
            requirements.iter_mut().find(|(key, _)| key == argument_key)
        {
            requirements.add_requirement(ty);
        } else {
            let mut requirement = ArgumentTypeRequirements::default();
            requirement.add_requirement(ty);
            requirements.push((argument_key.clone(), requirement));
        };
    }
}

impl core::fmt::Display for Template {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for piece in &self.pieces {
            write!(f, "{piece}")?;
        }
        Ok(())
    }
}

#[doc(hidden)]
pub trait ToTemplate {
    fn to_template(self) -> Result<Template, Error>;
}

impl ToTemplate for Template {
    fn to_template(self) -> Result<Template, Error> {
        Ok(self)
    }
}

impl ToTemplate for &str {
    fn to_template(self) -> Result<Template, Error> {
        Template::parse(self)
    }
}
