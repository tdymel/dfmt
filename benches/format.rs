use criterion::{Criterion, criterion_group, criterion_main};
use dyf::{FormatString, dformat as dyf_dformat};
use dyn_str_fmt::{
    ArgumentsBuilder, Template, UncheckedArgumentsBuilder, dformat, dformat_unchecked,
};

#[derive(Debug)]
pub struct TestStruct {
    pub hello: i32,
    pub world: i32,
}

fn format_perf(c: &mut Criterion) {
    // let some_i32: i32 = 42;

    // c.bench_function("format! i32", |b| {
    //     b.iter(|| format!("{}", some_i32));
    // });

    // c.bench_function("lib i32", |b| {
    //     b.iter(|| format(some_i32));
    // });

    // c.bench_function("compile dyf", |b| {
    //     b.iter(|| FormatString::from_string("This is a {{{arg1:0>+#05.5?}}} example. The magic number is {}. Float = {}. Bool = {}. DebugStruct = {:?}.".to_owned()).unwrap());
    // });

    let compiled_format_string = FormatString::from_string(
        "This is a {{{}}} example. The magic number is {}. Float = {}. Bool = {}.".to_owned(),
    )
    .unwrap();
    let format_string = "This is a {{{0}}} {1} {2} {3} {4:?}.".to_string();
    let compiled_template = Template::parse_str(format_string.as_str()).unwrap();

    let magic_number = 42;
    let float_number = 21.3;
    let bool_value = true;
    let arg1 = "random";
    let debug_struct = TestStruct {
        hello: 32,
        world: 42,
    };
    // c.bench_function("compile pest parser", |b| {
    //     b.iter(|| Template::from(format_string.as_ref()));
    // });

    // c.bench_function("parse_naive", |b| {
    //     b.iter(|| parse_pieces(format_string.as_ref()));
    // });

    // c.bench_function("parser nom", |b| {
    //     b.iter(|| parse_format(format_string.as_ref()));
    // });

    // c.bench_function("compile dyf", |b| {
    //     b.iter(|| FormatString::from_string(format_string.to_string()));
    // });

    // c.bench_function("dyf format", |b| {
    //     b.iter(|| {
    //         dyf_dformat!(
    //             &compiled_format_string,
    //             "random",
    //             magic_number,
    //             float_number,
    //             bool_value
    //         )
    //         .unwrap()
    //     });
    // });

    c.bench_function("format via macro", |b| {
        b.iter(|| {
            dformat_unchecked!(
                compiled_template,
                arg1,
                magic_number,
                float_number,
                bool_value,
                debug_struct
            )
        });
    });

    c.bench_function("format via template", |b| {
        b.iter(|| {
            compiled_template
                .arguments()
                .display_unchecked(0, &arg1)
                .display_unchecked(1, &magic_number)
                .display_unchecked(2, &float_number)
                .display_unchecked(3, &bool_value)
                .debug_unchecked(4, &debug_struct)
                .format()
                .unwrap();
        });
    });

    c.bench_function("format!", |b| {
        b.iter(|| {
            format!(
                "This is a {{{0}}} {1} {2} {3} {4:?}.",
                arg1, magic_number, float_number, bool_value, debug_struct
            )
        });
    });

    // c.bench_function("compile", |b| {
    //     b.iter(|| {
    //         CompiledTemplate::compile(
    //             "This is an {{{arg1:0>+#05.5?}}}{:0>+#05.*p} example {0:0>+#0*.5E}.",
    //         )
    //     });
    // });
}

criterion_group!(benches, format_perf);
criterion_main!(benches);
