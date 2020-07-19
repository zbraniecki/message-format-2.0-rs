use message_format::ast::*;

pub fn main() {
    // Anne and John published a post in Birthday Party
    // Anne and Mary published a post in Birthday Party
    // John and Mark published a post in Birthday Party
    let _en = Message(MessageValue::Single(Pattern(vec![
        PatternElement::Placeholder(InlineExpression::FunctionReference {
            id: Identifier("LIST".into()),
            arguments: vec![InlineExpression::VariableReference(Identifier(
                "userNames".into(),
            ))],
        }),
        PatternElement::Text(" published a post in ".into()),
        PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
            "groupName".into(),
        ))),
    ])));

    // Anne i John opublikowali post w grupie Birthday Party
    // Anne i Mary opublikowały post w grupie Birthday Party
    // John i Mark opublikowali post w grupie Birthday Party
    let _pl = Message(MessageValue::Multi {
        selector: vec![InlineExpression::FunctionReference {
            id: Identifier("GENDER".into()),
            arguments: vec![InlineExpression::VariableReference(Identifier(
                "usersGender".into(),
            ))],
        }],
        variants: vec![
            Variant {
                key: vec![VariantKey::StringLiteral("masculine-personal".into())],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowali post w grupie ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "groupName".into(),
                    ))),
                ]),
            },
            Variant {
                key: vec![VariantKey::StringLiteral("non-masculine-personal".into())],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowały post w grupie ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "groupName".into(),
                    ))),
                ]),
            },
        ],
    });
}
