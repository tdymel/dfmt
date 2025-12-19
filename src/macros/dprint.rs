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