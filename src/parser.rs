use crate::token::{Token, TokenType, Literal};

pub struct Parser{
    tokens : Vec<Token>,
    current : usize,
}

impl Parser{
    pub fn new(tokens : Vec<Token>) -> Parser{
        Parser { tokens: tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expr{
        return self.expression();
    }

    fn synchronize(&mut self){
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon{return;}
            match self.peek().token_type {
                TokenType::Class => todo!(),
                TokenType::Fun => todo!(),
                TokenType::Return => return,
                _ => panic!("Yo wtf"),
            }

            self.advance();
        }
    }

    fn expression(&mut self) -> Expr{
        return self.comparison();
    }

    fn equality(&mut self) -> Expr{
        let mut expr : Expr = self.comparison();
        while self.matcher(TokenType::NotEqual) || self.matcher(TokenType::Equality) {
            let operator : Token = self.previous();
            let right : Expr = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr : Expr = self.term();
        println!("Token # {}" , self.current);
        while self.matcher(TokenType::Greater) || self.matcher(TokenType::GreaterEqual) || self.matcher(TokenType::Less) || self.matcher(TokenType::LessEqual) {
            let operator : Token = self.previous();
            let right : Expr = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn matcher(&mut self, t: TokenType) -> bool {
        if self.check(t){
            self.advance();
            return true;
        }else{
            return false;
        }
    }

    fn previous(&self) -> Token {
        return self.tokens.get(self.current - 1).unwrap().clone();
    }

    fn check(&self, t : TokenType) -> bool {
        if(self.is_at_end()) {return false};
        return self.peek().token_type == t;
    }
    
    fn advance(&mut self) -> Token {
        if(!self.is_at_end()) {self.current+=1;}
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::TERMINATE;
    }

    fn peek(&self) -> Token {
        return self.tokens.get(self.current).unwrap().clone();
    }

    fn term(&mut self) -> Expr {
        let mut expr : Expr = self.factor();

        while self.matcher(TokenType::Minus) || self.matcher(TokenType::Plus){
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr : Expr = self.unary();
        while  self.matcher(TokenType::Slash) || self.matcher(TokenType::Aster){
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.matcher(TokenType::Not) || self.matcher(TokenType::Minus){
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }else{return self.primary();}
    }

    fn primary(&mut self) -> Expr {
        println!("{:?}",self.peek().token_type);
        if self.matcher(TokenType::False) {return Expr::Literal(Literal::False)};
        if self.matcher(TokenType::True) {return Expr::Literal(Literal::True)};
        if self.matcher(TokenType::Nil) {return Expr::Literal(Literal::Nil)};
        if self.matcher(TokenType::Number) || self.matcher(TokenType::String){
            return Expr::Literal(self.previous().literal.unwrap());
        }
        if self.matcher(TokenType::LeftParen){
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(Box::new(expr));
        }
        else{
            panic!("Wtf is {:?}", self.peek().token_type);
        }
    }

    fn consume(&mut self, t: TokenType, msg: &str) -> Token {
        if(self.check(t)) {return self.advance();}

        panic!("{}", msg);
    }

}


pub enum Expr{
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Literal),
    Grouping(Box<Expr>),
}