use std::vec;

use crate::{token::{Token, TokenType, Literal}, statement::Statement, expression::{Expression, Symbol}};

pub struct Parser{
    tokens : Vec<Token>,
    current : usize,
}

impl Parser{
    pub fn new(tokens : Vec<Token>) -> Parser{
        Parser { tokens: tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String>{
        let mut stmts : Vec<Statement> = vec![];
        while(!self.end_of_file()){
            stmts.push(self.declaration().expect("Statement not valid parsing"));
        }
        return Ok(stmts);
    }

    // fn synchronize(&mut self){
    //     self.advance();
    //     while !self.end_of_file() {
    //         if self.previous().token_type == TokenType::Semicolon{return;}
    //         match self.peek().token_type {
    //             TokenType::Class => todo!(),
    //             TokenType::Fun => todo!(),
    //             TokenType::Return => return,
    //             _ => panic!("Yo wtf"),
    //         }

    //         self.advance();
    //     }
    // }

    fn declaration(&mut self) -> Result<Statement, String>{
        if self.matcher(TokenType::Let) {return self.declare_var();}
        return self.statement();
    }

    fn statement(&mut self) -> Result<Statement, String>{
        if(self.matcher(TokenType::Print)){
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn expression(&mut self) -> Expression{
        return self.comparison();
    }

    fn equality(&mut self) -> Expression{
        let mut expr : Expression = self.comparison();
        while self.matcher(TokenType::NotEqual) || self.matcher(TokenType::Equality) {
            let operator : Token = self.previous();
            let right : Expression = self.comparison();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right))
        }
        return expr;
    }

    fn comparison(&mut self) -> Expression {
        let mut expr : Expression = self.term();
        // println!("Token # {}" , self.current);
        while self.matcher(TokenType::Greater) || self.matcher(TokenType::GreaterEqual) || self.matcher(TokenType::Less) || self.matcher(TokenType::LessEqual) {
            let operator : Token = self.previous();
            let right : Expression = self.term();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
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
        if(self.end_of_file()) {return false};
        return self.peek().token_type == t;
    }
    
    fn advance(&mut self) -> Token {
        if(!self.end_of_file()) {self.current+=1;}
        return self.previous();
    }

    fn end_of_file(&self) -> bool {
        return self.peek().token_type == TokenType::TERMINATE;
    }

    fn peek(&self) -> Token {
        return self.tokens.get(self.current).unwrap().clone();
    }

    fn term(&mut self) -> Expression {
        let mut expr : Expression = self.factor();

        while self.matcher(TokenType::Minus) || self.matcher(TokenType::Plus){
            let operator = self.previous();
            let right = self.factor();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn factor(&mut self) -> Expression {
        let mut expr : Expression = self.unary();
        while  self.matcher(TokenType::Slash) || self.matcher(TokenType::Aster){
            let operator = self.previous();
            let right = self.unary();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn unary(&mut self) -> Expression {
        if self.matcher(TokenType::Not) || self.matcher(TokenType::Minus){
            let operator = self.previous();
            let right = self.unary();
            return Expression::Unary(operator, Box::new(right));
        }else{return self.primary();}
    }

    fn primary(&mut self) -> Expression {
        // println!("{:?}",self.peek().token_type);
        if self.matcher(TokenType::False) {return Expression::Literal(Literal::False)};
        if self.matcher(TokenType::True) {return Expression::Literal(Literal::True)};
        if self.matcher(TokenType::Nil) {return Expression::Literal(Literal::Nil)};
        if self.matcher(TokenType::Number) || self.matcher(TokenType::String){
            return Expression::Literal(self.previous().literal.unwrap());
        }
        if self.matcher(TokenType::LeftParen){
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expression::Grouping(Box::new(expr));
        }
        if self.matcher(TokenType::Identifier){
            return Expression::Variable(Symbol{name: self.previous().lex });
        }
        // if self.matcher(TokenType::Identifier){
        //     return 
        // }
        else{
            panic!("Wtf is {:?}", self.peek().token_type);
        }
    }

    fn consume(&mut self, t: TokenType, msg: &str) -> Token {
        if(self.check(t)) {return self.advance();}

        panic!("{}", msg);
    }

    // Statment for print
    fn print_statement(&mut self) -> Result<Statement, String> {
        let ex = self.expression();
        self.consume(TokenType::Semicolon, "Expect ; after statement");
        return Ok(Statement::Print(ex));
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let ex = self.expression();
        self.consume(TokenType::Semicolon, "; Expected after expression");
        return Ok(Statement::Expression(ex));
    }

    fn declare_var(&mut self) -> Result<Statement, String> {
        let name : Token = self.consume(TokenType::Identifier, "Expect identifier after let");

        let mut init : Option<Expression> = None;
        if self.matcher(TokenType::Assignment){ // = 
            let ex = self.expression();
            init = Some(ex);
        }

        self.consume(TokenType::Semicolon, "Expect ; after variable declaration");

        return Ok(Statement::Declaration(Symbol{name: name.lex}, init));
    }

}


