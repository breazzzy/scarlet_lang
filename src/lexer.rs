use std::{collections::HashMap, vec};

use crate::{
    token::{Literal, TokenType},
    Token,
};

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(src: &String) -> Lexer {
        let mut _keywords = HashMap::new();
        _keywords.insert("and".to_string(), TokenType::And);
        _keywords.insert("class".to_string(), TokenType::Class);
        _keywords.insert("else".to_string(), TokenType::Else);
        _keywords.insert("false".to_string(), TokenType::False);
        _keywords.insert("for".to_string(), TokenType::For);
        _keywords.insert("fun".to_string(), TokenType::Fun);
        _keywords.insert("loop".to_string(), TokenType::Loop);
        _keywords.insert("if".to_string(), TokenType::If);
        _keywords.insert("nil".to_string(), TokenType::Nil);
        _keywords.insert("or".to_string(), TokenType::Or);
        // words.insert("print".to_string(), TokenType::Print);
        _keywords.insert("return".to_string(), TokenType::Return);
        _keywords.insert("super".to_string(), TokenType::Super);
        _keywords.insert("this".to_string(), TokenType::This);
        _keywords.insert("true".to_string(), TokenType::True);
        _keywords.insert("let".to_string(), TokenType::Let);
        _keywords.insert("while".to_string(), TokenType::While);
        _keywords.insert("break".to_string(), TokenType::Break);
        _keywords.insert("continue".to_string(), TokenType::Continue);
        _keywords.insert("struct".to_string(), TokenType::Struct);
        Lexer {
            source: src.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: _keywords,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current > self.source.len() - 1
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::TERMINATE,
            literal: None,
            lex: "".to_string(),
            line: self.line,
        });
        // self.tokens.iter().map(|t| println!("{}", t));
        // println!("{:?}", self.tokens);
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftSquigly),
            '}' => self.add_token(TokenType::RightSquigly),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Aster),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            '/' => self.add_token(TokenType::Slash),
            '?' => self.add_token(TokenType::Ternary),
            ':' => self.add_token(TokenType::Colon),
            '!' => {
                if self.matcher('=') {
                    self.add_token(TokenType::NotEqual)
                } else {
                    self.add_token(TokenType::Not)
                };
            }
            '=' => {
                if self.matcher('=') {
                    self.add_token(TokenType::Equality)
                } else {
                    self.add_token(TokenType::Assignment)
                }
            }
            '<' => {
                if self.matcher('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.matcher('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '#' => {
                //Comments are #
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            'o' => {
                if self.peek() == 'r' {
                    self.add_token(TokenType::Or);
                    self.advance();
                }
            }
            'a' => {
                if self.peek() == 'n' && self.peek_next() == 'd' {
                    self.add_token(TokenType::And);
                    self.advance();
                    self.advance();
                } else {
                    self.identifier();
                }
            }
            '"' => self.string(),
            ' ' | '\r' | '\t' => {} // Do nothing with white space
            '\n' => self.line += 1,
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    panic!("Unexpected Charecter {} on line {}", c, self.line)
                };
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }
        if self.is_at_end() {
            //Error unterminated string
            return;
        }
        self.advance();
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_literal_token(TokenType::String, Some(Literal::Str(value)));
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        return self.source.chars().nth(self.current - 1).unwrap();
    }

    // pub fn advance_by(&mut self, n : usize) -> Vec<char>{
    //     let mut out = vec![];
    //     for i in 0..n{
    //         out.push(self.advance());
    //     }
    //     return out;
    // }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn matcher(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_literal_token(token_type, None);
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lex = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            literal,
            lex: lex.to_string(),
            line: self.line,
        })
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        self.add_literal_token(
            TokenType::Number,
            Some(Literal::Number(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            )),
        )
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    // fn is_alpha(&self, c: char) -> bool {
    //     return ('a' <= c && c >= 'z') || ('A' <= c && c >= 'Z') || c == '_';
    // }

    // fn is_digit(&self, c: char) -> bool {
    //     return c >= '0' && c <= '9';
    // }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let a = &self.source[self.start..self.current];
        let t = self.keywords.get(a);
        if let Some(istype) = t {
            self.add_token(*istype);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }
}
