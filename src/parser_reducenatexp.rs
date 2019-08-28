use crate::lexer::{Token, Tokens};
use crate::util::{get_depth_space, get_peano_num};

use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum RuleNode {
    MROne(MROneNode),
    RPlus(RPlusNode),
    PZero(PZeroNode),
    PSucc(PSuccNode),
}

impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let before_terms = Terms::new(tokens);
        let eval_op = match tokens.pop().expect("eval operator") {
            Token::EvalMR(_) => "-*->".to_string(),
            _ => panic!("TODO"),
        };
        let after_terms = Terms::new(tokens);

        match eval_op.as_ref() {
            "-*->" => RuleNode::MROne(MROneNode {
                before_terms,
                after_terms,
            }),
            _ => panic!("TODO"),
        }
    }
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::MROne(node) => node.show(w, depth, with_newline),
            RuleNode::RPlus(node) => node.show(w, depth, with_newline),
            RuleNode::PZero(node) => node.show(w, depth, with_newline),
            RuleNode::PSucc(node) => node.show(w, depth, with_newline),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MROneNode {
    before_terms: Terms,
    after_terms: Terms,
}
impl MROneNode {
    fn show<W: Write>(mut self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} -*-> {} by MR-One {{\n",
            get_depth_space(depth),
            self.before_terms.clone().to_string(),
            self.after_terms.to_string(),
        );

        let premise = get_rplus(&mut self.before_terms);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct RPlusNode {
    n1: usize,
    n2: usize,
}
impl RPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} + {} ---> {} by R-Plus {{\n",
            get_depth_space(depth),
            get_peano_num(self.n1),
            get_peano_num(self.n2),
            get_peano_num(self.n1 + self.n2),
        );

        let premise = get_rule_plus(self.n1, self.n2);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

fn get_rule_plus(n1: usize, n2: usize) -> RuleNode {
    if n1 == 0 {
        RuleNode::PZero(PZeroNode { n: n2 })
    } else {
        RuleNode::PSucc(PSuccNode { n1, n2 })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PZeroNode {
    n: usize,
}
impl PZeroNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n = get_peano_num(self.n);
        let _ = write!(w, "{}", get_depth_space(depth));
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "Z plus {} is {} by P-Zero {{}}{}", n, n, nl)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PSuccNode {
    n1: usize,
    n2: usize,
}
impl PSuccNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} plus {} is {} by P-Succ {{\n",
            get_depth_space(depth),
            get_peano_num(self.n1),
            get_peano_num(self.n2),
            get_peano_num(self.n1 + self.n2),
        );
        let premise = get_rule_plus(self.n1 - 1, self.n2);
        let _ = premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

fn get_rplus(terms: &mut Terms) -> RuleNode {
    let (_, n1) = terms.pop().expect("");
    let (_, n2) = terms.pop().expect("");
    RuleNode::RPlus(RPlusNode { n1, n2 })
}

#[derive(Debug, Clone)]
struct Terms {
    terms: Vec<(String, usize)>,
}
impl Terms {
    fn new(tokens: &mut Tokens) -> Terms {
        let mut terms: Vec<(String, usize)> = Vec::new();
        let num = tokens.consume_peano_num();
        terms.push(("".to_string(), num));
        loop {
            match tokens.peek() {
                Some(token) => match token {
                    Token::OpC(operator, _) => {
                        tokens.pop(); // consume operator
                        let num = tokens.consume_peano_num();
                        terms.push((operator, num));
                    }
                    Token::EvalMR(_) => break,
                    _ => panic!("unexpected token: {:?}"),
                },
                None => break,
            }
        }
        Terms { terms }
    }
    fn pop(&mut self) -> Option<(String, usize)> {
        self.terms.reverse();
        let term = self.terms.pop();
        self.terms.reverse();
        term
    }
    fn to_string(self) -> String {
        let mut s = "".to_string();
        let terms = self.terms.into_iter();
        for (operator, num) in terms {
            let snum = &get_peano_num(num);
            match operator.as_ref() {
                "" => s += snum,
                "+" => s = s + " + " + snum,
                "*" => s = s + " * " + snum,
                _ => panic!("TODO"),
            }
        }
        s
    }
}
