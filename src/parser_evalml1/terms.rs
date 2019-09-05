use super::lexer::{Token, Tokens};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Term {
    Leaf(String, i32),
    Node(String, Terms),
}
impl Term {
    fn new(tokens: &mut Tokens, operator: String) -> Term {
        match tokens.peek().expect("") {
            Token::PS => {
                tokens.pop(); // consume (
                let terms = Terms::new(tokens);
                Term::Node(operator, terms)
            }
            Token::Int(_) => {
                let num = tokens.consume_num();
                Term::Leaf(operator, num)
            }
            _ => panic!("unexpect"),
        }
    }
    fn get_operator(&self) -> String {
        match self {
            Term::Leaf(op, _) => op.to_string(),
            Term::Node(op, _) => op.to_string(),
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
                    Token::Int(_) => panic!("unexpected num token"),
                    Token::Op(_) => {
                        let op = tokens.consume_op();
                        terms.push(Term::new(tokens, op));
                    }
                    Token::Eval(_) => {
                        tokens.pop(); // consume evalto
                        tokens.pop(); // consume expression result
                        break;
                    }
                    Token::PS => panic!("unexpect ( token"),
                    Token::PE => {
                        tokens.pop(); // consume )
                        break;
                    }
                },
                None => panic!("expect at least one eval token"),
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

        let mut split_position = 0;
        let mut priority: usize = 0;
        let mut ret_op: String = "".to_string();
        let terms = self.terms.clone();
        for (i, term) in terms.into_iter().enumerate() {
            let operator = term.get_operator();
            if priority
                <= *priorities
                    .get(&operator)
                    .expect("cannot get operator priority")
            {
                split_position = i;
                priority = *priorities
                    .get(&operator)
                    .expect("cannot get operator priority");
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
                Term::Leaf(operator, num) => {
                    let snum = &num.to_string();
                    match operator.as_ref() {
                        "" => s += snum,
                        "+" => s = s + " + " + snum,
                        "-" => s = s + " - " + snum,
                        "*" => s = s + " * " + snum,
                        _ => panic!("TODO"),
                    }
                }
                Term::Node(operator, terms) => {
                    match operator.as_ref() {
                        "" => (),
                        "+" => s = s + " + ",
                        "-" => s = s + " - ",
                        "*" => s = s + " * ",
                        _ => panic!("TODO"),
                    }
                    s += "(";
                    s += &terms.to_string();
                    s += ")";
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
                    Term::Leaf(_, num) => new_terms.push(Term::Leaf(String::from(""), num)),
                    Term::Node(_, v) => new_terms.push(Term::Node(String::from(""), v)),
                }
            } else {
                new_terms.push(term);
            }
        }
        self.terms = new_terms;
    }
}
