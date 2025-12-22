#![feature(test)]

extern crate test;

use dfmt::{dformat, dformat_unchecked, ArgumentsBuilder, Template, UncheckedArgumentsBuilder};
use test::Bencher;

#[derive(Debug)]
pub struct TestStruct {
    pub hello: i32,
    pub world: i32,
}

#[bench]
fn template_parse_simple_1arg(b: &mut Bencher) {
    let format_string_simple_1arg = "Some string {}.".to_string();
    b.iter(|| Template::parse(&format_string_simple_1arg));
}

#[bench]
fn template_parse_simple_7args(b: &mut Bencher) {
    let format_string_simple_7args = "Some string {} sth {}{} {} {}{}{}.".to_string();
    b.iter(|| Template::parse(&format_string_simple_7args));
}

#[bench]
fn template_parse_complex(b: &mut Bencher) {
    let format_string_complex =
        "This is a {{{0:#x}}} {0:o} {3:?} {2:0w$} {1} {arg:.p$E} {:*^20}.".to_string();
    b.iter(|| Template::parse(&format_string_complex));
}

#[bench]
fn dformat_simple_1arg(b: &mut Bencher) {
    let format_string_simple_1arg = "Some string {}.".to_string();
    let compiled_template_simple_1arg = Template::parse(&format_string_simple_1arg).unwrap();
    let simple_arg1 = "simple arg 1";
    b.iter(|| dformat!(compiled_template_simple_1arg, simple_arg1).unwrap());
}

#[bench]
fn dformat_simple_7args(b: &mut Bencher) {
    let format_string_simple_7args = "Some string {} sth {}{} {} {}{}{}.".to_string();
    let compiled_template_simple_7args = Template::parse(&format_string_simple_7args).unwrap();
    let args = [
        "simple arg 1",
        "simple arg 2",
        "simple arg 3",
        "simple arg 4",
        "simple arg 5",
        "simple arg 6",
        "simple arg 7",
    ];
    b.iter(|| {
        dformat!(
            compiled_template_simple_7args,
            args[0],
            args[1],
            args[2],
            args[3],
            args[4],
            args[5],
            args[6]
        )
        .unwrap()
    });
}

#[bench]
fn dformat_unchecked_simple_1arg(b: &mut Bencher) {
    let format_string_simple_1arg = "Some string {}.".to_string();
    let compiled_template_simple_1arg = Template::parse(&format_string_simple_1arg).unwrap();
    let simple_arg1 = "simple arg 1";
    b.iter(|| dformat_unchecked!(compiled_template_simple_1arg, simple_arg1));
}

#[bench]
fn dformat_unchecked_simple_7args(b: &mut Bencher) {
    let format_string_simple_7args = "Some string {} sth {}{} {} {}{}{}.".to_string();
    let compiled_template_simple_7args = Template::parse(&format_string_simple_7args).unwrap();
    let args = [
        "simple arg 1",
        "simple arg 2",
        "simple arg 3",
        "simple arg 4",
        "simple arg 5",
        "simple arg 6",
        "simple arg 7",
    ];
    b.iter(|| {
        dformat_unchecked!(
            compiled_template_simple_7args,
            args[0],
            args[1],
            args[2],
            args[3],
            args[4],
            args[5],
            args[6]
        )
    });
}

#[bench]
fn format_simple_1arg(b: &mut Bencher) {
    let simple_arg1 = "simple arg 1";
    b.iter(|| format!("Some string {}.", simple_arg1));
}

#[bench]
fn format_simple_7args(b: &mut Bencher) {
    let args = [
        "simple arg 1",
        "simple arg 2",
        "simple arg 3",
        "simple arg 4",
        "simple arg 5",
        "simple arg 6",
        "simple arg 7",
    ];
    b.iter(|| {
        format!(
            "Some string {} sth {}{} {} {}{}{}.",
            args[0], args[1], args[2], args[3], args[4], args[5], args[6]
        )
    });
}

#[bench]
fn manual_via_template_simple_1arg(b: &mut Bencher) {
    let format_string_simple_1arg = "Some string {}.".to_string();
    let compiled_template_simple_1arg = Template::parse(&format_string_simple_1arg).unwrap();
    let simple_arg1 = "simple arg 1";
    b.iter(|| {
        compiled_template_simple_1arg
            .arguments()
            .builder()
            .display(0, &simple_arg1)
            .format()
            .unwrap();
    });
}

#[bench]
fn manual_via_template_simple_7args(b: &mut Bencher) {
    let format_string_simple_7args = "Some string {} sth {}{} {} {}{}{}.".to_string();
    let compiled_template_simple_7args = Template::parse(&format_string_simple_7args).unwrap();
    let args = [
        "simple arg 1",
        "simple arg 2",
        "simple arg 3",
        "simple arg 4",
        "simple arg 5",
        "simple arg 6",
        "simple arg 7",
    ];
    b.iter(|| {
        compiled_template_simple_7args
            .arguments()
            .builder()
            .display(0, &args[0])
            .display(1, &args[1])
            .display(2, &args[2])
            .display(3, &args[3])
            .display(4, &args[4])
            .display(5, &args[5])
            .display(6, &args[6])
            .format()
            .unwrap();
    });
}

