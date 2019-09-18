use super::lexer::{Token, Tokens};
use super::terms::Term;

#[derive(Debug, Clone)]
pub struct Expression {
    pub terms: Vec<(String, Term)>,
}
impl Expression {
    pub fn new(tokens: &mut Tokens) -> Expression {
        let term = Term::new(tokens);
        let mut terms: Vec<(String, Term)> = Vec::new();
        terms.push((String::from(""), term));
        Expression { terms }
    }
}
