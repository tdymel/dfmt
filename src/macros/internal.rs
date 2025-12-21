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
        struct WrappedArg<'a, T>(&'a T);
        trait BlackMagic<'a> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error>;
        }
        impl<'a, T: core::fmt::Display
                    + core::fmt::Debug
                    + core::fmt::LowerExp
                    + core::fmt::UpperExp
                    + core::fmt::LowerHex
                    + core::fmt::UpperHex
                    + core::fmt::Binary
                    + core::fmt::Octal> BlackMagic<'a> for &&&&&&&&&&&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::IntegerLike(self.0))
            }
        }
        impl<'a, T: core::fmt::Display
                    + core::fmt::Debug
                    + core::fmt::LowerExp
                    + core::fmt::UpperExp> BlackMagic<'a> for &&&&&&&&&&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::FloatLike(self.0))
            }
        }
        impl<'a, T: core::fmt::Display + core::fmt::Debug> BlackMagic<'a> for &&&&&&&&&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::DisplayAndDebug(self.0))
            }
        }
        impl<'a, T: core::fmt::Display> BlackMagic<'a> for &&&&&&&&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::Display(self.0))
            }
        }
        impl<'a, T: core::fmt::Debug> BlackMagic<'a> for &&&&&&&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::Debug(self.0))
            }
        }
        impl<'a, T: core::fmt::UpperHex> BlackMagic<'a> for &&&&&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::UpperHex(self.0))
            }
        }
        impl<'a, T: core::fmt::LowerExp> BlackMagic<'a> for &&&&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::LowerExp(self.0))
            }
        }
        impl<'a, T: core::fmt::UpperExp> BlackMagic<'a> for &&&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::UpperExp(self.0))
            }
        }
        impl<'a, T: core::fmt::Octal> BlackMagic<'a> for &&&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::Octal(self.0))
            }
        }
        impl<'a, T: core::fmt::Binary> BlackMagic<'a> for &&WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::Binary(self.0))
            }
        }
        impl<'a, T: core::fmt::Pointer> BlackMagic<'a> for &WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Ok($crate::ArgumentValue::Pointer(self.0))
            }
        }
        impl<'a, T> BlackMagic<'a> for WrappedArg<'a, T> {
            fn to_argument_value(&self) -> Result<$crate::ArgumentValue<'a>, $crate::Error> {
                Err($crate::Error::UnexpectedArgumentValue)
            }
        }

        let typed_value = (&&&&&&&&&&&&&WrappedArg(&$value)).to_argument_value()?;
        if $checked {
            $arguments.add_argument_value($key, typed_value)
        } else {
            $arguments.add_argument_value_unchecked($key, typed_value);
            Ok(()) as Result<(), $crate::Error>
        }
    }};
}
