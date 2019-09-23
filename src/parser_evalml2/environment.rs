use super::lexer::{Token, Tokens};
use super::value::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    pub stack: Vec<(String, Value)>,
}
impl Environment {
    pub fn new(tokens: &mut Tokens) -> Environment {
        let mut stack: Vec<(String, Value)> = Vec::new();

        if let Some(Token::ENV) = tokens.peek() {
            return Environment { stack };
        }

        let var: String = match tokens.peek().expect("") {
            Token::Var(_) => {
                let var = tokens.consume_var();
                tokens.pop(); // consume =
                var
            }
            _ => panic!("unexpected"),
        };
        let val: Value = tokens.consume_val();
        stack.push((var, val));

        loop {
            match tokens.peek().expect("") {
                Token::COMMA => {
                    tokens.pop(); // consume ,
                    let var = tokens.consume_var();
                    tokens.pop(); // consume =
                    let val = tokens.consume_val();
                    stack.push((var, val));
                }
                Token::ENV => {
                    break;
                }
                _ => panic!("unexpected"),
            }
        }
        Environment { stack }
    }

    pub fn set_val(&mut self, identifier: String, value: Value) {
        self.stack.push((identifier, value));
    }

    pub fn pop_val(&mut self) {
        self.stack.pop();
    }

    pub fn _remove_val(&mut self, identifier: &String) {
        let idx = self.stack.len() - 1 - self.get_match_loc(identifier);
        self.stack.remove(idx);
    }

    pub fn get_match_loc(&self, identifier: &String) -> usize {
        for (loc, (var, _val)) in self.stack.iter().rev().enumerate() {
            if var == identifier {
                return loc;
            }
        }
        panic!(format!("variable \'{}\' is not declared", identifier));
    }

    pub fn get_val(&self, identifier: &String) -> Value {
        for (var, val) in self.stack.clone().into_iter().rev() {
            if &var == identifier {
                return val;
            }
        }
        panic!(format!("variable \'{}\' is not declared", identifier));
    }

    pub fn to_string(self) -> String {
        if self.stack.len() == 0 {
            String::from("|- ")
        } else {
            let mut s = "".to_string();
            for (i, (var, val)) in self.stack.iter().enumerate() {
                if i != 0 {
                    s += ", ";
                }
                s += &format!("{} = {}", var, val.to_string());
            }
            s + " |- "
        }
    }
}
