#![feature(test)]

extern crate test;

use dfmt::{ArgumentKey, Template};
use test::Bencher;

#[bench]
fn auto_deref_inference(b: &mut Bencher) {
    let template = Template::parse("Hello, {world} {arg1} {0} {1} {2} {3} {4}!").unwrap();
    let mut args = template.arguments();
    let argument_key = ArgumentKey::Name("world".to_string());
    let requirements = args
        .template
        .argument_type_requirements(&argument_key)
        .unwrap();

    b.iter(|| {
        // args.add_argument_value(argument_key.clone(), dfmt::ArgumentValue::Display(&"world")).unwrap();
        // dfmt::__internal__dfmt_process!(true, args, argument_key.clone(), "world").unwrap();
        dfmt::__internal__dfmt_black_magic!(
            true,
            requirements,
            display,
            args,
            argument_key.clone(),
            "world",
            Display,
            core::fmt::Display
        )
        .unwrap();
        args.clear();

        // args.template
        //     .argument_type_requirements(&argument_key)
        //     .unwrap();
        Ok(()) as Result<(), dfmt::Error>
    });
}
