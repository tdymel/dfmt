
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