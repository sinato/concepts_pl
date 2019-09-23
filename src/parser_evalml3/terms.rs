use super::environment::Environment;
use super::expression::{Expression, LetExpression};
use super::lexer::{Token, Tokens};
use super::value::Value;

#[derive(Debug, Clone)]
pub enum Term {
    Val(i32),
    Var(String),
    If(IfTerm),
    Let(LetTerm),
    Fun(FunTerm),
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
            Token::IF => {
                tokens.pop(); // consume if
                let condition_expression = Expression::new(tokens);
                tokens.pop(); // consume then
                let then_expression = Expression::new(tokens);
                tokens.pop(); // consume else
                let else_expression = Expression::new(tokens);
                Term::If(IfTerm {
                    condition_expression,
                    then_expression,
                    else_expression,
                })
            }
            Token::LET => {
                tokens.pop(); // consume let
                let let_expression = LetExpression::new(tokens);
                tokens.pop(); // consume in
                let in_expression = Expression::new(tokens);
                Term::Let(LetTerm {
                    let_expression,
                    in_expression,
                })
            }
            Token::FUN => {
                tokens.pop(); // consume fun
                let parameter: String = tokens.consume_var();
                tokens.pop(); // consume ->
                let function_body = Expression::new(tokens);
                Term::Fun(FunTerm {
                    parameter,
                    function_body,
                })
            }
            _ => panic!("todo"),
        }
    }

    pub fn get_val(self, environment: Environment) -> Value {
        match self {
            Term::Val(num) => Value::Num(num),
            Term::Var(identifier) => environment.get_val(&identifier),
            Term::If(_if_term) => panic!("todo"),
            Term::Let(let_term) => let_term.get_val(environment),
            Term::Fun(fun_term) => fun_term.get_val(environment),
        }
    }

    pub fn get_identifier(self) -> String {
        match self {
            Term::Var(identifier) => identifier,
            _ => panic!("unexpected"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfTerm {
    pub condition_expression: Expression,
    pub then_expression: Expression,
    pub else_expression: Expression,
}

#[derive(Debug, Clone)]
pub struct LetTerm {
    pub let_expression: LetExpression,
    pub in_expression: Expression,
}
impl LetTerm {
    pub fn get_val(self, environment: Environment) -> Value {
        let mut new_env = environment.clone();
        let new_val = self.let_expression.expression.get_val(environment);
        new_env.set_val(self.let_expression.identifier, new_val);
        self.in_expression.get_val(new_env)
    }
}

#[derive(Debug, Clone)]
pub struct FunTerm {
    pub parameter: String,
    pub function_body: Expression,
}
impl FunTerm {
    pub fn get_val(self, environment: Environment) -> Value {
        Value::Fun(self, environment)
    }
    pub fn to_string(&self, environment: &Environment) -> String {
        format!(
            "fun {} -> {}",
            self.parameter,
            self.function_body.to_string(environment)
        )
    }
}
