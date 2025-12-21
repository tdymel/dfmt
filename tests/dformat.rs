use dfmt::dformat;

#[derive(Debug)]
struct TestStruct;

const TEST_VALUE: i32 = 42;
const TEST_POINTER: *const i32 = &TEST_VALUE;

macro_rules! test_dformat {
    ($test_name:ident, $template:literal, $($args:tt)*) => {
        #[test]
        fn $test_name() {
            assert_eq!(
                dformat!($template.to_string(), $($args)*).unwrap(),
                format!($template, $($args)*)
            )
        }
    };
}

test_dformat!(with_literal_pieces, "Hello, {}!", "world");
test_dformat!(with_non_ascii_chars, "Привет, {}!", "мир");
test_dformat!(with_non_ascii_chars_2, "Привет, {:^50}!", "мир");
test_dformat!(named_arg, "{arg} {} {0}", "Hello", arg = 42);
test_dformat!(indexed_arg, "{0} {1} {0}", 42, "Hello");
test_dformat!(empty_spec, "{:}", "Hello");
test_dformat!(ty_debug, "{:?}", TestStruct {});
test_dformat!(ty_binary, "{:b}", 42);
test_dformat!(ty_octal, "{:o}", 42);
test_dformat!(ty_pointer, "{:p}", TEST_POINTER);
test_dformat!(ty_lower_exp, "{:e}", 420000);
test_dformat!(ty_upper_exp, "{:E}", 420000);
test_dformat!(ty_lower_hex, "{:x}", 420000);
test_dformat!(ty_upper_hex, "{:X}", 420000);
test_dformat!(width, "{:w$}", 32.23322323, w = 3);
test_dformat!(width_fixed, "{:20}", 32.23322323);
test_dformat!(precision, "{:.prec$}", 32.23322323, prec = 3);
test_dformat!(precision_fixed, "{:.20}", 32.23322323);
test_dformat!(precision_dyn, "{:.*}", 4, 32.23322323);
test_dformat!(ty_debug_alt, "{:#?}", TestStruct {});
test_dformat!(ty_binary_alt, "{:#b}", 42);
test_dformat!(ty_octal_alt, "{:#o}", 42);
test_dformat!(ty_lower_hex_alt, "{:#x}", 420000);
test_dformat!(ty_upper_hex_alt, "{:#X}", 420000);
test_dformat!(zero_pad, "{:020}", 42);
test_dformat!(align_left, "{:*<20}", 42);
test_dformat!(align_center, "{:*^20}", 42);
test_dformat!(align_right, "{:*>20}", 42);
test_dformat!(sign_plus, "{:+}", 42);
test_dformat!(sign_minus, "{:-}", 42);
test_dformat!(float_with_precision_and_upper_e, "{:.3E}", 42.3232323232);
test_dformat!(
    float_with_precision_and_upper_e_var,
    "{:.p$E}",
    42.3232323232,
    p = 3
);
test_dformat!(
    float_with_precision_and_upper_e_dyn,
    "{:.*E}",
    3,
    42.3232323232
);
test_dformat!(everything_1, "{arg:*>+#020.4o}", arg = 42);
test_dformat!(everything_2, "{arg:*>+#020.4E}", arg = 42.232323);
test_dformat!(
    everything_3,
    "{arg:*>+#0w$.p$E}",
    arg = 42.232323,
    w = 50,
    p = 3
);
test_dformat!(everything_4, "{arg:*>+#0w$.*E}", 5, arg = 42.232323, w = 50);
