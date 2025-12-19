#![feature(formatting_options)]

use dfmt::{Template, dformat, dformat_unchecked};

#[test]
fn black_magic4() {
    let str_template = "Black magic: {{{test:+^+#0width$.5b}}}.";
    let precompiled_template = Template::parse(str_template).unwrap();

    println!("Template: {}", precompiled_template.to_string());

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
