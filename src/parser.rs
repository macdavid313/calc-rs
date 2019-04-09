use std::fmt;
use std::iter::Peekable;
use std::slice::Iter;

use crate::lexer::{lex, Token};

#[derive(Debug)]
enum Expr {
    Numeral(i64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Numeral(i) => write!(f, "{}", i),
            Expr::Plus(a, b) => write!(f, "{} + {}", a, b),
            Expr::Minus(a, b) => write!(f, "{} - {}", a, b),
            Expr::Mul(a, b) => write!(f, "{} * {}", a, b),
            Expr::Div(a, b) => write!(f, "{} / {}", a, b),
        }
    }
}

fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    fn parse_expr(iter: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
        match parse_term(iter) {
            Ok(mut t1) => loop {
                if let Some(&&token) = iter.peek() {
                    match token {
                        Token::Plus => {
                            iter.next();
                            match parse_term(iter) {
                                Ok(t2) => t1 = Expr::Plus(Box::new(t1), Box::new(t2)),
                                Err(e) => return Err(e),
                            }
                        }
                        Token::Minus => {
                            iter.next();
                            match parse_term(iter) {
                                Ok(t2) => t1 = Expr::Minus(Box::new(t1), Box::new(t2)),
                                Err(e) => return Err(e),
                            }
                        }
                        _ => return Ok(t1),
                    }
                } else {
                    return Ok(t1);
                }
            },
            Err(e) => Err(e),
        }
    }

    fn parse_term(iter: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
        match parse_factor(iter) {
            Ok(mut t1) => loop {
                if let Some(&&token) = iter.peek() {
                    match token {
                        Token::Mul => {
                            iter.next();
                            match parse_factor(iter) {
                                Ok(t2) => t1 = Expr::Mul(Box::new(t1), Box::new(t2)),
                                Err(e) => return Err(e),
                            }
                        }
                        Token::Div => {
                            iter.next();
                            match parse_factor(iter) {
                                Ok(t2) => t1 = Expr::Div(Box::new(t1), Box::new(t2)),
                                Err(e) => return Err(e),
                            }
                        }
                        _ => return Ok(t1),
                    }
                } else {
                    return Ok(t1);
                }
            },
            Err(e) => Err(e),
        }
    }

    fn parse_factor(iter: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
        if let Some(&token) = iter.next() {
            match token {
                Token::Numeral(n) => Ok(Expr::Numeral(n)),
                Token::Lparen => match parse_expr(iter) {
                    Ok(expr) => {
                        if let Some(next_token) = iter.next() {
                            match next_token {
                                Token::Rparen => Ok(expr),
                                _ => Err(String::from("Parsing error: ')' expected")),
                            }
                        } else {
                            Err(String::from("Parsing error: ')' expected"))
                        }
                    }
                    Err(e) => Err(format!("Parsing error: {}", e)),
                },
                _ => Err(String::from("Parsing error: Numeral or '(' expected")),
            }
        } else {
            Err(String::from("Parsing error: unexpected EOF"))
        }
    }

    if tokens.is_empty() {
        Err(String::from("Can't parse; no tokens."))
    } else {
        let mut iter = tokens.iter().peekable();
        parse_expr(&mut iter)
    }
}

fn eval_expr(expr: Expr) -> i64 {
    match expr {
        Expr::Numeral(a) => a,
        Expr::Plus(a, b) => eval_expr(*a) + eval_expr(*b),
        Expr::Minus(a, b) => eval_expr(*a) - eval_expr(*b),
        Expr::Mul(a, b) => eval_expr(*a) * eval_expr(*b),
        Expr::Div(a, b) => eval_expr(*a) / eval_expr(*b),
    }
}

pub fn eval(program: String) -> Result<i64, String> {
    match parse(lex(program)) {
        Ok(expr) => Ok(eval_expr(expr)),
        Err(e) => Err(e),
    }
}