#[bench]
fn manual_via_template_unchecked_simple_1arg(b: &mut Bencher) {
    let format_string_simple_1arg = "Some string {}.".to_string();
    let compiled_template_simple_1arg = Template::parse(&format_string_simple_1arg).unwrap();
    let simple_arg1 = "simple arg 1";
    b.iter(|| {
        compiled_template_simple_1arg
            .arguments()
            .display_unchecked(0, &simple_arg1)
            .format()
            .unwrap();
    });
}

#[bench]
fn manual_via_template_unchecked_simple_7args(b: &mut Bencher) {
    let format_string_simple_7args = "Some string {} sth {}{} {} {}{}{}.".to_string();
    let compiled_template_simple_7args = Template::parse(&format_string_simple_7args).unwrap();
    let args = [
        "simple arg 1",
        "simple arg 2",
        "simple arg 3",
        "simple arg 4",
        "simple arg 5",
        "simple arg 6",
        "simple arg 7",
    ];
    b.iter(|| {
        compiled_template_simple_7args
            .arguments()
            .display_unchecked(0, &args[0])
            .display_unchecked(1, &args[1])
            .display_unchecked(2, &args[2])
            .display_unchecked(3, &args[3])
            .display_unchecked(4, &args[4])
            .display_unchecked(5, &args[5])
            .display_unchecked(6, &args[6])
            .format()
            .unwrap();
    });
}

#[bench]
fn dformat_complex(b: &mut Bencher) {
    let format_string_complex =
        "This is a {{{0:#x}}} {0:o} {3:?} {2:0w$} {1} {arg:.p$E} {:*^20}.".to_string();
    let compiled_template_complex = Template::parse(&format_string_complex).unwrap();
    let magic_number = 42;
    let float_number = 21.3;
    let bool_value = true;
    let arg1: &str = "random";
    let debug_struct = TestStruct {
        hello: 32,
        world: 42,
    };
    b.iter(|| {
        dformat!(
            compiled_template_complex,
            magic_number,
            bool_value,
            arg1,
            debug_struct,
            arg = float_number,
            p = 15,
            w = 25
        )
        .unwrap()
    });
}

#[bench]
fn dformat_unchecked_complex(b: &mut Bencher) {
    let format_string_complex =
        "This is a {{{0:#x}}} {0:o} {3:?} {2:0w$} {1} {arg:.p$E} {:*^20}.".to_string();
    let compiled_template_complex = Template::parse(&format_string_complex).unwrap();
    let magic_number = 42;
    let float_number = 21.3;
    let bool_value = true;
    let arg1: &str = "random";
    let debug_struct = TestStruct {
        hello: 32,
        world: 42,
    };
    b.iter(|| {
        dformat_unchecked!(
            compiled_template_complex,
            magic_number,
            bool_value,
            arg1,
            debug_struct,
            arg = float_number,
            p = 15,
            w = 25
        )
    });
}

#[bench]
fn format_complex(b: &mut Bencher) {
    let magic_number = 42;
    let float_number = 21.3;
    let bool_value = true;
    let arg1: &str = "random";
    let debug_struct = TestStruct {
        hello: 32,
        world: 42,
    };
    b.iter(|| {
        format!(
            "This is a {{{0:#x}}} {0:o} {3:?} {2:0w$} {1} {arg:.p$E} {:*^50}.",
            magic_number,
            bool_value,
            arg1,
            debug_struct,
            arg = float_number,
            p = 15,
            w = 25
        )
    });
}

#[bench]
fn manual_via_template_complex(b: &mut Bencher) {
    let format_string_complex =
        "This is a {{{0:#x}}} {0:o} {3:?} {2:0w$} {1} {arg:.p$E} {:*^20}.".to_string();
    let compiled_template_complex = Template::parse(&format_string_complex).unwrap();
    let magic_number = 42;
    let float_number = 21.3;
    let bool_value = true;
    let arg1: &str = "random";
    let debug_struct = TestStruct {
        hello: 32,
        world: 42,
    };
    b.iter(|| {
        compiled_template_complex
            .arguments()
            .builder()
            .display(0, &magic_number)
            .lower_hex(0, &magic_number)
            .octal(0, &magic_number)
            .display(1, &bool_value)
            .display(2, &arg1)
            .debug(3, &debug_struct)
            .upper_exp("arg", &float_number)
            .width_or_precision_amount("p", &15)
            .width_or_precision_amount("w", &25)
            .format()
            .unwrap();
    });
}

#[bench]
fn manual_via_template_unchecked_complex(b: &mut Bencher) {
    let format_string_complex =
        "This is a {{{0:#x}}} {0:o} {3:?} {2:0w$} {1} {arg:.p$E} {:*^20}.".to_string();
    let compiled_template_complex = Template::parse(&format_string_complex).unwrap();
    let magic_number = 42;
    let float_number = 21.3;
    let bool_value = true;
    let arg1: &str = "random";
    let debug_struct = TestStruct {
        hello: 32,
        world: 42,
    };
    b.iter(|| {
        compiled_template_complex
            .arguments()
            .display_unchecked(0, &magic_number)
            .lower_hex_unchecked(0, &magic_number)
            .octal_unchecked(0, &magic_number)
            .display_unchecked(1, &bool_value)
            .display_unchecked(2, &arg1)
            .debug_unchecked(3, &debug_struct)
            .upper_exp_unchecked("arg", &float_number)
            .width_or_precision_amount_unchecked("p", &15)
            .width_or_precision_amount_unchecked("w", &25)
            .format()
            .unwrap();
    });
}

fn main() {}
