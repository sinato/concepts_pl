use super::environment::Environment;
use super::lexer::{Token, Tokens};
use super::terms::Term;
use super::value::Value;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LetExpression {
    pub identifier: String,
    pub expression: Expression,
}
impl LetExpression {
    pub fn new(tokens: &mut Tokens) -> LetExpression {
        match tokens.peek().expect("") {
            Token::XEQ => {
                tokens.pop(); // consume x =
                let expression = Expression::new(tokens);
                LetExpression {
                    identifier: String::from("x"),
                    expression,
                }
            }
            Token::YEQ => {
                tokens.pop(); // consume y =
                let expression = Expression::new(tokens);
                LetExpression {
                    identifier: String::from("y"),
                    expression,
                }
            }
            _ => panic!("todo"),
        }
    }
}
impl LetExpression {
    pub fn to_string(self) -> String {
        let mut s = "".to_string();
        s += &self.identifier;
        s += " = ";
        s += &self.expression.to_string();
        s
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub terms: Vec<(String, Term)>,
}
impl Expression {
    pub fn new(tokens: &mut Tokens) -> Expression {
        let mut terms: Vec<(String, Term)> = Vec::new();

        // consume first token
        let term = Term::new(tokens);
        terms.push((String::from(""), term));

        // consume remaining token
        loop {
            match tokens.peek() {
                Some(token) => match token {
                    Token::Op(operator) => {
                        tokens.pop(); // consume operator
                        let term = Term::new(tokens);
                        terms.push((operator, term));
                    }
                    _ => break,
                },
                None => panic!("unexpected"),
            }
        }
        Expression { terms }
    }

    pub fn get_first_term(&mut self) -> Term {
        let (_, term) = self.terms.pop().expect("");
        term
    }

    pub fn get_val(mut self, environment: Environment) -> Value {
        if self.terms.len() == 1 {
            let (_, term) = self.terms.pop().expect("");
            term.get_val(environment)
        } else {
            let (split_position, operator) = self.get_split_position();
            let (former, latter) = self.get_splitted_expression(split_position);
            let former_val = former.get_val(environment.clone());
            let latter_val = latter.get_val(environment);
            match operator.as_ref() {
                "+" => former_val + latter_val,
                "*" => former_val * latter_val,
                _ => panic!(""),
            }
        }
    }

    pub fn to_string(self) -> String {
        let mut s = "".to_string();
        let terms = self.terms.into_iter();
        for t in terms {
            let (operator, term) = t;
            match term {
                Term::Val(num) => {
                    s = add_op(operator, s);
                    s += &num.to_string()
                }
                Term::Var(identifier) => {
                    s = add_op(operator, s);
                    s += &identifier
                }
                Term::If(if_term) => {
                    s = add_op(operator, s);
                    s += &format!(
                        "if {} then {} else {}",
                        if_term.condition_expression.to_string(),
                        if_term.then_expression.to_string(),
                        if_term.else_expression.to_string()
                    )
                }
                Term::Let(let_term) => {
                    s = add_op(operator, s);
                    s += &format!(
                        "let {} in {}",
                        let_term.let_expression.to_string(),
                        let_term.in_expression.to_string()
                    )
                }
            }
        }
        s
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
        for (i, (operator, term)) in terms.into_iter().enumerate() {
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

    pub fn get_splitted_expression(&self, split_position: usize) -> (Expression, Expression) {
        let mut former: Vec<(String, Term)> = Vec::new();
        let mut latter: Vec<(String, Term)> = Vec::new();

        let terms = self.terms.clone();
        for (i, term) in terms.into_iter().enumerate() {
            if i < split_position {
                former.push(term);
            } else {
                latter.push(term);
            }
        }
        let former = Expression { terms: former };
        let mut latter = Expression { terms: latter };
        latter.rm_first_operator();
        (former, latter)
    }

    fn rm_first_operator(&mut self) {
        let mut new_terms: Vec<(String, Term)> = Vec::new();
        let terms = self.terms.clone().into_iter();
        for (i, (operator, term)) in terms.enumerate() {
            if i == 0 {
                new_terms.push((String::from(""), term))
            } else {
                new_terms.push((operator, term));
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
