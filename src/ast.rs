pub struct Message(pub MessageValue);

pub enum MessageValue {
    Single(Pattern),
    Multi {
        selector: Vec<InlineExpression>,
        variants: Vec<Variant>,
    },
}

pub struct Pattern(pub Vec<PatternElement>);

pub enum PatternElement {
    Text(String),
    Placeholder(InlineExpression),
}

pub struct Variant {
    pub key: Vec<VariantKey>,
    pub value: Pattern,
}

pub enum VariantKey {
    StringLiteral(String),
    NumberLiteral(String),
}

pub enum InlineExpression {
    StringLiteral(String),
    NumberLiteral(String),
    FunctionReference {
        id: Identifier,
        arguments: Vec<InlineExpression>,
    },
    VariableReference(Identifier),
}

pub struct Identifier(pub String);
