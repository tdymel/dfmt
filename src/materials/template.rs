use crate::{
    ArgumentKey, ArgumentTypeRequirements, Arguments, ToArgumentKey,
    error::Error,
    values::{Piece, Precision, Specifier, Type, Width},
};

#[derive(Debug, Clone)]
pub struct Template {
    pub pieces: Vec<Piece>,
    pub requirements: Vec<(ArgumentKey, ArgumentTypeRequirements)>,
}

impl Template {
    pub fn parse(template: &str) -> Result<Self, Error> {
        let pieces = Piece::parse(template)?;

        let mut requirements = Vec::with_capacity(pieces.len());
        pieces.iter().for_each(|piece| match piece {
            Piece::Argument { key, specifier } => {
                if let Some(specifier) = specifier {
                    Template::add_requirement(&mut requirements, key, specifier.ty);
                    if let Precision::Dynamic(precision_key) = &specifier.precision {
                        Template::add_requirement(&mut requirements, precision_key, Type::WidthOrPrecisionAmount);
                    }
                    if let Width::Dynamic(width_key) = &specifier.width {
                        Template::add_requirement(&mut requirements, width_key, Type::WidthOrPrecisionAmount);
                    }
                } else {
                    Template::add_requirement(&mut requirements, key, Type::Display);
                }
            }
            _ => {}
        });

        Ok(Self {
            pieces,
            requirements,
        })
    }

    pub fn arguments(&self) -> Arguments<'_> {
        Arguments::new(self)
    }

    pub fn to_template(&self) -> &Template {
        self
    }

    pub fn literal<V: ToString>(mut self, literal: V) -> Self {
        self.pieces.push(Piece::Literal(literal.to_string()));
        self
    }

    pub fn specified_argument<K: ToArgumentKey>(mut self, key: K, specifier: Specifier) -> Self {
        let argument_key = key.to_argument_key();
        Template::add_requirement(&mut self.requirements, &argument_key, specifier.ty);
        if let Precision::Dynamic(precision_key) = &specifier.precision {
            Template::add_requirement(&mut self.requirements, precision_key, Type::WidthOrPrecisionAmount);
        }
        if let Width::Dynamic(width_key) = &specifier.width {
            Template::add_requirement(&mut self.requirements, width_key, Type::WidthOrPrecisionAmount);
        }
        self.pieces.push(Piece::Argument {
            key: argument_key,
            specifier: Some(specifier),
        });
        self
    }

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

pub trait ToTemplate {
    fn to_template(self) -> Template;
}

impl ToTemplate for Template {
    fn to_template(self) -> Template {
        self
    }
}

impl ToTemplate for &str {
    fn to_template(self) -> Template {
        Template::parse(self).unwrap()
    }
}
