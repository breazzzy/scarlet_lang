use std::fmt::{self};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Ternary, //?
    Colon,
    LeftParen,
    RightParen,
    Comma,
    Dot,
    Minus,
    Semicolon,
    Slash,
    Plus,
    Aster,

    //
    Not,
    NotEqual,
    Assignment,
    Equality,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    //LITERALS
    Identifier,
    String,
    Number,

    //KEYWORDS
    And,
    Class,
    Else,
    False,
    True,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Let,
    While,
    LeftSquigly,  // {}
    RightSquigly, // }
    //
    TERMINATE,
}

#[derive(Debug, Clone)]
pub enum Literal {
    // Identifier(String),
    Str(String),
    Number(f64),
    True,
    False,
    Nil,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<Literal>,
    pub lex: String,
    pub line: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token_type {
            TokenType::String => write!(
                f,
                "Token {:?}({:?})",
                self.token_type,
                self.literal.as_ref().unwrap()
            ),
            TokenType::Number => write!(
                f,
                "Token {:?}({:?})",
                self.token_type,
                self.literal.as_ref().unwrap()
            ),
            _ => write!(f, "Token {:?}", self.token_type),
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {:?}, {:?}",
            self.token_type,
            self.literal.as_ref().unwrap()
        )
    }
}
