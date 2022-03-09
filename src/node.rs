use crate::token::Token;

#[derive(Debug)]
pub enum ASTNode {
    Number(f32),
    SignedNumber(f32, Box<ASTNode>),
    String(String),
    Bool(bool),
    Var(String),
    Ref(String, Option<String>),
    BinaryOp(Token, Box<ASTNode>, Box<ASTNode>),
    Range(Box<ASTNode>, Box<ASTNode>),
    RowRange(f32, f32, Option<String>),
    ColRange(String, String, Option<String>),
    UndeterminedRange(String, Box<ASTNode>),
    Call(String, Vec<ASTNode>),
}
