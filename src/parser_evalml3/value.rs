use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    Num(i32),
    Bool(String),
}
impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Num(num) => num.to_string(),
            Value::Bool(b) => b.to_string(),
        }
    }
    pub fn get_num(self) -> i32 {
        match self {
            Value::Num(i) => i,
            _ => panic!("unexpcted"),
        }
    }
}
impl Add for Value {
    type Output = Self;

    fn add(self, other: Value) -> Value {
        match self {
            Value::Num(num1) => match other {
                Value::Num(num2) => Value::Num(num1 + num2),
                _ => panic!(""),
            },
            _ => panic!(""),
        }
    }
}
impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Value) -> Value {
        match self {
            Value::Num(num1) => match other {
                Value::Num(num2) => Value::Num(num1 - num2),
                _ => panic!(""),
            },
            _ => panic!(""),
        }
    }
}
impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Value) -> Value {
        match self {
            Value::Num(num1) => match other {
                Value::Num(num2) => Value::Num(num1 * num2),
                _ => panic!(""),
            },
            _ => panic!(""),
        }
    }
}
