/// Dynamic drop in `eprint!` replacement.
/// ```rust
/// dfmt::deprint!("Hello, {}!", "World").unwrap();
/// dfmt::deprint!("Hello, {}!".to_string(), "World").unwrap();
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[macro_export]
macro_rules! deprint {
    ($template:literal, $($args:tt)*) => {{
        eprint!($template, $($args)*);
        Ok(()) as Result<(), $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            eprint!("{}", $crate::dformat!($template, $($args)*)?);
            Ok(())
        })()
    };
}

/// Dynamic drop in `eprint!` replacement. Unchecked variant.
/// ```rust
/// dfmt::deprint_unchecked!("Hello, {}!", "World");
/// dfmt::deprint_unchecked!("Hello, {}!".to_string(), "World");
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[macro_export]
macro_rules! deprint_unchecked {
    ($template:literal, $($args:tt)*) => {{
        eprint!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::deprint!($template, $($args)*).unwrap()
    };
}

/// Dynamic drop in `eprintln!` replacement.
/// ```rust
/// dfmt::deprintln!("Hello, {}!", "World").unwrap();
/// dfmt::deprintln!("Hello, {}!".to_string(), "World").unwrap();
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[macro_export]
macro_rules! deprintln {
    ($template:literal, $($args:tt)*) => {{
        eprintln!($template, $($args)*);
        Ok(()) as Result<(), $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            eprintln!("{}", $crate::dformat!($template, $($args)*)?);
            Ok(())
        })()
    };
}

/// Dynamic drop in `eprintln!` replacement. Unchecked variant.
/// ```rust
/// dfmt::deprintln_unchecked!("Hello, {}!", "World");
/// dfmt::deprintln_unchecked!("Hello, {}!".to_string(), "World");
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[macro_export]
macro_rules! deprintln_unchecked {
    ($template:literal, $($args:tt)*) => {{
        eprintln!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::deprintln!($template, $($args)*).unwrap()
    };
}
