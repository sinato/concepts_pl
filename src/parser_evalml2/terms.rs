use super::lexer::{Token, Tokens};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Term {
    Single(String, i32),
    SingleB(String, String),
    Val(String, String),
    Paren(String, Terms),
    If(String, IfTerms),
    Let(String, LetTerms),
}

impl Term {
    fn new(tokens: &mut Tokens, operator: String) -> Term {
        match tokens.peek().expect("") {
            Token::PS => {
                tokens.pop(); // consume (
                let terms = consume_paren_terms(tokens);
                Term::Paren(operator, terms)
            }
            Token::Int(_) => {
                let num = tokens.consume_num();
                Term::Single(operator, num)
            }
            Token::Val(_) => {
                let val = tokens.consume_val();
                Term::Val(operator, val)
            }
            Token::Bool(_) => {
                let b = tokens.consume_bool();
                Term::SingleB(operator, b)
            }
            Token::IF => {
                let if_terms = IfTerms::new(tokens);
                Term::If(operator, if_terms)
            }
            Token::LET => {
                let let_terms = LetTerms::new(tokens);
                Term::Let(operator, let_terms)
            }
            _ => panic!("unexpect"),
        }
    }
    fn get_operator(&self) -> String {
        match self {
            Term::Single(op, _) => op.to_string(),
            Term::SingleB(op, _) => op.to_string(),
            Term::Val(op, _) => op.to_string(),
            Term::Paren(op, _) => op.to_string(),
            Term::If(op, _) => op.to_string(),
            Term::Let(op, _) => op.to_string(),
        }
    }
}

fn consume_paren_terms(tokens: &mut Tokens) -> Terms {
    let terms: Vec<Term> = Vec::new();
    let mut terms = Terms { terms };
    terms.push(Term::new(tokens, "".to_string()));
    loop {
        match tokens.peek() {
            Some(token) => match token {
                Token::Op(_) => {
                    let op = tokens.consume_op();
                    terms.push(Term::new(tokens, op));
                }
                Token::PE => {
                    tokens.pop(); // consume )
                    break;
                }
                _ => panic!(format!("unexpected token: {:?}", token)),
            },
            None => panic!("expect at least one eval token"),
        }
    }
    terms
}

fn consume_expression_terms(tokens: &mut Tokens) -> Terms {
    let terms: Vec<Term> = Vec::new();
    let mut terms = Terms { terms };
    terms.push(Term::new(tokens, "".to_string()));
    loop {
        match tokens.peek() {
            Some(token) => match token {
                Token::Op(_) => {
                    let op = tokens.consume_op();
                    terms.push(Term::new(tokens, op));
                }
                Token::IF
                | Token::ELSE
                | Token::THEN
                | Token::LET
                | Token::IN
                | Token::Eval(_)
                | Token::PE => break,
                _ => panic!(format!("unexpected token: {:?}", tokens)),
            },
            None => break,
        }
    }
    terms
}

