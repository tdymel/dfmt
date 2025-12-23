/// Dynamic drop in `format!` replacement.
/// ```rust
/// use dfmt::*;
///
/// let template = "This example template {contains:*^width$} multiple {{{0}}}
///     named and {} indexed args {0:+020o} {3:#?} {:.*}.";
/// let precompiled_template = Template::parse(template).unwrap();
///
/// dformat!(template, contains = "STH", 42, 3, 3.1423223, template, width=50).unwrap();
/// dformat!(precompiled_template, contains = "STH", 42, 3, 3.1423223, template, width=50).unwrap();
///
/// // Use `format!` under the hood
/// dfmt::dformat!("Hello, {}!", "World").unwrap();
/// ```
///
/// ## Supported features
/// | Name | Feature |
/// | ---- | ------- |
/// | Fill/Alignment | `<`, `^`, `>` |
/// | Sign | `+`, `-` |
/// | Alternate | `#` |
/// | Zero-padding | `0` |
/// | Width | `{:20}`, `{:width$}` |
/// | Precision | `{:.5}`, `{:.precision$}`, `{:*}` |
/// | Type | `?`, `x`, `X`, `o`, `b`, `e`, `E`, `p` |
/// | Argument keys | `{}`, `{0}`, `{arg}` |
#[cfg(feature = "std")]
#[macro_export]
macro_rules! dformat {
    ($template:literal, $($args:tt)*) => {{
        #[cfg(not(feature = "std"))]
        use alloc::format;

        Ok(format!($template, $($args)*)) as Result<String, $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::__internal__dfmt!(true, $template, $($args)*)
    };
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! dformat {
    ($template:literal, $($args:tt)*) => {{
        Ok(alloc::format!($template, $($args)*)) as Result<String, $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::__internal__dfmt!(true, $template, $($args)*)
    };
}

/// Dynamic drop in `format!` replacement. Unchecked variant.
/// ```rust
/// dfmt::dformat_unchecked!("Hello, {}!", "World");
/// dfmt::dformat_unchecked!("Hello, {}!".to_string(), "World");
/// ```
/// Refer to the [`dformat!()`][$crate::dformat] documentation for the full API overview.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! dformat_unchecked {
    ($template:literal, $($args:tt)*) => {{
        format!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::__internal__dfmt!(false, $template, $($args)*).unwrap()
    };
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! dformat_unchecked {
    ($template:literal, $($args:tt)*) => {{
        alloc::format!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::__internal__dfmt!(false, $template, $($args)*).unwrap()
    };
}
