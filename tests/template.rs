use dfmt::{AllowedSpecifier, AllowedSpecifierBuilder, ArgumentKey, Specifier, Template, Type};

#[test]
fn expect_argument_argument_not_found() {
    let template = Template::parse("Hello, {worl}!").unwrap();
    assert_eq!(
        template
            .expect_argument("world", &AllowedSpecifier::all())
            .err()
            .unwrap(),
        dfmt::Error::ArgumentNotFound(ArgumentKey::Name("world".to_string()))
    )
}

#[test]
fn expect_argument_argument_not_within_constraints() {
    let template = Template::parse("Hello, {world:?}!").unwrap();
    assert_eq!(
        template
            .expect_argument("world", &AllowedSpecifier::none())
            .err()
            .unwrap(),
        dfmt::Error::ArgumentNotWithinConstraints(
            ArgumentKey::Name("world".to_string()),
            Specifier::default().ty(dfmt::Type::Debug),
            Box::new(AllowedSpecifier::none())
        )
    )
}

#[test]
fn expect_argument_argument_not_within_constraints2() {
    let template = Template::parse("Hello, {world:?}!").unwrap();
    assert_eq!(
        template
            .expect_argument("world", &AllowedSpecifier::all().forbid(Type::Debug))
            .err()
            .unwrap(),
        dfmt::Error::ArgumentNotWithinConstraints(
            ArgumentKey::Name("world".to_string()),
            Specifier::default().ty(dfmt::Type::Debug),
            Box::new(AllowedSpecifier::all().forbid(Type::Debug))
        )
    )
}
