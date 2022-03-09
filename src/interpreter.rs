use crate::{lexer::Lexer, node::ASTNode, parser::Parser, token::Token};
pub struct Interpreter {
    lexer: Lexer,
    parser: Parser,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            lexer: Lexer::new(),
            parser: Parser::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CellPosition {
    pub sheet: String,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum ComputeResult {
    Number(f32),
    String(String),
    Bool(bool),
}

fn bigger_string(left: &String, right: &String) -> bool {
    let chs_r = right.chars().collect::<Vec<char>>();
    let chs_l = left.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < chs_l.len() {
        if let Some(ch) = chs_r.get(i) {
            if (chs_l[i] as u8) < (*ch as u8) {
                return true;
            } else if (chs_l[i] as u8) > (*ch as u8) {
                return false;
            }
        } else {
            return false;
        }
        i += 1;
    }
    return false;
}

impl ComputeResult {
    pub fn as_num(&self) -> Result<f32, String> {
        match self {
            Self::Number(n) => Ok(*n),
            Self::String(s) => match s.parse::<f32>() {
                Ok(v) => Ok(v),
                Err(_) => Err(format!("Cannot convert {} to number", s)),
            },
            _ => Err("Expect a number or string".to_string()),
        }
    }
}

impl Interpreter {
    pub fn compute(
        &mut self,
        input: String,
        position: CellPosition,
    ) -> Result<ComputeResult, String> {
        let tokens = self.lexer.make_tokens(input)?;
        let node = self.parser.parse(tokens)?;
        self.evaluate(&node, &position)
    }

    fn evaluate(&self, node: &ASTNode, position: &CellPosition) -> Result<ComputeResult, String> {
        match node {
            ASTNode::Number(num) => Ok(ComputeResult::Number(*num)),
            ASTNode::SignedNumber(sign, num) => match self.evaluate(num, position)? {
                ComputeResult::Number(num) => Ok(ComputeResult::Number(sign * num)),
                ComputeResult::String(str) => Ok(ComputeResult::String(format!("{}{}", sign, str))),
                _ => Err("Invalid number".to_string()),
            },
            ASTNode::String(str) => Ok(ComputeResult::String(str.clone())),
            ASTNode::Bool(bool) => Ok(ComputeResult::Bool(*bool)),
            ASTNode::BinaryOp(tp, left, right) => self.do_bin_op(tp, left, right, position),
            _ => Err("Not implemented".to_string()),
        }
    }

    fn do_bin_op(
        &self,
        token: &Token,
        left: &ASTNode,
        right: &ASTNode,
        position: &CellPosition,
    ) -> Result<ComputeResult, String> {
        let left = self.evaluate(left, position)?;
        let right = self.evaluate(right, position)?;
        match token {
            Token::Plus | Token::Minus | Token::Mul | Token::Div => {
                let left = left.as_num()?;
                let right = right.as_num()?;
                match token {
                    Token::Plus => Ok(ComputeResult::Number(left + right)),
                    Token::Minus => Ok(ComputeResult::Number(left - right)),
                    Token::Mul => Ok(ComputeResult::Number(left * right)),
                    Token::Div => Ok(ComputeResult::Number(left / right)),
                    _ => Err("Will never enter this arm".to_string()),
                }
            }
            Token::Gt | Token::Gte | Token::Lt | Token::Lte => {
                let (left, right) = match (left, right) {
                    (ComputeResult::Number(l), ComputeResult::Number(r)) => (l, r),
                    (ComputeResult::String(l), ComputeResult::String(r)) => {
                        if bigger_string(&l, &r) {
                            (0_f32, 1_f32)
                        } else if l == r {
                            (0_f32, 0_f32)
                        } else {
                            (1_f32, 0_f32)
                        }
                    }
                    (ComputeResult::String(_), ComputeResult::Number(_)) => (1_f32, 0_f32),
                    (ComputeResult::Number(_), ComputeResult::String(_)) => (0_f32, 1_f32),
                    _ => return Err("Not comparable".to_string()),
                };
                match token {
                    Token::Gt => Ok(ComputeResult::Bool(left > right)),
                    Token::Gte => Ok(ComputeResult::Bool(left >= right)),
                    Token::Lt => Ok(ComputeResult::Bool(left < right)),
                    Token::Lte => Ok(ComputeResult::Bool(left <= right)),
                    _ => Err("Will never enter this arm".to_string()),
                }
            }
            _ => Err("Should not enter this arm".to_string()),
        }
    }
}
