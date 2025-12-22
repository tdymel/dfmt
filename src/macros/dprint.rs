/// Dynamic drop in `print!` replacement.
/// ```rust
/// dfmt::dprint!("Hello, {}!", "World").unwrap();
/// dfmt::dprint!("Hello, {}!".to_string(), "World").unwrap();
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[macro_export]
macro_rules! dprint {
    ($template:literal, $($args:tt)*) => {{
        print!($template, $($args)*);
        Ok(()) as Result<(), $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            print!("{}", $crate::dformat!($template, $($args)*)?);
            Ok(())
        })()
    };
}

/// Dynamic drop in `print!` replacement. Unchecked variant.
/// ```rust
/// dfmt::dprint_unchecked!("Hello, {}!", "World");
/// dfmt::dprint_unchecked!("Hello, {}!".to_string(), "World");
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[macro_export]
macro_rules! dprint_unchecked {
    ($template:literal, $($args:tt)*) => {{
        print!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::dprint!($template, $($args)*).unwrap()
    };
}

/// Dynamic drop in `println!` replacement.
/// ```rust
/// dfmt::dprintln!("Hello, {}!", "World").unwrap();
/// dfmt::dprintln!("Hello, {}!".to_string(), "World").unwrap();
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[macro_export]
macro_rules! dprintln {
    ($template:literal, $($args:tt)*) => {{
        println!($template, $($args)*);
        Ok(()) as Result<(), $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        (|| -> Result<(), $crate::Error> {
            println!("{}", $crate::dformat!($template, $($args)*)?);
            Ok(())
        })()
    };
}

/// Dynamic drop in `println!` replacement. Unchecked variant.
/// ```rust
/// dfmt::dprintln_unchecked!("Hello, {}!", "World");
/// dfmt::dprintln_unchecked!("Hello, {}!".to_string(), "World");
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[macro_export]
macro_rules! dprintln_unchecked {
    ($template:literal, $($args:tt)*) => {{
        println!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::dprintln!($template, $($args)*).unwrap()
    };
}
