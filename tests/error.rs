use dfmt::*;

macro_rules! test_dformat {
    ($test_name:ident, $template:literal, $error:expr, $($args:tt)*) => {
        #[test]
        fn $test_name() {
            assert_eq!(
                dformat!($template.to_string(), $($args)*).unwrap_err(),
                $error
            )
        }
    };
}

test_dformat!(template_parsing_1, "Hello }{", Error::UnexpectedToken, 1);
test_dformat!(template_parsing_2, "Hello {", Error::UnexpectedToken, 1);
test_dformat!(template_parsing_3, "Hello }", Error::UnexpectedToken, 1);
test_dformat!(
    template_parsing_4,
    "Hello {{{} {",
    Error::UnexpectedToken,
    1
);
test_dformat!(
    unexpected_argument_value,
    "{arg}",
    Error::DuplicateArgument(TypedArgumentKey::new(
        ArgumentKey::Name("arg".to_string()),
        Type::Display
    )),
    arg = 42,
    arg = 3.14
);

test_dformat!(
    unexpected_argument_value_2,
    "{0} {0:o}",
    Error::UnexpectedArgumentValue,
    3.14
);

test_dformat!(
    argument_not_found,
    "{arg}",
    Error::ArgumentNotFound(ArgumentKey::Name("arrg".to_string())),
    arrg = 42
);
