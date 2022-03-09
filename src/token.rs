use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f32),
    // SignedNumber,
    Plus,
    Minus,
    Mul,
    Div,
    Lparen,
    Rparen,
    // Eof,
    Ref(String),
    Var(String),
    Sheet(String),
    Ee,
    Ne,
    Lt,
    Gt,
    Lte,
    Gte,
    Comma,
    Colon,
    String(String),
    Csref,
    And,
    Bool(bool),
}

impl Token {
    pub fn matches(&self, token: &Token) -> bool {
        discriminant(self) == discriminant(token)
    }
}
