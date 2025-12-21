use criterion::{Criterion, criterion_group, criterion_main};
use dfmt::{ArgumentsBuilder, Template, UncheckedArgumentsBuilder, dformat, dformat_unchecked};

#[derive(Debug)]
pub struct TestStruct {
    pub hello: i32,
    pub world: i32,
}

fn format_perf(c: &mut Criterion) {
    let format_string_simple_1arg = "Some string {}.".to_string();
    let format_string_simple_7args = "Some string {} sth {}{} {} {}{}{}.".to_string();
    let format_string_complex = "This is a {{{0:#x}}} {0:o} {3:?} {2:0w$} {1} {arg:.p$E} {:*^20}.".to_string();

    let compiled_template_simple_1arg = Template::parse(&format_string_simple_1arg).unwrap();
    let compiled_template_simple_7args = Template::parse(&format_string_simple_7args).unwrap();
    let compiled_template_complex = Template::parse(&format_string_complex).unwrap();

    let magic_number = 42;
    let float_number = 21.3;
    let bool_value = true;
    let arg1: &str = "random";
    let debug_struct = TestStruct {
        hello: 32,
        world: 42,
    };

    let simple_arg1 = "simple arg 1";
    let simple_arg2 = "simple arg 2";
    let simple_arg3 = "simple arg 3";
    let simple_arg4 = "simple arg 4";
    let simple_arg5 = "simple arg 5";
    let simple_arg6 = "simple arg 6";
    let simple_arg7 = "simple arg 7";

    // Template parsing
    c.bench_function("Template::parse - simple - 1 arg", |b| {
        b.iter(|| Template::parse(&format_string_simple_1arg));
    });

    c.bench_function("Template::parse - simple - 7 arg", |b| {
        b.iter(|| Template::parse(&format_string_simple_7args));
    });

    c.bench_function("Template::parse - complex", |b| {
        b.iter(|| Template::parse(&format_string_complex));
    });

    // Simple case
    c.bench_function("dformat! - simple - 1 arg", |b| {
        b.iter(|| dformat!(compiled_template_simple_1arg, simple_arg1).unwrap());
    });

    c.bench_function("dformat! - simple - 7 args", |b| {
        b.iter(|| {
            dformat!(
                compiled_template_simple_7args,
                simple_arg1,
                simple_arg2,
                simple_arg3,
                simple_arg4,
                simple_arg5,
                simple_arg6,
                simple_arg7
            )
            .unwrap()
        });
    });

    c.bench_function("dformat_unchecked! - simple - 1 arg", |b| {
        b.iter(|| dformat_unchecked!(compiled_template_simple_1arg, simple_arg1));
    });

    c.bench_function("dformat_unchecked! - simple - 7 args", |b| {
        b.iter(|| {
            dformat_unchecked!(
                compiled_template_simple_7args,
                simple_arg1,
                simple_arg2,
                simple_arg3,
                simple_arg4,
                simple_arg5,
                simple_arg6,
                simple_arg7
            )
        });
    });

    c.bench_function("format! - simple - 1 arg", |b| {
        b.iter(|| format!("Some string {}.", simple_arg1));
    });

    c.bench_function("format! - simple - 7 args", |b| {
        b.iter(|| {
            format!(
                "Some string {} sth {}{} {} {}{}{}.",
                simple_arg1,
                simple_arg2,
                simple_arg3,
                simple_arg4,
                simple_arg5,
                simple_arg6,
                simple_arg7
            )
        });
    });

    c.bench_function("Manual via template - simple - 1 arg", |b| {
        b.iter(|| {
            compiled_template_simple_1arg
                .arguments()
                .builder()
                .display(0, &simple_arg1)
                .format()
                .unwrap();
        });
    });

    c.bench_function("Manual via template - simple - 7 args", |b| {
        b.iter(|| {
            compiled_template_simple_7args
                .arguments()
                .builder()
                .display(0, &simple_arg1)
                .display(1, &simple_arg2)
                .display(2, &simple_arg3)
                .display(3, &simple_arg4)
                .display(4, &simple_arg5)
                .display(5, &simple_arg6)
                .display(6, &simple_arg7)
                .format()
                .unwrap();
        });
    });

    c.bench_function("Manual via template unchecked - simple - 1 arg", |b| {
        b.iter(|| {
            compiled_template_simple_1arg
                .arguments()
                .display_unchecked(0, &simple_arg1)
                .format()
                .unwrap();
        });
    });

    c.bench_function("Manual via template unchecked - simple - 7 args", |b| {
        b.iter(|| {
            compiled_template_simple_7args
                .arguments()
                .display_unchecked(0, &simple_arg1)
                .display_unchecked(1, &simple_arg2)
                .display_unchecked(2, &simple_arg3)
                .display_unchecked(3, &simple_arg4)
                .display_unchecked(4, &simple_arg5)
                .display_unchecked(5, &simple_arg6)
                .display_unchecked(6, &simple_arg7)
                .format()
                .unwrap();
        });
    });

    // Complex
    c.bench_function("dformat! - complex", |b| {
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
    });

    c.bench_function("dformat_unchecked! - complex", |b| {
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
    });

    c.bench_function("format! - complex", |b| {
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
    });

    c.bench_function("Manual via template - complex", |b| {
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
    });

    c.bench_function("Manual via template unchecked - complex", |b| {
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
    });
}

criterion_group!(benches, format_perf);
criterion_main!(benches);
