use super::value::Value;

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(String),
    Op(String),
    Bool(String),
    Eval(String),
    Var(String),
    IDE(String),
    ENV,
    ERR,
    COMMA,
    PS,
    PE,
    SS,
    SE,
    IF,
    IN,
    FUN,
    LET,
    THEN,
    ELSE,
    EQ,
    ARROW,
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
    pub fn consume_val(&mut self) -> Value {
        match self.peek() {
            Some(token) => match token {
                Token::Int(_) => Value::Num(self.consume_num()),
                Token::Bool(_) => Value::Bool(self.consume_bool()),
                _ => panic!("unexpected"),
            },
            None => panic!("unexpected"),
        }
    }
    pub fn consume_num(&mut self) -> i32 {
        let token = self.pop().expect("");
        match token {
            Token::Int(val) => val.parse().expect(""),
            _ => panic!(""),
        }
    }
    pub fn consume_var(&mut self) -> String {
        let token = self.pop().expect("");
        match token {
            Token::Var(val) => val,
            _ => panic!(""),
        }
    }
    pub fn consume_bool(&mut self) -> String {
        let token = self.pop().expect("");
        match token {
            Token::Bool(val) => val,
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
        let token_patterns = vec![
            ("MINT", r"-[1-9][0-9]*"),
            ("INT", r"[1-9][0-9]*"),
            ("BOOL", r"(true|false)"),
            ("ERR", r"error"),
            ("ARROW", r"->"),
            ("ENV", r"\|-"),
            ("OP", r"\+|-|\*|<"),
            ("PS", r"\("),
            ("PE", r"\)"),
            ("SS", r"\["),
            ("SE", r"\]"),
            ("EQ", r"="),
            ("COMMA", r","),
            ("IF", r"if"),
            ("THEN", r"then"),
            ("FUN", r"fun"),
            ("ELSE", r"else"),
            ("LET", r"let"),
            ("IN", r"in"),
            ("VAR", r"(x|y|f|sq|sm)"),
            ("EVAL", r"evalto"),
        ];
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

        for caps in self.re.captures_iter(&code) {
            let mut typ = String::from("nil");
            let val = String::from(&caps[0]);
            for name in &self.names {
                if caps.name(name).is_some() {
                    typ = name.to_string();
                }
            }
            match typ.as_ref() {
                "MINT" => tokens.push(Token::Int(val)),
                "INT" => tokens.push(Token::Int(val)),
                "BOOL" => tokens.push(Token::Bool(val)),
                "ERR" => tokens.push(Token::ERR),
                "ENV" => tokens.push(Token::ENV),
                "VAR" => tokens.push(Token::Var(val)),
                "OP" => tokens.push(Token::Op(val)),
                "EVAL" => tokens.push(Token::Eval(val)),
                "FUN" => tokens.push(Token::FUN),
                "PS" => tokens.push(Token::PS),
                "PE" => tokens.push(Token::PE),
                "SS" => tokens.push(Token::SS),
                "SE" => tokens.push(Token::SE),
                "EQ" => tokens.push(Token::EQ),
                "ARROW" => tokens.push(Token::ARROW),
                "COMMA" => tokens.push(Token::COMMA),
                "IF" => tokens.push(Token::IF),
                "IN" => tokens.push(Token::IN),
                "LET" => tokens.push(Token::LET),
                "THEN" => tokens.push(Token::THEN),
                "ELSE" => tokens.push(Token::ELSE),
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
