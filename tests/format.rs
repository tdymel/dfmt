#![feature(formatting_options)]

use dyn_str_fmt::{Template, dformat, dformat_unchecked};

#[test]
fn black_magic4() {
    let str_template = "Black magic: {{{test:+^+#0width$.5b}}}.";
    let precompiled_template = Template::parse_str(str_template).unwrap();

    println!(
        "Str template: {}",
        dformat_unchecked!(str_template, test = 42, width = 50)
    );
    println!(
        "Precompiled template: {}",
        dformat!(precompiled_template, test = 42, width = 50).unwrap()
    );
    println!(
        "format! macro under the hood: {}",
        dformat!("Black magic: {test:+^+#0width$.5b}", test = &42, width = 50).unwrap()
    );
}

#[test]
fn experimental_formatter() {
    let mut result = String::new();
    let mut formatter = core::fmt::FormattingOptions::new()
        .fill('*')
        .align(Some(core::fmt::Alignment::Center))
        .alternate(true)
        .sign(Some(core::fmt::Sign::Plus))
        .sign_aware_zero_pad(true)
        .width(Some(100))
        .precision(Some(10))
        .create_formatter(&mut result);

    let test = 42;
    let t_dyn: &dyn core::fmt::Debug = &test;
    t_dyn.fmt(&mut formatter).unwrap();

    println!("Experimental Formatter: {}", result);
}
