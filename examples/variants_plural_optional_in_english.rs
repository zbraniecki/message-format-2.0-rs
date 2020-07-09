use message_format::ast::*;

pub fn main() {
    // 1 selected
    // 2 selected
    // 5 selected
    let _en = Message(MessageValue::Single(Pattern(vec![
        PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
            "itemsCount".into(),
        ))),
        PatternElement::Text(" selected".into()),
    ])));

    // 1 zaznaczony
    // 2 zaznaczone
    // 5 zaznaczonych
    let _pl = Message(MessageValue::Multi {
        selector: InlineExpression::FunctionReference {
            id: Identifier("PLURAL".into()),
            arguments: vec![InlineExpression::VariableReference(Identifier(
                "itemsCount".into(),
            ))],
        },
        variants: vec![
            Variant {
                key: VariantKey::StringLiteral("one".into()),
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "itemsCount".into(),
                    ))),
                    PatternElement::Text(" zaznaczony".into()),
                ]),
            },
            Variant {
                key: VariantKey::StringLiteral("few".into()),
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "itemsCount".into(),
                    ))),
                    PatternElement::Text(" zaznaczone".into()),
                ]),
            },
            Variant {
                key: VariantKey::StringLiteral("other".into()),
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "itemsCount".into(),
                    ))),
                    PatternElement::Text(" zaznaczonych".into()),
                ]),
            },
        ],
    });
}
