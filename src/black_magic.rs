#[macro_export]
macro_rules! dformat {
    ($template:literal, $($args:tt)*) => {{
        Ok(format!($template, $($args)*)) as Result<String, $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::dformat_internal!(true, $template, $($args)*)
    };
}

#[macro_export]
macro_rules! dformat_unchecked {
    ($template:literal, $($args:tt)*) => {{
        format!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::dformat_internal!(false, $template, $($args)*).unwrap()
    };
}

#[macro_export]
macro_rules! dprint {
    ($template:literal, $($args:tt)*) => {{
        print!($template, $($args)*);
        Ok(()) as Result<(), $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            print!("{}", dformat!($template, $($args)*)?);
            Ok(())
        })()
    };
}

#[macro_export]
macro_rules! dprint_unchecked {
    ($template:literal, $($args:tt)*) => {{
        print!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::dprint!($template, $($args)*).unwrap()
    };
}

#[macro_export]
macro_rules! dprintln {
    ($template:literal, $($args:tt)*) => {{
        println!($template, $($args)*);
        Ok(()) as Result<(), $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            println!("{}", dformat!($template, $($args)*)?);
            Ok(())
        })()
    };
}

#[macro_export]
macro_rules! dprintln_unchecked {
    ($template:literal, $($args:tt)*) => {{
        println!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::dprintln!($template, $($args)*).unwrap()
    };
}

#[macro_export]
macro_rules! deprint {
    ($template:literal, $($args:tt)*) => {{
        eprint!($template, $($args)*);
        Ok(()) as Result<(), $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            eprint!("{}", dformat!($template, $($args)*)?);
            Ok(())
        })()
    };
}

#[macro_export]
macro_rules! deprint_unchecked {
    ($template:literal, $($args:tt)*) => {{
        eprint!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::deprint!($template, $($args)*).unwrap()
    };
}

#[macro_export]
macro_rules! deprintln {
    ($template:literal, $($args:tt)*) => {{
        eprintln!($template, $($args)*);
        Ok(()) as Result<(), $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            eprintln!("{}", dformat!($template, $($args)*)?);
            Ok(())
        })()
    };
}

#[macro_export]
macro_rules! deprintln_unchecked {
    ($template:literal, $($args:tt)*) => {{
        eprintln!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::deprintln!($template, $($args)*).unwrap()
    };
}

#[macro_export]
macro_rules! dwrite {
    ($output:expr, $template:literal, $($args:tt)*) => {{
        write!($output, $template, $($args)*)
            .map_err(|err| $crate::Error::Fmt(err))
    }};
    ($output:expr, $template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            write!($output, "{}", dformat!($template, $($args)*)?)
                .map_err(|err| $crate::Error::Fmt(err))
        })()
    };
}

#[macro_export]
macro_rules! dwriteln {
    ($output:expr, $template:literal, $($args:tt)*) => {{
        writeln!($output, $template, $($args)*)
            .map_err(|err| $crate::Error::Fmt(err))
    }};
    ($output:expr, $template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            writeln!($output, "{}", dformat!($template, $($args)*)?)
                .map_err(|err| $crate::Error::Fmt(err))
        })()
    };
}


#[macro_export]
macro_rules! dformat_internal {
    ($checked:literal, $template:expr, $($args:tt)*) => {
        (|| -> Result<String, $crate::Error> {
            let t = &$template;
            let template = {
                use $crate::ToTemplate;
                t.to_template()
            };
            let mut arguments = template.arguments();
            $crate::dformat_process_args!($checked, arguments, 0, $($args)*)?;
            arguments.format()
        })()
    };
}

#[macro_export]
macro_rules! dformat_process_args {
    // Argument with ident key
    ($checked:literal, $arguments:expr, $index:expr, $key:ident = $value:expr, $($rest:tt)*) => {{
        $crate::dformat_process!($checked, $arguments, $crate::ArgumentKey::Name(stringify!($key).to_string()), $value)?;
        $crate::dformat_process_args!($checked, $arguments, $index, $($rest)*)
    }};
    ($checked:literal, $arguments:expr, $index:expr, $key:ident = $value:expr) => {
        $crate::dformat_process!($checked, $arguments, $crate::ArgumentKey::Name(stringify!($key).to_string()), $value)
    };
    // Argument without key
    ($checked:literal, $arguments:expr, $index:expr, $value:expr, $($rest:tt)*) => {{
        $crate::dformat_process!($checked, $arguments, $crate::ArgumentKey::Index($index), $value)?;
        $crate::dformat_process_args!($checked, $arguments, $index + 1, $($rest)*)
    }};
    ($checked:literal, $arguments:expr, $index:expr, $value:expr) => {
        $crate::dformat_process!($checked, $arguments, $crate::ArgumentKey::Index($index), $value)
    };
    // Base case for recursion, do nothing when no arguments are left
    ($checked:literal, $arguments:expr, $index:expr) => {Ok(())};
}

#[macro_export]
macro_rules! dformat_process {
    ($checked:literal, $arguments:expr, $key:expr, $value:expr) => {{
        let typed_value = match $arguments.argument_type_requirements(&$key) {
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
                $crate::perform_black_magic!($value, IntegerLike, IntegerLike)
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
                $crate::perform_black_magic!($value, FloatLike, FloatLike)
            }
            $crate::ArgumentTypeRequirements {
                debug: true,
                display: true,
                ..
            } => {
                trait DisplayAndDebug: core::fmt::Display + core::fmt::Debug {}
                impl<T: core::fmt::Display + core::fmt::Debug> DisplayAndDebug for T {}
                $crate::perform_black_magic!($value, DisplayAndDebug, DisplayAndDebug)
            }
            $crate::ArgumentTypeRequirements { display: true, .. } => {
                $crate::perform_black_magic!($value, Display, core::fmt::Display)
            }
            $crate::ArgumentTypeRequirements { debug: true, .. } => {
                $crate::perform_black_magic!($value, Debug, core::fmt::Debug)
            }
            $crate::ArgumentTypeRequirements { binary: true, .. } => {
                $crate::perform_black_magic!($value, Binary, core::fmt::Binary)
            }
            $crate::ArgumentTypeRequirements { octal: true, .. } => {
                $crate::perform_black_magic!($value, Octal, core::fmt::Octal)
            }
            $crate::ArgumentTypeRequirements { pointer: true, .. } => {
                $crate::perform_black_magic!($value, Pointer, core::fmt::Pointer)
            }
            $crate::ArgumentTypeRequirements {
                lower_exp: true, ..
            } => {
                $crate::perform_black_magic!($value, LowerExp, core::fmt::LowerExp)
            }
            $crate::ArgumentTypeRequirements {
                upper_exp: true, ..
            } => {
                $crate::perform_black_magic!($value, UpperExp, core::fmt::UpperExp)
            }
            $crate::ArgumentTypeRequirements {
                lower_hex: true, ..
            } => {
                $crate::perform_black_magic!($value, LowerHex, core::fmt::LowerHex)
            }
            $crate::ArgumentTypeRequirements {
                upper_hex: true, ..
            } => {
                $crate::perform_black_magic!($value, UpperHex, core::fmt::UpperHex)
            }
            $crate::ArgumentTypeRequirements { .. } => {
                $crate::perform_black_magic!($value, Display, core::fmt::Display)
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
macro_rules! perform_black_magic {
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
