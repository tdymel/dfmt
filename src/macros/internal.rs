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

#[macro_export]
macro_rules! __internal__dfmt_process {
    ($checked:literal, $arguments:expr, $key:expr, $value:expr) => {{
        let typed_value = match $arguments.template.argument_type_requirements(&$key)? {
            $crate::ArgumentTypeRequirements {
                debug: true,
                display: true,
                lower_exp: true,
                upper_exp: true,
                lower_hex: true,
                upper_hex: true,
                binary: true,
                octal: true,
                ..
            } => {
                trait IntegerLike:
                    core::fmt::Display
                    + core::fmt::Debug
                    + core::fmt::LowerExp
                    + core::fmt::UpperExp
                    + core::fmt::LowerHex
                    + core::fmt::UpperHex
                    + core::fmt::Binary
                    + core::fmt::Octal
                {
                }
                impl<
                    T: core::fmt::Display
                        + core::fmt::Debug
                        + core::fmt::LowerExp
                        + core::fmt::UpperExp
                        + core::fmt::LowerHex
                        + core::fmt::UpperHex
                        + core::fmt::Binary
                        + core::fmt::Octal,
                > IntegerLike for T
                {
                }
                $crate::__internal__dfmt_perform_black_magic!($value, IntegerLike, IntegerLike)
            }
            $crate::ArgumentTypeRequirements {
                debug: true,
                display: true,
                lower_exp: true,
                upper_exp: true,
                ..
            } => {
                trait FloatLike:
                    core::fmt::Display
                    + core::fmt::Debug
                    + core::fmt::LowerExp
                    + core::fmt::UpperExp
                {
                }
                impl<
                    T: core::fmt::Display
                        + core::fmt::Debug
                        + core::fmt::LowerExp
                        + core::fmt::UpperExp,
                > FloatLike for T
                {
                }
                $crate::__internal__dfmt_perform_black_magic!($value, FloatLike, FloatLike)
            }
            $crate::ArgumentTypeRequirements {
                debug: true,
                display: true,
                ..
            } => {
                trait DisplayAndDebug: core::fmt::Display + core::fmt::Debug {}
                impl<T: core::fmt::Display + core::fmt::Debug> DisplayAndDebug for T {}
                $crate::__internal__dfmt_perform_black_magic!($value, DisplayAndDebug, DisplayAndDebug)
            }
            $crate::ArgumentTypeRequirements { display: true, .. } => {
                $crate::__internal__dfmt_perform_black_magic!($value, Display, core::fmt::Display)
            }
            $crate::ArgumentTypeRequirements { debug: true, .. } => {
                $crate::__internal__dfmt_perform_black_magic!($value, Debug, core::fmt::Debug)
            }
            $crate::ArgumentTypeRequirements { binary: true, .. } => {
                $crate::__internal__dfmt_perform_black_magic!($value, Binary, core::fmt::Binary)
            }
            $crate::ArgumentTypeRequirements { octal: true, .. } => {
                $crate::__internal__dfmt_perform_black_magic!($value, Octal, core::fmt::Octal)
            }
            $crate::ArgumentTypeRequirements { pointer: true, .. } => {
                $crate::__internal__dfmt_perform_black_magic!($value, Pointer, core::fmt::Pointer)
            }
            $crate::ArgumentTypeRequirements {
                lower_exp: true, ..
            } => {
                $crate::__internal__dfmt_perform_black_magic!($value, LowerExp, core::fmt::LowerExp)
            }
            $crate::ArgumentTypeRequirements {
                upper_exp: true, ..
            } => {
                $crate::__internal__dfmt_perform_black_magic!($value, UpperExp, core::fmt::UpperExp)
            }
            $crate::ArgumentTypeRequirements {
                lower_hex: true, ..
            } => {
                $crate::__internal__dfmt_perform_black_magic!($value, LowerHex, core::fmt::LowerHex)
            }
            $crate::ArgumentTypeRequirements {
                upper_hex: true, ..
            } => {
                $crate::__internal__dfmt_perform_black_magic!($value, UpperHex, core::fmt::UpperHex)
            }
            $crate::ArgumentTypeRequirements { .. } => {
                $crate::__internal__dfmt_perform_black_magic!($value, Display, core::fmt::Display)
            }
        };
        if $checked {
            $arguments.add_argument_value($key, typed_value)
        } else {
            $arguments.add_argument_value_unchecked($key, typed_value);
            Ok(()) as Result<(), $crate::Error>
        }
    }};
}

#[macro_export]
macro_rules! __internal__dfmt_perform_black_magic {
    ($value:expr, $variant:ident, $type:path) => {{
        struct WrappedArg<'a, T>(&'a T);
        trait BlackMagic<'a> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error>;
        }
        impl<'a, T: $type> BlackMagic<'a> for &WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::$variant(self.0))
            }
        }
        impl<'a, T> BlackMagic<'a> for WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Err($crate::Error::UnexpectedArgumentValue)
            }
        }

        (&&&WrappedArg(&$value)).to_argument_value()?
    }};
}
