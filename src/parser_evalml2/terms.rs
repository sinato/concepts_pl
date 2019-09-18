use super::lexer::{Token, Tokens};

#[derive(Debug, Clone)]
pub enum Term {
    Val(i32),
    Var(String),
}
impl Term {
    pub fn new(tokens: &mut Tokens) -> Term {
        match tokens.peek().expect("a token") {
            Token::Int(_) => {
                let num: i32 = tokens.consume_num();
                Term::Val(num)
            }
            Token::Var(_) => {
                let var: String = tokens.consume_var();
                Term::Var(var)
            }
            _ => panic!("todo"),
        }
    }
}