#[derive(Debug, Clone)]
pub struct IfTerms {
    pub condition_terms: Terms,
    pub then_terms: Terms,
    pub else_terms: Terms,
}
impl IfTerms {
    pub fn new(tokens: &mut Tokens) -> IfTerms {
        tokens.pop(); // consume if
        let condition_terms = consume_expression_terms(tokens);
        tokens.pop(); // consume then
        let then_terms = consume_expression_terms(tokens);
        tokens.pop(); // consume else
        let else_terms = consume_expression_terms(tokens);
        IfTerms {
            condition_terms,
            then_terms,
            else_terms,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LetTerms {
    pub val: String,
    pub equal_terms: Terms,
    pub in_terms: Terms,
}
impl LetTerms {
    pub fn new(tokens: &mut Tokens) -> LetTerms {
        tokens.pop(); // consume let
        let val = match tokens.pop() {
            Some(token) => match token {
                Token::XEQ => "x".to_string(),
                Token::YEQ => "y".to_string(),
                _ => panic!("expects x or y"),
            },
            None => panic!("ecpects x or y"),
        };
        let equal_terms = consume_expression_terms(tokens);
        tokens.pop(); // consume in
        let in_terms = consume_expression_terms(tokens);
        LetTerms {
            val,
            equal_terms,
            in_terms,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Terms {
    terms: Vec<Term>,
}
impl Terms {
    pub fn new(tokens: &mut Tokens) -> Terms {
        let terms: Vec<Term> = Vec::new();
        let mut terms = Terms { terms };
        terms.push(Term::new(tokens, "".to_string()));
        loop {
            match tokens.peek() {
                Some(token) => match token {
                    Token::Op(_) => {
                        let op = tokens.consume_op();
                        terms.push(Term::new(tokens, op));
                    }
                    Token::Eval(_) => break,
                    _ => panic!(format!("unexpected token: {:?}", tokens)),
                },
                None => break,
            }
        }
        terms
    }
    fn push(&mut self, term: Term) {
        self.terms.push(term)
    }
    pub fn pop(&mut self) -> Option<Term> {
        self.terms.pop()
    }
    pub fn len(&mut self) -> usize {
        self.terms.len()
    }
    pub fn get_split_position(&self) -> (usize, String) {
        let mut priorities: HashMap<String, usize> = HashMap::new();
        priorities.insert("".to_string(), 0);
        priorities.insert("*".to_string(), 10);
        priorities.insert("+".to_string(), 20);
        priorities.insert("-".to_string(), 20);
        priorities.insert("<".to_string(), 30);

        let mut split_position = 0;
        let mut priority: usize = 0;
        let mut ret_op: String = "".to_string();
        let terms = self.terms.clone();
        for (i, term) in terms.into_iter().enumerate() {
            let operator = term.get_operator();
            if priority
                <= *priorities
                    .get(&operator)
                    .expect(&format!("cannot get operator priority: {}", operator))
            {
                split_position = i;
                priority = *priorities
                    .get(&operator)
                    .expect(&format!("cannot get operator priority: {}", operator));
                ret_op = operator;
            }
        }
        (split_position, ret_op.to_string())
    }
    pub fn get_splitted_terms(&self, split_position: usize) -> (Terms, Terms) {
        let mut former: Vec<Term> = Vec::new();
        let mut latter: Vec<Term> = Vec::new();

        let terms = self.terms.clone();
        for (i, term) in terms.into_iter().enumerate() {
            if i < split_position {
                former.push(term);
            } else {
                latter.push(term);
            }
        }
        let former = Terms { terms: former };
        let mut latter = Terms { terms: latter };
        latter.rm_first_operator();
        (former, latter)
    }
    pub fn to_string(self) -> String {
        let mut s = "".to_string();
        let terms = self.terms.into_iter();
        for term in terms {
            match term {
                Term::Single(operator, num) => {
                    s = add_op(operator, s);
                    s += &num.to_string()
                }
                Term::SingleB(operator, val) => {
                    s = add_op(operator, s);
                    s += &val
                }
                Term::Val(operator, val) => {
                    s = add_op(operator, s);
                    s += &val.to_string()
                }
                Term::Paren(operator, terms) => {
                    s = add_op(operator, s);
                    s += "(";
                    s += &terms.to_string();
                    s += ")";
                }
                Term::If(operator, if_terms) => {
                    s = add_op(operator, s);
                    s += &format!(
                        "if {} then {} else {}",
                        if_terms.condition_terms.to_string(),
                        if_terms.then_terms.to_string(),
                        if_terms.else_terms.to_string()
                    )
                }
                Term::Let(operator, let_terms) => {
                    s = add_op(operator, s);
                    s += &format!(
                        "let {} = {} in {}",
                        let_terms.val,
                        let_terms.equal_terms.to_string(),
                        let_terms.in_terms.to_string(),
                    )
                }
            }
        }
        s
    }
    fn rm_first_operator(&mut self) {
        let mut new_terms: Vec<Term> = Vec::new();
        let terms = self.terms.clone().into_iter();
        for (i, term) in terms.enumerate() {
            if i == 0 {
                match term {
                    Term::Single(_, num) => new_terms.push(Term::Single(String::from(""), num)),
                    Term::SingleB(_, val) => new_terms.push(Term::SingleB(String::from(""), val)),
                    Term::Paren(_, v) => new_terms.push(Term::Paren(String::from(""), v)),
                    Term::If(_, v) => new_terms.push(Term::If(String::from(""), v)),
                    _ => panic!("todo"),
                }
            } else {
                new_terms.push(term);
            }
        }
        self.terms = new_terms;
    }
}

fn add_op(operator: String, s: String) -> String {
    match operator.as_ref() {
        "" => s + "",
        "+" => s + " + ",
        "-" => s + " - ",
        "*" => s + " * ",
        "<" => s + " < ",
        _ => panic!("TODO"),
    }
}
