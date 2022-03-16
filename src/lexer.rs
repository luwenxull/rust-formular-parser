use crate::token::Token;
use regex::Regex;

pub struct Lexer {
    pos: usize,
    chars: Vec<char>,
    cell_reg: Regex,
    // letters_map: String,
    // digits_reg: Regex,
    current: Option<char>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            pos: 0,
            chars: vec![],
            cell_reg: Regex::new(r"^\$?[A-Za-z]+\$?\d+$").unwrap(),
            // letters_map: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$".to_string(),
            // digits_reg: Regex::new(r"[0-9]").unwrap(),
            current: None
        }
    }
}

impl Lexer {
    fn reset(&mut self, input: String) {
        self.chars = input.chars().collect();
        self.pos = 0;
        self.current = match self.chars.get(self.pos) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current = match self.chars.get(self.pos) {
            Some(c) => Some(*c),
            None => None,
        }
    }

    pub fn make_tokens(&mut self, input: String) -> Result<Vec<Token>, String> {
        self.reset(input);
        let mut tokens = Vec::with_capacity(self.chars.len());
        while let Some(c) = self.current {
            if c == ' ' {
                self.advance();
                continue;
            } else if c == '(' {
                tokens.push(Token::Lparen);
                self.advance();
            } else if c == ')' {
                tokens.push(Token::Rparen);
                self.advance();
            } else if c == '+' {
                tokens.push(Token::Plus);
                self.advance();
            } else if c == '-' {
                tokens.push(Token::Minus);
                self.advance();
            } else if c == '*' {
                tokens.push(Token::Mul);
                self.advance();
            } else if c == '/' {
                tokens.push(Token::Div);
                self.advance();
            } else if c == ',' {
                tokens.push(Token::Comma);
                self.advance();
            } else if c == ':' {
                tokens.push(Token::Colon);
                self.advance();
            } else if c == '=' {
                tokens.push(Token::Ee);
                self.advance();
            } else if c == '!' {
                tokens.push(Token::Csref);
                self.advance();
            } else if c == '&' {
                tokens.push(Token::And);
                self.advance();
            } else if c == '<' {
                tokens.push(self.make_lt());
            } else if c == '>' {
                tokens.push(self.make_gt());
            } else if c == '"' || c == '\'' {
                match self.make_str(c) {
                    Ok(token) => tokens.push(token),
                    Err(reason) => return Err(reason),
                }
            } else if c.is_digit(10) {
                tokens.push(self.make_num()?)
            } else if c == '$' || c.is_ascii_alphabetic() {
                tokens.push(self.make_identifier());
            } else {
                return Err(format!("Unexpected character: {}", c));
            }
        }
        Ok(tokens)
    }

    fn make_num(&mut self) -> Result<Token, String> {
        let mut num_str = String::new();
        let mut dot_count = 0;
        while let Some(c) = self.current {
            if c.is_digit(10) {
                num_str.push(c);
                self.advance();
            } else if c == '.' {
                if dot_count != 0 {
                    return Err(format!("Invalid number format: {}", num_str));
                }
                dot_count += 1;
                num_str.push(c);
                self.advance();
            } else if c == 'E' || c == 'e' {
                self.advance();
                let mut f = 1_f32;
                if let Some(c) = self.current {
                    if c == '-' || c == '+' {
                        if c == '-' {
                            f = -1_f32;
                        }
                        self.advance();
                    }
                    if let Token::Number(n) = self.make_num()? {
                        return Ok(Token::Number(
                            num_str.parse::<f32>().unwrap() * (10_f32).powf(f * n),
                        ));
                    }
                    return Err(format!("Invalid number format: {}", num_str));
                } else {
                    return Err(format!("Invalid number format: {}", num_str));
                }
            } else if c == '%' {
                self.advance();
                return Ok(Token::Number(num_str.parse::<f32>().unwrap() / 100_f32));
            } else {
                break;
            }
        }
        return Ok(Token::Number(num_str.parse::<f32>().unwrap()));
    }

    fn make_identifier(&mut self) -> Token {
        let mut ident_str = String::new();
        while let Some(c) = self.current {
            if c == '$' || c.is_ascii_alphabetic()
                || (ident_str != "" && c.is_digit(10))
            {
                ident_str.push(c);
                self.advance();
            } else {
                break;
            }
        }
        if ident_str == "TRUE" {
            return Token::Bool(true);
        } else if ident_str == "FALSE" {
            return Token::Bool(false);
        } else if let Some(c) = self.current {
            if c == '!' {
                return Token::Sheet(ident_str);
            }
        }
        if self.cell_reg.is_match(&ident_str) {
            return Token::Ref(ident_str.replace("$", ""));
        } else {
            return Token::Var(ident_str);
        }
    }

    fn make_gt(&mut self) -> Token {
        // >
        self.advance();
        if let Some(c) = self.current {
            // >=
            if c == '=' {
                self.advance();
                return Token::Gte;
            } else {
                return Token::Gt;
            }
        } else {
            return Token::Gt;
        }
    }

    fn make_lt(&mut self) -> Token {
        // <
        self.advance();
        if let Some(c) = self.current {
            // <=
            if c == '=' {
                self.advance();
                return Token::Lte;
            } else if c == '>' {
                // <>
                self.advance();
                return Token::Ne;
            } else {
                return Token::Lt;
            }
        } else {
            return Token::Lt;
        }
    }

    fn make_str(&mut self, end: char) -> Result<Token, String> {
        let mut str_str = String::new();
        self.advance(); // skip leading char
        while let Some(c) = self.current {
            if c == end {
                self.advance();
                if end == '"' {
                    return Ok(Token::String(str_str));
                } else {
                    return Ok(Token::Sheet(str_str));
                }
            } else {
                str_str.push(c);
                self.advance();
            }
        }
        return Err(format!("Invalid string format: {}", str_str));
    }
}
