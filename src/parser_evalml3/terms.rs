use super::environment::Environment;
use super::expression::{Expression, LetExpression};
use super::lexer::{Token, Tokens};
use super::value::Value;

#[derive(Debug, Clone)]
pub enum Term {
    Val(i32),
    Var(String),
    Paren(Expression),
    If(IfTerm),
    Let(LetTerm),
    Fun(FunTerm),
    App(AppTerm),
}
impl Term {
    pub fn new(tokens: &mut Tokens) -> Term {
        match tokens.peek().expect("a token") {
            Token::Int(_) => {
                let num: i32 = tokens.consume_num();
                Term::Val(num)
            }
            Token::Var(variable) => {
                // assumes that function name is not (x or y)
                match variable.as_ref() {
                    "x" | "y" => {
                        // not function variable
                        let var: String = tokens.consume_var();
                        Term::Var(var)
                    }
                    _ => {
                        // function variable
                        let function = Box::new(Term::Var(tokens.consume_var()));

                        println!("function argument: {:?}", tokens);

                        let term = Term::new(tokens);
                        let terms: Vec<(String, Term)> = vec![("".to_string(), term];
                        let argument = Expression { terms };

                        Term::App(AppTerm { function, argument })
                    }
                }
            }
            Token::PS => {
                tokens.pop(); // consume (
                let expression = Expression::new(tokens);
                tokens.pop(); // consume )
                Term::Paren(expression)
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
            Term::Paren(expression) => expression.get_val(environment),
            Term::If(_if_term) => panic!("todo"),
            Term::Let(let_term) => let_term.get_val(environment),
            Term::Fun(fun_term) => fun_term.get_val(environment),
            Term::App(app_term) => app_term.get_val(environment),
        }
    }
    pub fn get_identifier(self) -> String {
        match self {
            Term::Var(identifier) => identifier,
            _ => panic!("unexpected"),
        }
    }
    pub fn to_string(&self, environment: &Environment) -> String {
        match self {
            Term::Val(num) => num.to_string(),
            Term::Var(identifier) => identifier.clone(),
            Term::Paren(expression) => format!("({})", expression.to_string(environment)),
            Term::If(if_term) => if_term.to_string(environment),
            Term::Let(let_term) => let_term.to_string(environment),
            Term::Fun(fun_term) => fun_term.to_string(environment),
            Term::App(app_term) => app_term.to_string(environment),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfTerm {
    pub condition_expression: Expression,
    pub then_expression: Expression,
    pub else_expression: Expression,
}
impl IfTerm {
    pub fn to_string(&self, environment: &Environment) -> String {
        format!(
            "if {} then {} else {}",
            self.condition_expression.to_string(environment),
            self.then_expression.to_string(environment),
            self.else_expression.to_string(environment)
        )
    }
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
    pub fn to_string(&self, environment: &Environment) -> String {
        format!(
            "let {} in {}",
            self.let_expression.to_string(&environment),
            self.in_expression.to_string(&environment)
        )
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

#[derive(Debug, Clone)]
pub struct AppTerm {
    pub function: Box<Term>,
    pub argument: Expression,
}
impl AppTerm {
    pub fn get_val(self, environment: Environment) -> Value {
        let function_name = self.function.get_identifier();
        let value: Value = environment.get_val(&function_name);
        match value {
            Value::Fun(fun_term, mut clojure_env) => {
                let parameter: String = fun_term.parameter;
                clojure_env.set_val(parameter, self.argument.get_val(environment));
                let expression: Expression = fun_term.function_body;
                expression.get_val(clojure_env)
            }
            _ => panic!("unexpected"),
        }
    }
    pub fn get_fun_info(&self, environment: Environment) -> (FunTerm, Environment) {
        let function_name = self.function.clone().get_identifier();
        let value: Value = environment.get_val(&function_name);
        match value {
            Value::Fun(fun_term, env) => (fun_term, env),
            _ => panic!("unexpected"),
        }
    }
    pub fn to_string(&self, environment: &Environment) -> String {
        format!(
            "{} {}",
            self.function.clone().get_identifier(),
            self.argument.to_string(environment)
        )
    }
}
