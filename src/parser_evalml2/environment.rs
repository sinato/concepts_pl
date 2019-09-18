use super::lexer::{Token, Tokens};

#[derive(Debug, Clone)]
pub struct Environment {
    pub x: Option<i32>,
    pub y: Option<i32>,
}
impl Environment {
    pub fn new(tokens: &mut Tokens) -> Environment {
        let mut environment = Environment { x: None, y: None };

        match tokens.peek() {
            Some(token) => match token {
                Token::XEQ => {
                    tokens.pop(); // consume x =
                    let num: i32 = tokens.consume_num();
                    environment.x = Some(num);
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
                    let num: i32 = tokens.consume_num();
                    environment.y = Some(num);
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

    pub fn get_val(&self, identifier: String) -> i32 {
        match identifier.as_ref() {
            "x" => self.x.expect("expects Some(x)"),
            "y" => self.y.expect("expects Some(y)"),
            _ => panic!("unexpected identifier"),
        }
    }

    pub fn to_string(&self) -> String {
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
