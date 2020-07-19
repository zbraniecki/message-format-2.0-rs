use message_format::ast::*;

pub fn main() {
    // Anne published 1 picture.
    // John published 5 pictures.
    let _en = Message(MessageValue::Multi {
        selector: vec![InlineExpression::FunctionReference {
            id: Identifier("PLURAL".into()),
            arguments: vec![InlineExpression::VariableReference(Identifier(
                "numPictures".into(),
            ))],
        }],
        variants: vec![
            Variant {
                key: vec![VariantKey::StringLiteral("one".into())],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" published ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" picture.".into()),
                ]),
            },
            Variant {
                key: vec![VariantKey::StringLiteral("other".into())],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" published ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" pictures.".into()),
                ]),
            },
        ],
    });

    // Anne opublikowała 1 zdjęcie.
    // John opublikował 5 zdjęć.
    let _pl = Message(MessageValue::Multi {
        selector: vec![
            InlineExpression::FunctionReference {
                id: Identifier("GENDER".into()),
                arguments: vec![InlineExpression::VariableReference(Identifier(
                        "usersGender".into(),
                ))],
            },
            InlineExpression::FunctionReference {
                id: Identifier("PLURAL".into()),
                arguments: vec![InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                ))],
            },
        ],
        variants: vec![
            Variant {
                key: vec![
                    VariantKey::StringLiteral("masculine".into()),
                    VariantKey::StringLiteral("one".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikował ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcie.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("feminine".into()),
                    VariantKey::StringLiteral("one".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowała ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcie.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("neuter".into()),
                    VariantKey::StringLiteral("one".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowało ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcie.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("masculine".into()),
                    VariantKey::StringLiteral("few".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikował ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcia.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("feminine".into()),
                    VariantKey::StringLiteral("few".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowała ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcia.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("neuter".into()),
                    VariantKey::StringLiteral("few".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowało ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcia.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("masculine".into()),
                    VariantKey::StringLiteral("many".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikował ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcia.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("feminine".into()),
                    VariantKey::StringLiteral("many".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowała ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcia.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("neuter".into()),
                    VariantKey::StringLiteral("many".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowało ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęcia.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("masculine".into()),
                    VariantKey::StringLiteral("other".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikował ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęć.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("feminine".into()),
                    VariantKey::StringLiteral("other".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowała ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęć.".into()),
                ]),
            },
            Variant {
                key: vec![
                    VariantKey::StringLiteral("neuter".into()),
                    VariantKey::StringLiteral("other".into()),
                ],
                value: Pattern(vec![
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "userName".into(),
                    ))),
                    PatternElement::Text(" opublikowało ".into()),
                    PatternElement::Placeholder(InlineExpression::VariableReference(Identifier(
                        "numPictures".into(),
                    ))),
                    PatternElement::Text(" zdjęć.".into()),
                ]),
            },
        ],
    });
}
