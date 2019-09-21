use super::lexer::{Token, Tokens};
use super::value::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    pub x: Option<Value>,
    pub y: Option<Value>,
}
impl Environment {
    pub fn new(tokens: &mut Tokens) -> Environment {
        let mut environment = Environment { x: None, y: None };
        match tokens.peek() {
            Some(token) => match token {
                Token::XEQ => {
                    tokens.pop(); // consume x =
                    let val = tokens.consume_val();
                    environment.x = Some(val);
                }
                _ => panic!("unexpected"),
            },
            None => panic!("unexpected"),
        }
        tokens.pop(); // consume ,
        match tokens.peek() {
            Some(token) => match token {
                Token::YEQ => {
                    tokens.pop(); // consume y =
                    let val = tokens.consume_val();
                    environment.y = Some(val);
                }
                _ => panic!("unexpected"),
            },
            None => panic!("unexpected"),
        }
        tokens.pop(); // consume |-
        environment
    }

    pub fn get_num(&self) -> usize {
        let mut num = 0;
        if let Some(_) = self.x {
            num += 1;
        }
        if let Some(_) = self.y {
            num += 1;
        }
        num
    }

    pub fn get_val(self, identifier: String) -> Value {
        match identifier.as_ref() {
            "x" => self.x.expect("expects Some(x)"),
            "y" => self.y.expect("expects Some(y)"),
            _ => panic!("unexpected identifier"),
        }
    }

    pub fn to_string(self) -> String {
        if self.get_num() == 0 {
            String::from("|- ")
        } else {
            let mut s = "".to_string();
            s = match self.x {
                Some(val) => s + &format!("x = {}", val.to_string()),
                None => s,
            };
            s = match self.y {
                Some(val) => s + &format!(", y = {}", val.to_string()),
                None => s,
            };
            s + " |- "
        }
    }
}
