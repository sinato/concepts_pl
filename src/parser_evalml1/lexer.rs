use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(String),
    Op(String),
    Eval(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}
impl Tokens {
    pub fn pop(&mut self) -> Option<Token> {
        self.tokens.reverse();
        let token = self.tokens.pop();
        self.tokens.reverse();
        token
    }
    pub fn peek(&self) -> Option<Token> {
        let mut tokens = self.clone();
        tokens.pop()
    }
    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }
    pub fn reverse(&mut self) {
        self.tokens.reverse()
    }
    pub fn consume_num(&mut self) -> i32 {
        let token = self.pop().expect("");
        match token {
            Token::Int(val) => val.parse().expect(""),
            _ => panic!(""),
        }
    }
    pub fn consume_op(&mut self) -> String {
        let token = self.pop().expect("");
        match token {
            Token::Op(op) => op,
            _ => panic!(""),
        }
    }
}

pub struct Lexer {
    re: Regex,
    names: Vec<&'static str>,
}

impl Lexer {
    // static constructor
    pub fn new() -> Lexer {
        let token_patterns = vec![("INT", r"[0-9]"), ("OP", r"\+|-"), ("EVAL", r"evalto")];
        let re = make_regex(&token_patterns);
        let names = get_names(&token_patterns);
        let re = Regex::new(&re).expect("something went wrong making the regex");
        Lexer { re, names }
    }
    pub fn lex(&self, code: String) -> Tokens {
        let mut code = code;
        let tokens = self.tokenize(&mut code);
        tokens
    }
    fn tokenize(&self, code: &mut String) -> Tokens {
        let mut tokens: Vec<Token> = Vec::new();

        for (i, caps) in self.re.captures_iter(&code).enumerate() {
            let mut typ = String::from("nil");
            let val = String::from(&caps[0]);
            for name in &self.names {
                if caps.name(name).is_some() {
                    typ = name.to_string();
                }
            }
            match typ.as_ref() {
                "INT" => tokens.push(Token::Int(val)),
                "OP" => tokens.push(Token::Op(val)),
                "EVAL" => tokens.push(Token::Eval(val)),
                _ => panic!("unexpected type token"),
            }
        }
        Tokens { tokens }
    }
}
fn make_regex(token_patterns: &Vec<(&str, &str)>) -> String {
    token_patterns
        .into_iter()
        .map(|pattern| format!("(?P<{}>{})", pattern.0, pattern.1))
        .collect::<Vec<String>>()
        .join("|")
}
fn get_names<'a, 'b>(token_patterns: &Vec<(&'a str, &'b str)>) -> Vec<&'a str> {
    token_patterns
        .into_iter()
        .map(|pattern| pattern.0)
        .collect()
}
