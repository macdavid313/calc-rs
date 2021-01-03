use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Numeral(i64),
    Plus,
    Minus,
    Mul,
    Div,
    Lparen,
    Rparen,
}

pub fn lex(program: String) -> Vec<Token> {
    fn lex_numeral(chars: &mut Peekable<Chars>, accum: &mut Vec<char>) -> i64 {
        while let Some(&next_c) = chars.peek() {
            if next_c.is_ascii_digit() {
                accum.push(next_c);
                chars.next();
            } else {
                break;
            }
        }
        let s: String = accum.iter().collect();
        match s.parse::<i64>() {
            Ok(n) => n,
            Err(e) => panic!("{}: {}", s, e.to_string()),
        }
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = program.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            tokens.push(Token::Numeral(lex_numeral(&mut chars, &mut vec![c])));
        } else {
            match c {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Mul),
                '/' => tokens.push(Token::Div),
                '(' => tokens.push(Token::Lparen),
                ')' => tokens.push(Token::Rparen),
                _ => (),
            }
        }
    }
    tokens
}
