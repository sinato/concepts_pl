use super::lexer::{Token, Tokens};
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum RuleNode {
    EInt(EIntNode),
    EPlus(EPlusNode),
    BPlus(BPlusNode),
}
impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let i1_token = tokens.pop().expect("");
        let i1: i32 = match i1_token {
            Token::Int(val) => val.parse().expect(""),
            _ => panic!(""),
        };
        let e1 = Expression::Int(i1);

        tokens.pop(); // consume +

        let i2_token = tokens.pop().expect(""); // consume 5
        let i2: i32 = match i2_token {
            Token::Int(val) => val.parse().expect(""),
            _ => panic!(""),
        };
        let e2 = Expression::Int(i2);

        tokens.pop(); // consume evalto
        tokens.pop(); // consume 8
        RuleNode::EPlus(EPlusNode { e1, e2 })
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::EInt(node) => node.show(w, depth, with_newline),
            RuleNode::EPlus(node) => node.show(w, depth, with_newline),
            RuleNode::BPlus(node) => node.show(w, depth, with_newline),
        }
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Int(i32),
    Bin(String, Box<Expression>, Box<Expression>),
}
impl Expression {
    fn get_val(&self) -> i32 {
        match self {
            Expression::Int(val) => *val,
            Expression::Bin(operator, box_ex1, box_ex2) => {
                let val1 = box_ex1.get_val();
                let val2 = box_ex2.get_val();
                match operator.as_ref() {
                    "+" => val1 + val2,
                    _ => panic!(""),
                }
            }
        }
    }
    fn get_rule(self) -> RuleNode {
        match self {
            Expression::Int(i) => RuleNode::EInt(EIntNode { i }),
            Expression::Bin(operator, box_ex1, box_ex2) => match operator.as_ref() {
                "+" => RuleNode::EPlus(EPlusNode {
                    e1: *box_ex1,
                    e2: *box_ex2,
                }),
                _ => panic!(""),
            },
        }
    }
    fn to_string(self) -> String {
        match self {
            Expression::Int(val) => val.to_string(),
            Expression::Bin(operator, box_ex1, box_ex2) => {
                box_ex1.to_string() + " " + &operator + " " + &box_ex2.to_string()
            }
        }
    }
}

#[derive(Debug, Clone)]
struct EIntNode {
    i: i32,
}
impl EIntNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let nl = if with_newline { "\n" } else { "" };
        write!(
            w,
            "{}{} evalto {} by E-Int {{}}{}",
            get_depth_space(depth),
            self.i,
            self.i,
            nl
        )
    }
}

#[derive(Debug, Clone)]
struct EPlusNode {
    e1: Expression,
    e2: Expression,
}
impl EPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val();
        let i2 = self.e2.get_val();
        let _ = write!(
            w,
            "{}{} + {} evalto {} by E-Plus {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            i1 + i2
        );
        let premise1 = self.e1.get_rule();
        let premise2 = self.e2.get_rule();
        let _ = premise1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = premise2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");

        let premise = BPlusNode { i1, i2 };
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
struct BPlusNode {
    i1: i32,
    i2: i32,
}
impl BPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        write!(
            w,
            "{}{} plus {} is {} by B-Plus {{}}{}",
            get_depth_space(depth),
            self.i1,
            self.i2,
            self.i1 + self.i2,
            if with_newline { "\n" } else { "" }
        )
    }
}

pub fn get_depth_space(depth: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..depth {
        s += " ";
    }
    s
}
