#[macro_export]
macro_rules! dformat {
    ($template:literal, $($args:tt)*) => {{
        Ok(format!($template, $($args)*)) as Result<String, $crate::Error>
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::__internal__dfmt!(true, $template, $($args)*)
    };
}

#[macro_export]
macro_rules! dformat_unchecked {
    ($template:literal, $($args:tt)*) => {{
        format!($template, $($args)*)
    }};
    ($template:expr, $($args:tt)*) => {
        $crate::__internal__dfmt!(false, $template, $($args)*).unwrap()
    };
}