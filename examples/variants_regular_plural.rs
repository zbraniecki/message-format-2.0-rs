use message_format::ast::*;

pub fn main() {
    // No items selected.
    // 1 item selected.
    // 2 items selected.
    // 5 items selected.
    let _en = Message(MessageValue::Multi {
        selector: InlineExpression::FunctionReference {
            id: Identifier("PLURAL".into()),
            arguments: vec![InlineExpression::VariableReference(Identifier(
                "itemsCount".into(),
            ))],
        },
        variants: vec![
            Variant {
                key: VariantKey::NumberLiteral("0".into()),
                value: Pattern(vec![PatternElement::Text("No items selected.".into())]),
            },
            Variant {
                key: VariantKey::StringLiteral("one".into()),
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "itemsCount".into(),
                    ))),
                    PatternElement::Text(" item selected.".into()),
                ]),
            },
            Variant {
                key: VariantKey::StringLiteral("other".into()),
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "itemsCount".into(),
                    ))),
                    PatternElement::Text(" items selected.".into()),
                ]),
            },
        ],
    });

    // Brak zaznaczonych elementów.
    // 1 zaznaczony element.
    // 2 zaznaczone elementy.
    // 5 zaznaczonych elementów.
    let _pl = Message(MessageValue::Multi {
        selector: InlineExpression::FunctionReference {
            id: Identifier("PLURAL".into()),
            arguments: vec![InlineExpression::VariableReference(Identifier(
                "itemsCount".into(),
            ))],
        },
        variants: vec![
            Variant {
                key: VariantKey::NumberLiteral("0".into()),
                value: Pattern(vec![PatternElement::Text(
                    "Brak zaznaczonych elementów.".into(),
                )]),
            },
            Variant {
                key: VariantKey::StringLiteral("one".into()),
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "itemsCount".into(),
                    ))),
                    PatternElement::Text(" zaznaczony element.".into()),
                ]),
            },
            Variant {
                key: VariantKey::StringLiteral("few".into()),
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "itemsCount".into(),
                    ))),
                    PatternElement::Text(" zaznaczone elementy.".into()),
                ]),
            },
            Variant {
                key: VariantKey::StringLiteral("other".into()),
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "itemsCount".into(),
                    ))),
                    PatternElement::Text(" zaznaczonych elementów.".into()),
                ]),
            },
        ],
    });
}
