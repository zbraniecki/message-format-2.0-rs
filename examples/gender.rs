use message_format::ast::*;

pub fn main() {
    // Anne published a post in Birthday Party
    // John published a post in Birthday Party
    let _en = Message(MessageValue::Single(Pattern(vec![
        PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
            "userName".into(),
        ))),
        PatternElement::Text(" published a post in ".into()),
        PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
            "groupName".into(),
        ))),
    ])));

    // Anne opublikowała post w grupie Birthday Party
    // John opublikował post w grupie Birthday Party
    let _pl = Message(MessageValue::Multi {
        selector: vec![InlineExpression::VariableReference(Identifier("userGender".into()))],
        variants: vec![
            Variant {
                key: vec![VariantKey::StringLiteral("feminine".into())],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowała post w grupie ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "groupName".into(),
                    ))),
                ]),
            },
            Variant {
                key: vec![VariantKey::StringLiteral("masculine".into())],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikował post w grupie ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "groupName".into(),
                    ))),
                ]),
            },
            Variant {
                key: vec![VariantKey::StringLiteral("neuter".into())],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowało post w grupie ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "groupName".into(),
                    ))),
                ]),
            },
        ],
    });
}
