use crate::{ArgumentValue, Arguments, Error, ToArgumentKey};
use core::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};

pub trait ArgumentsBuilder<'a> {
    fn format(self) -> Result<String, Error>;

    fn argument_value<K: ToArgumentKey>(self, key: K, value: ArgumentValue<'a>) -> Self;

    fn display<T: Display, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn debug<T: Debug, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn binary<T: Binary, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn octal<T: Octal, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn pointer<T: Pointer, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn lower_exp<T: LowerExp, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn upper_exp<T: UpperExp, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn lower_hex<T: LowerHex, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn upper_hex<T: UpperHex, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;
}

impl<'a> ArgumentsBuilder<'a> for Result<Arguments<'a>, Error> {
    fn format(self) -> Result<String, Error> {
        self?.format()
    }

    fn argument_value<K: ToArgumentKey>(
        self,
        key: K,
        value: ArgumentValue<'a>,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, value)?;
        Ok(args)
    }

    fn display<T: Display, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::Display(value))?;
        Ok(args)
    }

    fn debug<T: Debug, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::Debug(value))?;
        Ok(args)
    }

    fn binary<T: Binary, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::Binary(value))?;
        Ok(args)
    }

    fn octal<T: Octal, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::Octal(value))?;
        Ok(args)
    }

    fn pointer<T: Pointer, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::Pointer(value))?;
        Ok(args)
    }

    fn lower_exp<T: LowerExp, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::LowerExp(value))?;
        Ok(args)
    }

    fn upper_exp<T: UpperExp, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::UpperExp(value))?;
        Ok(args)
    }

    fn lower_hex<T: LowerHex, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::LowerHex(value))?;
        Ok(args)
    }

    fn upper_hex<T: UpperHex, K: ToArgumentKey>(
        self,
        key: K,
        value: &'a T,
    ) -> Result<Arguments<'a>, Error> {
        let mut args = self?;
        args.add_argument_value(key, ArgumentValue::UpperHex(value))?;
        Ok(args)
    }
}
pub trait UncheckedArgumentsBuilder<'a> {
    fn argument_value_unchecked<K: ToArgumentKey>(self, key: K, value: ArgumentValue<'a>) -> Self;

    fn display_unchecked<T: Display, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn debug_unchecked<T: Debug, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn binary_unchecked<T: Binary, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn octal_unchecked<T: Octal, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn pointer_unchecked<T: Pointer, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn lower_exp_unchecked<T: LowerExp, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn upper_exp_unchecked<T: UpperExp, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn lower_hex_unchecked<T: LowerHex, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;

    fn upper_hex_unchecked<T: UpperHex, K: ToArgumentKey>(self, key: K, value: &'a T) -> Self;
}

impl<'a> UncheckedArgumentsBuilder<'a> for Arguments<'a> {
    fn argument_value_unchecked<K: ToArgumentKey>(
        mut self,
        key: K,
        value: ArgumentValue<'a>,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, value);
        self
    }

    fn display_unchecked<T: Display, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::Display(value));
        self
    }

    fn debug_unchecked<T: Debug, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::Debug(value));
        self
    }

    fn binary_unchecked<T: Binary, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::Binary(value));
        self
    }

    fn octal_unchecked<T: Octal, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::Octal(value));
        self
    }

    fn pointer_unchecked<T: Pointer, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::Pointer(value));
        self
    }

    fn lower_exp_unchecked<T: LowerExp, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::LowerExp(value));
        self
    }

    fn upper_exp_unchecked<T: UpperExp, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::UpperExp(value));
        self
    }

    fn lower_hex_unchecked<T: LowerHex, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::LowerHex(value));
        self
    }

    fn upper_hex_unchecked<T: UpperHex, K: ToArgumentKey>(
        mut self,
        key: K,
        value: &'a T,
    ) -> Arguments<'a> {
        self.add_argument_value_unchecked(key, ArgumentValue::UpperHex(value));
        self
    }
}
