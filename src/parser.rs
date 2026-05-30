use std::process;
use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    IntLit(i64),
}

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
}

#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut tokens = tokens.iter().peekable();
    let mut stmts = vec![];

    while let Some(token) = tokens.next() {
        match token {
            Token::Return => {
                let expr = match tokens.next() {
                    Some(Token::IntLit(val)) => Expr::IntLit(*val),
                    _ => {
                        eprintln!("Expected expression after return");
                        process::exit(1);
                    }
                };
                if let Some(Token::Semi) = tokens.next() {} else {
                    eprintln!("Expected semicolon");
                    process::exit(1);
                }
                stmts.push(Stmt::Return(expr));
            }
            _ => {}
        }
    }

    Program { stmts }
}