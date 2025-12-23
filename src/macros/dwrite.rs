/// Dynamic drop in `write!` replacement.
///
/// ```rust
/// use core::fmt::Write;
///
/// let mut output = String::new();
/// dfmt::dwrite!(&mut output, "Hello, {}!", "World").unwrap();
/// dfmt::dwrite!(&mut output, "Hello, {}!".to_string(), "World").unwrap();
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! dwrite {
    ($output:expr, $template:literal, $($args:tt)*) => {{
        write!($output, $template, $($args)*)
            .map_err(|err| $crate::Error::Fmt(err))
    }};
    ($output:expr, $template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            write!($output, "{}", $crate::dformat!($template, $($args)*)?)
                .map_err(|err| $crate::Error::Fmt(err))
        })()
    };
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! dwrite {
    ($output:expr, $template:literal, $($args:tt)*) => {{
        alloc::write!($output, $template, $($args)*)
            .map_err(|err| $crate::Error::Fmt(err))
    }};
    ($output:expr, $template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            alloc::write!($output, "{}", $crate::dformat!($template, $($args)*)?)
                .map_err(|err| $crate::Error::Fmt(err))
        })()
    };
}

/// Dynamic drop in `writeln!` replacement.
/// ```rust
/// use core::fmt::Write;
///
/// let mut output = String::new();
/// dfmt::dwriteln!(&mut output, "Hello, {}!", "World").unwrap();
/// dfmt::dwriteln!(&mut output, "Hello, {}!".to_string(), "World").unwrap();
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! dwriteln {
    ($output:expr, $template:literal, $($args:tt)*) => {{
        writeln!($output, $template, $($args)*)
            .map_err(|err| $crate::Error::Fmt(err))
    }};
    ($output:expr, $template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            writeln!($output, "{}", $crate::dformat!($template, $($args)*)?)
                .map_err(|err| $crate::Error::Fmt(err))
        })()
    };
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! dwriteln {
    ($output:expr, $template:literal, $($args:tt)*) => {{
        alloc::writeln!($output, $template, $($args)*)
            .map_err(|err| $crate::Error::Fmt(err))
    }};
    ($output:expr, $template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            alloc::writeln!($output, "{}", $crate::dformat!($template, $($args)*)?)
                .map_err(|err| $crate::Error::Fmt(err))
        })()
    };
}
