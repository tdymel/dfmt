#![feature(formatting_options)]

use dfmt::{Template, dformat, dformat_unchecked, dprint, dprintln};

#[derive(Debug)]
struct TestStruct;

const TEST_VALUE: i32 = 42;
const TEST_POINTER: *const i32 = &TEST_VALUE;

#[test]
fn debug_verify() {
    assert_eq!(
        dformat!("{:p}".to_string(), TEST_POINTER).unwrap(),
        format!("{:p}", TEST_POINTER)
    )
}

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

// test_dformat!(with_literal_pieces, "Hello, {}!", "world");
// test_dformat!(named_arg, "{arg} {}", "Hello", arg = 42);
// test_dformat!(indexed_arg, "{0} {1} {0}", 42, "Hello");
// test_dformat!(empty_spec, "{:}", "Hello");
// test_dformat!(ty_debug, "{:?}", TestStruct {});
// test_dformat!(ty_binary, "{:b}", 42);
// test_dformat!(ty_octal, "{:o}", 42);
// test_dformat!(ty_pointer, "{:p}", TEST_POINTER);

// #[test]
// fn dformat() {
//     assert_eq!(
//         dformat!("Hello, {}!".to_string(), "world").unwrap(),
//         format!("Hello, {}!", "world")
//     );
//     assert_eq!(
//         dformat!("{1}, {0}!".to_string(), "world", "Hello").unwrap(),
//         format!("{1}, {0}!", "world", "Hello")
//     );
//     assert_eq!(
//         dformat!(
//             "{greeting}, {name}!".to_string(),
//             greeting = "Hello",
//             name = "world"
//         )
//         .unwrap(),
//         format!("{greeting}, {name}!", greeting = "Hello", name = "world")
//     );
//     assert_eq!(
//         dformat!("{:?}".to_string(), TestStruct).unwrap(),
//         format!("{:?}", TestStruct)
//     );
//     assert_eq!(dformat!("{}", 42).unwrap(), format!("{}", 42));
//     assert_eq!(dformat!("{:}", 42).unwrap(), format!("{:}", 42));

//     let precision = 3;
//     assert_eq!(
//         dformat!("{:.prec$}".to_string(), 1.23456, prec = precision).unwrap(),
//         format!("{:.3}", 1.23456)
//     );
//     assert_eq!(
//         dformat!("{:.1$}".to_string(), 1.23456, 3).unwrap(),
//         format!("{:.3}", 1.23456)
//     );

//     let width = 5;
//     assert_eq!(
//         dformat!("{:width$}".to_string(), 42, width = width).unwrap(),
//         format!("{:5}", 42)
//     );
//     assert_eq!(
//         dformat!("{:1$}".to_string(), 42, 5).unwrap(),
//         format!("{:5}", 42)
//     );
//     assert_eq!(
//         dformat!("{:5}".to_string(), 42).unwrap(),
//         format!("{:5}", 42)
//     );

//     assert_eq!(
//         dformat!("{:x}".to_string(), 255).unwrap(),
//         format!("{:x}", 255)
//     );
//     assert_eq!(
//         dformat!("{:o}".to_string(), 255).unwrap(),
//         format!("{:o}", 255)
//     );
//     assert_eq!(
//         dformat!("{:b}".to_string(), 255).unwrap(),
//         format!("{:b}", 255)
//     );

//     assert_eq!(
//         dformat!("{:.2}".to_string(), 3.14159).unwrap(),
//         format!("{:.2}", 3.14159)
//     );
//     assert_eq!(
//         dformat!("{:e}".to_string(), 1234.56).unwrap(),
//         format!("{:e}", 1234.56)
//     );

//     assert_eq!(
//         dformat!("{:<5}".to_string(), 42).unwrap(),
//         format!("{:<5}", 42)
//     );
//     assert_eq!(
//         dformat!("{:>5}".to_string(), 42).unwrap(),
//         format!("{:>5}", 42)
//     );
//     assert_eq!(
//         dformat!("{:^5}".to_string(), 42).unwrap(),
//         format!("{:^5}", 42)
//     );

//     assert_eq!(
//         dformat!("{:*<5}".to_string(), 42).unwrap(),
//         format!("{:*<5}", 42)
//     );
//     assert_eq!(
//         dformat!("{:*>5}".to_string(), 42).unwrap(),
//         format!("{:*>5}", 42)
//     );
//     assert_eq!(
//         dformat!("{:*^5}".to_string(), 42).unwrap(),
//         format!("{:*^5}", 42)
//     );

//     assert_eq!(
//         dformat!("{:*>5.2}".to_string(), 42.567).unwrap(),
//         format!("{:*>5.2}", 42.567)
//     );
//     assert_eq!(
//         dformat!("{:.1$} and {:.*}".to_string(), 1.23456, 2, 5, 1.23456).unwrap(),
//         format!("{:.2} and {:.5}", 1.23456, 1.23456)
//     );

//     assert_eq!(
//         dformat!("{:#x}".to_string(), 255).unwrap(),
//         format!("{:#x}", 255)
//     );
//     assert_eq!(
//         dformat!("{:#b}".to_string(), 255).unwrap(),
//         format!("{:#b}", 255)
//     );
//     assert_eq!(
//         dformat!("{:#o}".to_string(), 255).unwrap(),
//         format!("{:#o}", 255)
//     );

//     assert_eq!(
//         dformat!("{:0>8}".to_string(), 42).unwrap(),
//         format!("{:0>8}", 42)
//     );
//     assert_eq!(
//         dformat!("{:0^8}".to_string(), 42).unwrap(),
//         format!("{:0^8}", 42)
//     );

//     assert_eq!(
//         dformat!("{:+}".to_string(), 42).unwrap(),
//         format!("{:+}", 42)
//     );
//     assert_eq!(
//         dformat!("{:+}".to_string(), -42).unwrap(),
//         format!("{:+}", -42)
//     );

//     assert_eq!(
//         dformat!("{:0>+#10x}".to_string(), 255).unwrap(),
//         format!("{:0>+#10x}", 255)
//     );
//     assert_eq!(
//         dformat!("{:<#10b}".to_string(), 255).unwrap(),
//         format!("{:<#10b}", 255)
//     );
//     assert_eq!(
//         dformat!("{:^#10o}".to_string(), 255).unwrap(),
//         format!("{:^#10o}", 255)
//     );
// }
