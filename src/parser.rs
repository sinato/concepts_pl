use crate::lexer::Tokens;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum RuleNode {
    PZero(PZeroNode),
    PSucc(PSuccNode),
    TZero(TZeroNode),
}
impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let n1 = tokens.consume_peano_num();
        let op = tokens.consume_operator();
        let n2 = tokens.consume_peano_num();
        tokens.pop(); // consume is
        let n3 = tokens.consume_peano_num();
        match op.as_ref() {
            "plus" => get_rule_plus(n1, n2, n3),
            "times" => get_rule_times(n2),
            _ => panic!("unexpected operator"),
        }
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize) -> io::Result<()> {
        match self {
            RuleNode::PZero(node) => node.show(w, depth),
            RuleNode::PSucc(node) => node.show(w, depth),
            RuleNode::TZero(node) => node.show(w, depth),
        }
    }
}
fn get_rule_times(n2: usize) -> RuleNode {
    RuleNode::TZero(TZeroNode { nat_num: n2 })
}

fn get_rule_plus(n1: usize, n2: usize, n3: usize) -> RuleNode {
    if n1 == 0 {
        RuleNode::PZero(PZeroNode { nat_num: n2 })
    } else {
        let next_premise = get_rule_plus(n1 - 1, n2, n3 - 1);
        RuleNode::PSucc(PSuccNode {
            nat_n1: n1,
            nat_n2: n2,
            nat_n3: n3,
            premise: Box::new(next_premise),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PZeroNode {
    nat_num: usize,
}
impl PZeroNode {
    fn show<W: Write>(self, w: &mut W, depth: usize) -> io::Result<()> {
        let n = get_peano_num(self.nat_num);
        let _ = write!(w, "{}", get_depth_space(depth));
        write!(w, "Z plus {} is {} by P-Zero {{}}\n", n, n)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PSuccNode {
    nat_n1: usize,
    nat_n2: usize,
    nat_n3: usize,
    premise: Box<RuleNode>,
}
impl PSuccNode {
    fn show<W: Write>(self, w: &mut W, depth: usize) -> io::Result<()> {
        let n1 = get_peano_num(self.nat_n1);
        let n2 = get_peano_num(self.nat_n2);
        let n3 = get_peano_num(self.nat_n3);
        let _ = write!(
            w,
            "{}{} plus {} is {} by P-Succ {{\n",
            get_depth_space(depth),
            n1,
            n2,
            n3
        );
        let _ = self.premise.show(w, depth + 4);
        write!(w, "{}}}\n", get_depth_space(depth))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TZeroNode {
    nat_num: usize,
}
impl TZeroNode {
    fn show<W: Write>(self, w: &mut W, depth: usize) -> io::Result<()> {
        let n = get_peano_num(self.nat_num);
        let _ = write!(w, "{}", get_depth_space(depth));
        write!(w, "Z times {} is Z by T-Zero {{}}\n", n)
    }
}

fn get_peano_num(nat_num: usize) -> String {
    let mut s = "".to_string();

    for _ in 0..nat_num {
        s += "S(";
    }
    s += "Z";
    for _ in 0..nat_num {
        s += ")";
    }
    s
}

fn get_depth_space(depth: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..depth {
        s += " ";
    }
    s
}
