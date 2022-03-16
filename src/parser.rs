use crate::{node::ASTNode, token::Token, utils::some};
use std::cell::Cell;
// #[derive(Debug)]
pub struct Parser {
    pos: Cell<usize>,
    tokens: Vec<Token>,
    compare_op: Vec<Token>,
    and_op: Vec<Token>,
    arith_op: Vec<Token>,
    term_op: Vec<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            pos: Cell::new(0),
            tokens: vec![],
            compare_op: vec![
                Token::Ee,
                Token::Ne,
                Token::Gt,
                Token::Lt,
                Token::Gte,
                Token::Lte,
            ],
            and_op: vec![Token::And],
            arith_op: vec![Token::Plus, Token::Minus],
            term_op: vec![Token::Mul, Token::Div],
        }
    }
}

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<ASTNode, String> {
        self.tokens = tokens;
        self.pos.set(0);
        self.compare_expr()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos.get())
    }

    fn advance(&self) {
        self.pos.set(self.pos.get() + 1);
    }

    fn compare_expr(&self) -> Result<ASTNode, String> {
        let mut node = self.and_expr()?;
        while let Some(token) = self.current() {
            if some(&self.compare_op, |t| t.matches(token)) {
                self.advance();
                node = ASTNode::BinaryOp(token.clone(), Box::new(node), Box::new(self.and_expr()?));
            } else {
                break;
            }
        }
        return Ok(node);
    }

    fn and_expr(&self) -> Result<ASTNode, String> {
        let mut node = self.arith_expr()?;
        while let Some(token) = self.current() {
            if some(&self.and_op, |t| t.matches(token)) {
                self.advance();
                node =
                    ASTNode::BinaryOp(token.clone(), Box::new(node), Box::new(self.arith_expr()?));
            } else {
                break;
            }
        }
        return Ok(node);
    }

    fn arith_expr(&self) -> Result<ASTNode, String> {
        let mut node = self.term_expr()?;
        while let Some(token) = self.current() {
            if some(&self.arith_op, |t| t.matches(token)) {
                self.advance();
                node =
                    ASTNode::BinaryOp(token.clone(), Box::new(node), Box::new(self.term_expr()?));
            } else {
                break;
            }
        }
        return Ok(node);
    }

    fn term_expr(&self) -> Result<ASTNode, String> {
        let mut node = self.factor_expr()?;
        while let Some(token) = self.current() {
            if some(&self.term_op, |t| t.matches(token)) {
                self.advance();
                node =
                    ASTNode::BinaryOp(token.clone(), Box::new(node), Box::new(self.factor_expr()?));
            } else {
                break;
            }
        }
        return Ok(node);
    }

    fn factor_expr(&self) -> Result<ASTNode, String> {
        if let Some(token) = self.current() {
            if token.matches(&Token::Plus) {
                self.advance();
                return Ok(ASTNode::SignedNumber(1_f32, Box::new(self.range_expr()?)));
            } else if token.matches(&Token::Minus) {
                self.advance();
                return Ok(ASTNode::SignedNumber(-1_f32, Box::new(self.range_expr()?)));
            } else {
                return Ok(self.range_expr()?);
            }
        } else {
            return Err("Unexpected EOF".to_string());
        }
    }

    fn range_expr(&self) -> Result<ASTNode, String> {
        let left = self.atom_expr()?;
        match self.current() {
            Some(token) => {
                if token.matches(&Token::Colon) {
                    self.advance(); // 跳过 :
                    match left {
                        ASTNode::Var(_) | ASTNode::Number(_) => self.make_row_or_col_range(left, None),
                        ASTNode::UndeterminedRange(sheet, node) => self.make_row_or_col_range(*node, Some(sheet)),
                        ASTNode::Ref(_, _) => match self.atom_expr()? {
                            right @ ASTNode::Ref(_, _) => {
                                Ok(ASTNode::Range(Box::new(left), Box::new(right)))
                            }
                            _ => return Err("Range not valid. The right side of the range should be a cell reference".to_string()),
                        },
                        _ => return Err("Range not valid. The left side of the range should be a row reference or a column reference or a cell reference".to_string()),
                    }
                } else {
                    return Ok(left);
                }
            }
            None => Ok(left),
        }
    }

    fn atom_expr(&self) -> Result<ASTNode, String> {
        match self.current() {
            Some(token) => match token {
                Token::Number(n) => {
                    self.advance();
                    Ok(ASTNode::Number(*n))
                }
                Token::String(s) => {
                    self.advance();
                    Ok(ASTNode::String(s.clone()))
                }
                Token::Bool(b) => {
                    self.advance();
                    Ok(ASTNode::Bool(*b))
                }
                Token::Ref(r) => {
                    self.advance();
                    Ok(ASTNode::Ref(r.clone(), None))
                }
                Token::Lparen => self.sub_expr(),
                Token::Var(var) => {
                    self.advance();
                    match self.current() {
                        Some(token) => {
                            if token.matches(&Token::Lparen) {
                                self.call_expr(var.clone())
                            } else {
                                Ok(ASTNode::Var(var.clone()))
                            }
                        }
                        None => Ok(ASTNode::Var(var.clone())),
                    }
                }
                Token::Sheet(sheet) => {
                    self.advance();
                    match self.current() {
                        Some(token) => match token {
                            Token::Csref => {
                                self.advance();
                                match self.current() {
                                    Some(token) => match token {
                                        Token::Ref(r) => {
                                            self.advance();
                                            Ok(ASTNode::Ref(r.clone(), Some(sheet.clone())))
                                        }
                                        Token::Var(v) => {
                                            self.advance();
                                            Ok(ASTNode::UndeterminedRange(
                                                sheet.clone(),
                                                Box::new(ASTNode::Var(v.clone())),
                                            ))
                                        }
                                        Token::Number(n) => {
                                            self.advance();
                                            Ok(ASTNode::UndeterminedRange(
                                                sheet.clone(),
                                                Box::new(ASTNode::Number(*n)),
                                            ))
                                        }
                                        _ => Err("Not valid cross sheet reference".to_string()),
                                    },
                                    None => Err("Unexpected EOF".to_string()),
                                }
                            }
                            _ => Err("Expect '!'".to_string()),
                        },
                        None => Err("Unexpected EOF".to_string()),
                    }
                }
                _ => Err("Unexpected token".to_string()),
            },
            None => Err("Unexpected EOF".to_string()),
        }
    }

    fn sub_expr(&self) -> Result<ASTNode, String> {
        self.advance(); //跳过左括号
        let node = self.compare_expr()?;
        match self.current() {
            Some(Token::Rparen) => {
                self.advance();
                Ok(node)
            }
            _ => Err("Unmatched parenthesis".to_string()),
        }
    }

    fn call_expr(&self, name: String) -> Result<ASTNode, String> {
        self.advance();
        let mut args = vec![];
        loop {
            match self.current() {
                Some(token) => match token {
                    Token::Rparen => {
                        self.advance();
                        break Ok(ASTNode::Call(name, args));
                    }
                    Token::Comma => {
                        self.advance();
                        continue;
                    }
                    _ => args.push(self.compare_expr()?),
                },
                None => break Err("Unexpected EOF".to_string()),
            }
        }
    }

    /// 创建行或列范围
    fn make_row_or_col_range(
        &self,
        left: ASTNode,
        sheet: Option<String>,
    ) -> Result<ASTNode, String> {
        match left {
            ASTNode::Number(f) => match self.atom_expr()? {
                ASTNode::Number(t) => Ok(ASTNode::RowRange(f, t, sheet)),
                _ => Err("Range not valid. The right side of the range must be a row reference".to_string()),
            },
            ASTNode::Var(f) => match self.atom_expr()? {
                ASTNode::Var(t) => Ok(ASTNode::ColRange(f, t, sheet)),
                _ => Err("Range not valid. The right side of the range must be a col reference".to_string()),
            },
            _ => Err("Range not valid. The left side of the range must be a row reference or a column referencee".to_string()),
        }
    }
}
