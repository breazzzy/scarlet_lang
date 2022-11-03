use crate::{
    expression::{Expression, Symbol},
    statement::Statement,
    token::{Literal, Token, TokenType},
};
use std::vec;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    inloop: bool, // Used for break expression
    infunction: bool,
}

/*
    Precedence;
    Statements; Declaration;

    Expressions;
    Block Expression;

*/

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            inloop: false,
            infunction: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmts: Vec<Statement> = vec![];
        while !self.end_of_file() {
            stmts.push(self.declaration().expect("Statement not valid parsing"));
        }
        // println!("{:?}", stmts);
        return Ok(stmts);
    }

    fn statement(&mut self) -> Result<Statement, String> {
        // if self.matcher(TokenType::Break){
        //     self.consume(TokenType::Semicolon)?;
        //     return Ok(Statement::Break);
        // }

        if self.matcher(TokenType::Return){
            let expr = self.expression();
            self.consume(TokenType::Semicolon)?;
            return Ok(Statement::Return(expr));
        }

        return self.expression_statement();
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

    // let x = y
    fn declaration(&mut self) -> Result<Statement, String> {
        if self.matcher(TokenType::Let) {
            return self.declare_var();
        }
        if self.matcher(TokenType::Fun){
            return self.declare_fun();
        }
        return self.assignment();
        // return self.statement();
    }

    // x=y
    fn assignment(&mut self) -> Result<Statement, String> {
        if self.peek().token_type == TokenType::Identifier
            && self.peek_next().token_type == TokenType::Assignment
        {
            return self.assign_var();
        }
        return self.statement();
    }

    // 2+2
    fn expression(&mut self) -> Expression {
        return self.while_expr();
    }

    // fn while_statement(&mut self) -> Result<Statement, String> {
    //     let condition = self.expression();
    //     let body = self.statement()?;
    //     let mut stmts = vec![];
    //     match body {
    //         Statement::Block(sts) => stmts.extend(sts),
    //         _ => stmts.push(body),
    //     }

    //     return Ok(Statement::While(condition, stmts));
    // }
    //while
    fn while_expr(&mut self) -> Expression {
        if self.matcher(TokenType::While) {
            self.inloop = true;
            let condition = self.expression();
            // println!("{:?}", condition);
            let body = self.expression();
            // println!("{:?}", body);

            self.inloop = false;
            return Expression::WhileExpr(Box::new(condition), Box::new(body));
        } else {
            return self.loop_expr();
        }
    }

    //loop
    fn loop_expr(&mut self) -> Expression {
        if self.matcher(TokenType::Loop){
        self.inloop = true;
        let body = self.expression();
            self.inloop = false;

            return Expression::LoopExpr(Box::new(body));
    }else {
        return self.if_expr();
    }
    }

    //If
    pub fn if_expr(&mut self) -> Expression {
        if self.matcher(TokenType::If) {
            let p = self.expression();
            let then = self.expression();
            let mut else_s: Option<Expression> = None;
            if self.matcher(TokenType::Else) {
                else_s = Some(self.expression());
            }
            return Expression::IfExpr(Box::new(p), Box::new(then), Box::new(else_s));
        } else {
            return self.block();
        }
    }

    // {}
    pub fn block(&mut self) -> Expression {
        let mut stmts: Vec<Statement> = vec![];

        if self.matcher(TokenType::LeftSquigly) {
            while !self.check(TokenType::RightSquigly) && !self.end_of_file() {
                match self.declaration() {
                    Ok(stmt) => stmts.push(stmt),
                    Err(err) => panic!("{}", err),
                }
            }
            self.consume(TokenType::RightSquigly)
                .expect("Error ending block, } missing");
            return Expression::BlockExpr(stmts);
        } else {
            return self.ternary();
        }
    }
    //?
    fn ternary(&mut self) -> Expression {
        let ident: Expression = self.or();
        if self.matcher(TokenType::Ternary) {
            let r0 = self.expression();
            let _ = self.consume(TokenType::Colon);
            let r1 = self.expression();
            return Expression::Ternary(Box::new(ident), Box::new(r0), Box::new(r1));
        }
        return ident;
    }
    //or
    fn or(&mut self) -> Expression {
        let mut expr = self.and();
        while self.matcher(TokenType::Or) {
            let operator = self.previous();
            let right = self.and();
            expr = Expression::Logical(Box::new(expr), operator, Box::new(right));
        }

        return expr;
    }
    //and
    fn and(&mut self) -> Expression {
        let mut expr = self.equality();
        while self.matcher(TokenType::And) {
            let operator = self.previous();
            let right = self.equality();
            expr = Expression::Logical(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    // x == y
    fn equality(&mut self) -> Expression {
        let mut expr: Expression = self.comparison();
        while self.matcher(TokenType::NotEqual) || self.matcher(TokenType::Equality)
        /*|| self.matcher(TokenType::Assignment)*/
        {
            let operator: Token = self.previous();
            let right: Expression = self.comparison();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right))
        }
        return expr;
    }
    // x>y
    fn comparison(&mut self) -> Expression {
        let mut expr: Expression = self.binary();
        // println!("Token # {}" , self.current);
        while self.matcher(TokenType::Greater)
            || self.matcher(TokenType::GreaterEqual)
            || self.matcher(TokenType::Less)
            || self.matcher(TokenType::LessEqual)
        {
            let operator: Token = self.previous();
            let right: Expression = self.binary();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    // See if next token equals t, if so return true and pop to next token.
    fn matcher(&mut self, t: TokenType) -> bool {
        if self.check(t) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    // Get previous token
    fn previous(&self) -> Token {
        return self.tokens.get(self.current - 1).unwrap().clone();
    }

    // Check to see if next token is t but dont move to the next token
    fn check(&self, t: TokenType) -> bool {
        if self.end_of_file() {
            return false;
        };
        return self.peek().token_type == t;
    }
    // Move to next token
    fn advance(&mut self) -> Token {
        if !self.end_of_file() {
            self.current += 1;
        }
        return self.previous();
    }

    // Returns true if the current token is TERMINATE (The last token of every file)
    fn end_of_file(&self) -> bool {
        return self.peek().token_type == TokenType::TERMINATE;
    }

    // Get current token
    fn peek(&self) -> Token {
        return self.tokens.get(self.current).unwrap().clone();
    }

    //Check to see at end of file
    fn peek_next(&self) -> Token {
        return self.tokens.get(self.current + 1).unwrap().clone();
    }

    // This function only exists for clarity
    // self.term and self.factor are both binary expressions they are seperated for purposes of the order of operations
    // This function just makes the order a little more clear
    fn binary(&mut self) -> Expression {
        self.term()
    }

    // 5+5 // 5-5
    fn term(&mut self) -> Expression {
        let mut expr: Expression = self.factor();

        while self.matcher(TokenType::Minus) || self.matcher(TokenType::Plus) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    // 8*8  // 64/8
    fn factor(&mut self) -> Expression {
        let mut expr: Expression = self.unary();
        while self.matcher(TokenType::Slash) || self.matcher(TokenType::Aster) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }
    // !true // -x
    fn unary(&mut self) -> Expression {
        if self.matcher(TokenType::Not) || self.matcher(TokenType::Minus) {
            let operator = self.previous();
            let right = self.unary();
            return Expression::Unary(operator, Box::new(right));
        } else {
            return self.call();
        }
    }

    //Function calling
    fn call(&mut self) -> Expression {
        let mut expr = self.primary();
        loop {
            if self.matcher(TokenType::LeftParen) {
                expr = self.finish_call(expr);
                // println!("{:?}", expr);
            } else {
                break;
            }
        }

        expr
    }

    fn finish_call(&mut self, callee: Expression) -> Expression {
        let mut args: Vec<Expression> = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                let a = self.expression();
                args.push(a);
                if !self.matcher(TokenType::Comma) {
                    break;
                }
            }
        }
        let token = self
            .consume(TokenType::RightParen)
            .expect("Error: ')' expected at end of args.");
        return Expression::Call(Box::new(callee), token, args);
    }

    // Bottom of tree all literals, parenthesis and identifiers.
    fn primary(&mut self) -> Expression {
        // println!("{:?}",self.peek().token_type);
        if self.matcher(TokenType::False) {
            return Expression::Literal(Literal::False);
        };
        if self.matcher(TokenType::True) {
            return Expression::Literal(Literal::True);
        };
        if self.matcher(TokenType::Nil) {
            return Expression::Literal(Literal::Nil);
        };
        if self.matcher(TokenType::Number) || self.matcher(TokenType::String) {
            return Expression::Literal(self.previous().literal.unwrap());
        }
        if self.matcher(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen)
                .expect("Expect ')' after expression.");
            return Expression::Grouping(Box::new(expr));
        }
        if self.matcher(TokenType::Identifier) {
            return Expression::Primary(Symbol {
                name: self.previous().lex,
            });
        }
        if self.matcher(TokenType::Break) {
            if self.inloop {
                return Expression::BreakExpr;
            } else {
                panic!(
                    "@Line {}: Break only allowed in loops",
                    self.tokens[self.current].line
                )
            }
        }
        
        // if self.matcher(TokenType::Return) {
        //     if self.infunction {
        //         return Expression::ReturnExpr(Box::new(self.expression()));
        //     } else {
        //         panic!(
        //             "@Line {}: Return only allowed in functions",
        //             self.tokens[self.current].line
        //         )
        //     }
        // }
        // if self.matcher(TokenType::Identifier){
        //     return
        // }
        else {
            panic!(
                "@Line {}:Unexpected token {:?}",
                self.tokens[self.current].line,
                self.peek().token_type
            );
        }
    }

    // Returns next token if it is t. Other wise returns an error. Used when we know what the next token must be
    // For example if we have a declaration the code should look like
    // let IDENTIFIER = EXPRESSION;
    // After identifier we know the next token should be =. (The Assignment token type)
    // if self.constime(TokenType::Assignment [=]) dosn't return true there must be a syntax error on the user's end
    fn consume(&mut self, t: TokenType) -> Result<Token, String> {
        if self.check(t) {
            return Ok(self.advance());
        }

        Err(format!(
            "[Parser Error] Cant consume {:?} @Line {}",
            t, self.tokens[self.current].line
        ))
    }

    //The interpreter reads statments
    // Statment for print
    // fn print_statement(&mut self) -> Result<Statement, String> {
    //     let ex = self.expression();
    //     self.consume(TokenType::Semicolon)
    //         .expect("Expect ; after statement");
    //     return Ok(Statement::Print(ex));
    // }
    // Statment for expression
    fn expression_statement(&mut self) -> Result<Statement, String> {
        let ex = self.expression();
        match ex {
            Expression::BlockExpr(_) => (),
            Expression::IfExpr(_, _, _) => (),
            Expression::WhileExpr(_, _) => (),
            _ => (),
            // _ => _ = self.consume(TokenType::Semicolon)?,
        }
        if self.check(TokenType::Semicolon){
            self.consume(TokenType::Semicolon);
        }
        // self.consume(TokenType::Semicolon)
        //     .expect("; Expected after expression");
        return Ok(Statement::Expression(ex));
    }

    //Statment used for variable declaration
    fn declare_var(&mut self) -> Result<Statement, String> {
        let name: Token = self
            .consume(TokenType::Identifier)
            .expect("Expect identifier after let");

        let mut init: Option<Expression> = None;
        if self.matcher(TokenType::Assignment) {
            let ex = self.expression();
            init = Some(ex);
        }

        self.consume(TokenType::Semicolon)
            .expect("Expect ; after variable declaration");

        return Ok(Statement::Declaration(Symbol { name: name.lex }, init));
    }
    //Statment used for variable assignment
    fn assign_var(&mut self) -> Result<Statement, String> {
        let name: Token = self
            .consume(TokenType::Identifier)
            .expect("Error on parsing assignment");
        self.consume(TokenType::Assignment)
            .expect("Expect = for assignment expression");
        let expr = self.expression();
        match expr {
            Expression::BlockExpr(_) => (),
            Expression::IfExpr(_, _, _) => (),
            Expression::WhileExpr(_, _) => (),
            _ => {
                _ = self
                    .consume(TokenType::Semicolon)
                    .expect("Expect ; after variable assignment")
            }
        }

        return Ok(Statement::Assignment(Symbol { name: name.lex }, expr));
    }

    fn declare_fun(&mut self) -> Result<Statement, String> {
        let name = self.consume(TokenType::Identifier)?;
        _ = self.consume(TokenType::LeftParen)?;
        let mut params : Vec<Symbol> = vec![];
        if !self.check(TokenType::RightParen){
        loop {
            params.push(Symbol{name : self.consume(TokenType::Identifier)?.lex});
            if !self.matcher(TokenType::Comma){
                break;
            }
        }}
        self.consume(TokenType::RightParen)?;

        let body = self.block();

        return Ok(Statement::FuncDclaration(Symbol { name: name.lex }, params, body))
    }
}
