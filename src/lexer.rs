use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct DebugInfo {
    pub start: usize,
    pub end: usize,
    pub s: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Zero(DebugInfo),
    Equal(DebugInfo),
    ParenS(DebugInfo),
    Ps(DebugInfo),
    Pe(DebugInfo),
    Op(String, DebugInfo),
    OpC(String, DebugInfo),
    Lt(DebugInfo),
    Eval(DebugInfo),
    EvalMR(DebugInfo),
}
impl Token {
    pub fn get_debug_info(self, filename: &str) -> String {
        let debug_info = match self.clone() {
            Token::Lt(d)
            | Token::Eval(d)
            | Token::EvalMR(d)
            | Token::Zero(d)
            | Token::Equal(d)
            | Token::ParenS(d)
            | Token::Pe(d)
            | Token::Ps(d) => d,
            Token::Op(_, d) | Token::OpC(_, d) => d,
        };
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let contents_to_err = String::from(&contents[0..debug_info.end]);
        let num: usize = contents_to_err.split("\n").count();
        let splited_contents: Vec<&str> = contents.split("\n").collect();
        format!("{}:{}:{}", filename, num + 1, splited_contents[num])
    }
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
    pub fn consume_peano_num(&mut self) -> usize {
        let mut now_cnt = 0;
        let mut cnt = 0;
        while let Some(token) = self.pop() {
            match token {
                Token::Ps(_) => {
                    now_cnt += 1;
                    cnt += 1;
                }
                Token::Pe(_) => now_cnt -= 1,
                Token::Zero(_) => {
                    if cnt == 0 {
                        break;
                    }
                }
                _ => (),
            }
            if now_cnt == 0 {
                break;
            }
        }
        cnt
    }
    pub fn consume_operator(&mut self) -> String {
        if let Some(Token::Op(op, _)) = self.pop() {
            op
        } else {
            panic!("expects an operator")
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
            ("ZERO", r"Z"),
            ("OP", r"(plus)|(times)"),
            ("OPC", r"\+|\*"),
            ("LT", r"is less than"),
            ("EQ", r"is"),
            ("PS", r"S\("),
            ("PARENS", r"\("),
            ("PE", r"\)"),
            ("EVAL", r"evalto"),
            ("EVALMR", r"-\*->"),
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

        // get token's location for setting debugging info
        let mut locations: Vec<DebugInfo> = Vec::new();
        for mat in self.re.find_iter(&code) {
            let location = DebugInfo {
                start: mat.start(),
                end: mat.end(),
                s: mat.as_str().to_string(),
            };
            locations.push(location);
        }
        for (i, caps) in self.re.captures_iter(&code).enumerate() {
            let mut typ = String::from("nil");
            let val = String::from(&caps[0]);
            for name in &self.names {
                if caps.name(name).is_some() {
                    typ = name.to_string();
                }
            }
            let debug_info = locations.clone().remove(i);
            match typ.as_ref() {
                "ZERO" => tokens.push(Token::Zero(debug_info)),
                "OP" => tokens.push(Token::Op(val, debug_info)),
                "OPC" => tokens.push(Token::OpC(val, debug_info)),
                "LT" => tokens.push(Token::Lt(debug_info)),
                "EQ" => tokens.push(Token::Equal(debug_info)),
                "PARENS" => tokens.push(Token::ParenS(debug_info)),
                "PS" => tokens.push(Token::Ps(debug_info)),
                "PE" => tokens.push(Token::Pe(debug_info)),
                "EVAL" => tokens.push(Token::Eval(debug_info)),
                "EVALMR" => tokens.push(Token::EvalMR(debug_info)),
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
