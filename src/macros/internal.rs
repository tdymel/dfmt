#[doc(hidden)]
#[macro_export]
macro_rules! __internal__dfmt {
    ($checked:literal, $template:expr, $($args:tt)*) => {
        (|| -> Result<String, $crate::Error> {
            let t = &$template;
            let template = {
                use $crate::ToTemplate;
                t.to_template()
            };
            let mut arguments = template.arguments();
            $crate::__internal__dfmt_process_args!($checked, arguments, 0, $($args)*)?;
            arguments.format()
        })()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal__dfmt_process_args {
    // Argument with ident key
    ($checked:literal, $arguments:expr, $index:expr, $key:ident = $value:expr, $($rest:tt)*) => {{
        $crate::__internal__dfmt_process!($checked, $arguments, $crate::ArgumentKey::Name(stringify!($key).to_string()), $value)?;
        $crate::__internal__dfmt_process_args!($checked, $arguments, $index, $($rest)*)
    }};
    ($checked:literal, $arguments:expr, $index:expr, $key:ident = $value:expr) => {
        $crate::__internal__dfmt_process!($checked, $arguments, $crate::ArgumentKey::Name(stringify!($key).to_string()), $value)
    };
    // Argument without key
    ($checked:literal, $arguments:expr, $index:expr, $value:expr, $($rest:tt)*) => {{
        $crate::__internal__dfmt_process!($checked, $arguments, $crate::ArgumentKey::Index($index), $value)?;
        $crate::__internal__dfmt_process_args!($checked, $arguments, $index + 1, $($rest)*)
    }};
    ($checked:literal, $arguments:expr, $index:expr, $value:expr) => {
        $crate::__internal__dfmt_process!($checked, $arguments, $crate::ArgumentKey::Index($index), $value)
    };
    // Base case for recursion, do nothing when no arguments are left
    ($checked:literal, $arguments:expr, $index:expr) => {Ok(())};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal__dfmt_process {
    ($checked:literal, $arguments:expr, $key:expr, $value:expr) => {{
        let requirements = $arguments.template.argument_type_requirements(&$key)?;

        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            display,
            $arguments,
            $key,
            $value,
            Display,
            core::fmt::Display
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            debug,
            $arguments,
            $key,
            $value,
            Debug,
            core::fmt::Debug
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            pointer,
            $arguments,
            $key,
            $value,
            Pointer,
            core::fmt::Pointer
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            binary,
            $arguments,
            $key,
            $value,
            Binary,
            core::fmt::Binary
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            octal,
            $arguments,
            $key,
            $value,
            Octal,
            core::fmt::Octal
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            lower_hex,
            $arguments,
            $key,
            $value,
            LowerHex,
            core::fmt::LowerHex
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            upper_hex,
            $arguments,
            $key,
            $value,
            UpperHex,
            core::fmt::UpperHex
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            lower_exp,
            $arguments,
            $key,
            $value,
            LowerExp,
            core::fmt::LowerExp
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            upper_exp,
            $arguments,
            $key,
            $value,
            UpperExp,
            core::fmt::UpperExp
        )?;
        $crate::__internal__dfmt_black_magic!(
            $checked,
            requirements,
            width_or_precision_amount,
            $arguments,
            $key,
            $value,
            WidthOrPrecisionAmount,
            $crate::WidthOrPrecisionAmount
        )?;
        
        Ok(()) as Result<(), $crate::Error>
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal__dfmt_black_magic {
    ($checked:literal, $requirements:expr, $req_variant:ident, $arguments:expr, $key:expr, $value:expr, $variant:ident, $ty:path) => {{
        if $requirements.$req_variant {
            struct WrappedArg<'a, T>(&'a T);
            trait BlackMagic<'a> {
                fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error>;
            }
            impl<'a, T: $ty> BlackMagic<'a> for &WrappedArg<'a, T> {
                fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                    Ok($crate::ArgumentValue::$variant(self.0))
                }
            }
            impl<'a, T> BlackMagic<'a> for WrappedArg<'a, T> {
                fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                    Err($crate::Error::UnexpectedArgumentValue)
                }
            }
            let typed_value = (&&WrappedArg(&$value)).to_argument_value()?;

            if $checked {
                $arguments.add_argument_value($key, typed_value)?;
            } else {
                $arguments.add_argument_value_unchecked($key, typed_value);
            }
        }
        Ok(()) as Result<(), $crate::Error>
    }};
}
