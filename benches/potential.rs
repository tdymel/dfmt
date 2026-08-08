#![feature(test)]

extern crate test;

use test::Bencher;

#[bench]
fn simple_case_7_args(b: &mut Bencher) {
    let args = [
        "Test1", "Test2", "Test3", "Test4", "Test5", "Test6", "Test7",
    ];
    b.iter(|| {
        format!(
            "{}{}{}{}{}{}{}",
            args[0], args[1], args[2], args[3], args[4], args[5], args[6]
        )
    });
}

#[bench]
fn simple_case_7_args_named(b: &mut Bencher) {
    let args = [
        "Test1", "Test2", "Test3", "Test4", "Test5", "Test6", "Test7",
    ];
    b.iter(|| {
        format!(
            "{a}{b}{c}{d}{e}{f}{g}",
            a = args[0],
            b = args[1],
            c = args[2],
            d = args[3],
            e = args[4],
            f = args[5],
            g = args[6]
        )
    });
}

#[bench]
fn simple_case_7_args_indexed(b: &mut Bencher) {
    let args = [
        "Test1", "Test2", "Test3", "Test4", "Test5", "Test6", "Test7",
    ];
    b.iter(|| {
        format!(
            "{6}{0}{2}{1}{3}{5}{4}",
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
