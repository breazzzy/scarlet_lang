use std::{collections::HashMap, vec};

use crate::{expression::{Expression, Symbol}, statement::Statement};

pub struct Resolver{
    scope_stack : Vec<HashMap<String, bool>>,
    pub lex_scope : HashMap<u64, usize>,
}

impl Resolver{
    pub fn new() -> Resolver{
        Resolver{scope_stack: vec![], lex_scope : HashMap::new()}
    }

    pub fn block_expr(&mut self, block : crate::expression::Expression){
        self.begin_scope();
        if let Expression::BlockExpr(stmts) = block{
            self.resolve_stmts(stmts);
        }
        self.end_scope();
    }

    pub fn decl_stmt(&mut self, sym : Symbol, init : Option<Expression>){
        self.declare(sym.name.clone());
        if let Some(expr) = init {
            self.resolve_expr(expr);
        }
        self.define(sym.name.clone());
    }

    fn begin_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scope_stack.pop();
    }

    pub fn resolve(&mut self, stmts: Vec<Statement>){
        self.begin_scope();
        self.resolve_stmts(stmts);
        self.end_scope();
    }

    pub fn resolve_stmts(&mut self, stmts: Vec<crate::statement::Statement>) {
        // self.begin_scope();
        for stmt in stmts{
            self.resolve_stmt(stmt);
        }
        // self.end_scope();
    }

    fn resolve_stmt(&mut self, stmt: crate::statement::Statement) {
        match stmt {
            crate::statement::Statement::Expression(expr) => self.resolve_expr(expr.clone()),
            crate::statement::Statement::Declaration(sym, init) => self.decl_stmt(sym, init),
            crate::statement::Statement::Assignment(sym, expr) => self.assign_stmt(sym, expr),
            crate::statement::Statement::FuncDclaration(name, params, expr) => self.function_declaration(name,params,expr),
            crate::statement::Statement::Return(expr) => self.return_stmt(expr),
        }
    }

    fn resolve_expr(&mut self, expr: Expression) {
        match expr {
            Expression::Binary(left, _, right) => self.binary(left,right),
            Expression::Logical(left, _, right) => self.logical(left,right),
            Expression::Unary(op, expr) => self.unary(expr),
            Expression::Literal(_) => self.literal(),
            Expression::Grouping(expr) => self.grouping(expr),
            Expression::Ternary(condition, then, elses) => self.ternary(*condition,*then,*elses),
            Expression::Primary(sym) => self.var_expr(sym),
            Expression::Call(callee, _, args) => self.call(callee,args),
            Expression::BlockExpr(_) => self.block_expr(expr),
            Expression::IfExpr(condition, then, _else) => self.if_expr(condition,then,_else),
            Expression::LoopExpr(body) => self.loop_expr(*body),
            Expression::WhileExpr(condition, body) => self.while_expr(condition,body),
            Expression::BreakExpr => {},
            Expression::ContinueExpr => {},
        }
    }

    fn declare(&mut self, name: String) {
        if self.scope_stack.is_empty() {
            return;
        }
        let scope  = self.scope_stack.last_mut().expect("[Resolve Error] declare");// Really living up to better errors huh?
        scope.insert(name, false);
    }

    fn define(&mut self, name: String) {
        if self.scope_stack.is_empty(){
            return;
        }
        self.scope_stack.last_mut().expect("[Resolve Error] define").insert(name, true);
    }

    fn var_expr(&mut self, sym: Symbol) {
        // if !self.scope_stack.is_empty() && self.scope_stack.last().expect("[Resolve Error] assign").get(&sym.name).expect("[Resolve Error] getting assign") == &false{
        //     panic!("[Resolve Error] Cant read local variable in its own initializer.")
        // }
        self.resolve_local(sym);
    }

    fn resolve_local(&mut self, sym: Symbol) {
        for (i,val) in self.scope_stack.iter().rev().enumerate(){
            if val.contains_key(&sym.name.clone()){
                self.lex_scope.insert(sym.s_id, i);
                return;
            }else{
                continue;
            }
        }
    }

    fn assign_stmt(&mut self, sym: Symbol, expr: Expression) {
        self.resolve_expr(expr);
        self.resolve_local(sym);
    }

    fn function_declaration(&mut self, name: Symbol, params: Vec<Symbol>, expr: Expression) {
        self.declare(name.name.clone());
        self.define(name.name.clone());
        let stmts;
        if let Expression::BlockExpr(b_stmts) = expr {
            stmts = b_stmts;
        }else{
            panic!("[Resolve Error] Function Declratation");
        }

        self.begin_scope();
        for param in params{
            self.declare(param.name.clone());
            self.define(param.name.clone());
        }
        self.resolve_stmts(stmts);
        self.end_scope();
    }

    fn if_expr(&mut self, condition: Box<Expression>, then: Box<Expression>, _else: Box<Option<Expression>>) {
        self.resolve_expr(*condition);
        self.resolve_expr(*then);
        if let Some(els) = *_else{
            self.resolve_expr(els);
        }
    }

    fn return_stmt(&mut self, expr: Expression) {
        self.resolve_expr(expr);
    }

    fn while_expr(&mut self, condition: Box<Expression>, body: Box<Expression>) {
        self.resolve_expr(*condition);
        self.resolve_expr(*body);
    }

    fn binary(&mut self, left: Box<Expression>, right: Box<Expression>) {
        self.resolve_expr(*left);
        self.resolve_expr(*right);
    }

    fn call(&mut self, callee: Box<Expression>, args: Vec<Expression>) {
        self.resolve_expr(*callee);
        for expr in args{
            self.resolve_expr(expr);
        }
    }

    fn grouping(&mut self, expr: Box<Expression>) {
        self.resolve_expr(*expr);
    }

    fn literal(&mut self) {
        return;
    }

    fn logical(&mut self, left: Box<Expression>, right: Box<Expression>) {
        self.resolve_expr(*left);
        self.resolve_expr(*right);
    }

    fn unary(&mut self, expr: Box<Expression>) {
        self.resolve_expr(*expr);
    }

    fn ternary(&mut self, condition: Expression, then: Expression, elses: Expression) {
        self.resolve_expr(condition);
        self.resolve_expr(then);
        self.resolve_expr(elses);
    }

    fn loop_expr(&mut self, body: Expression) {
        self.resolve_expr(body);
    }





}