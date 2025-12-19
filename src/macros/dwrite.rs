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