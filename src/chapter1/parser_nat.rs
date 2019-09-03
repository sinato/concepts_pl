use super::lexer::Tokens;
use super::util::*;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum RuleNode {
    PZero(PZeroNode),
    PSucc(PSuccNode),
    TZero(TZeroNode),
    TSucc(TSuccNode),
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
            "times" => get_rule_times(n1, n2, n3),
            _ => panic!("unexpected operator"),
        }
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::PZero(node) => node.show(w, depth, with_newline),
            RuleNode::PSucc(node) => node.show(w, depth, with_newline),
            RuleNode::TZero(node) => node.show(w, depth, with_newline),
            RuleNode::TSucc(node) => node.show(w, depth, with_newline),
        }
    }
}

fn get_rule_times(n1: usize, n2: usize, n3: usize) -> RuleNode {
    if n1 == 0 {
        RuleNode::TZero(TZeroNode { nat_num: n2 })
    } else {
        let premise1 = Box::new(get_rule_times(n1 - 1, n2, n3 - n2));
        let premise2 = Box::new(get_rule_plus(n2, n3 - n2, n3));
        RuleNode::TSucc(TSuccNode {
            n1,
            n2,
            n3,
            premise1,
            premise2,
        })
    }
}

fn get_rule_plus(n1: usize, n2: usize, n3: usize) -> RuleNode {
    if n1 == 0 {
        RuleNode::PZero(PZeroNode { nat_num: n2 })
    } else {
        let premise = get_rule_plus(n1 - 1, n2, n3 - 1);
        RuleNode::PSucc(PSuccNode {
            n1,
            n2,
            n3,
            premise: Box::new(premise),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PZeroNode {
    nat_num: usize,
}
impl PZeroNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n = get_peano_num(self.nat_num);
        let _ = write!(w, "{}", get_depth_space(depth));
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "Z plus {} is {} by P-Zero {{}}{}", n, n, nl)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PSuccNode {
    n1: usize,
    n2: usize,
    n3: usize,
    premise: Box<RuleNode>,
}
impl PSuccNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n1 = get_peano_num(self.n1);
        let n2 = get_peano_num(self.n2);
        let n3 = get_peano_num(self.n3);
        let _ = write!(
            w,
            "{}{} plus {} is {} by P-Succ {{\n",
            get_depth_space(depth),
            n1,
            n2,
            n3
        );
        let _ = self.premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TZeroNode {
    nat_num: usize,
}
impl TZeroNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n = get_peano_num(self.nat_num);
        let _ = write!(w, "{}", get_depth_space(depth));
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "Z times {} is Z by T-Zero {{}}{}", n, nl)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TSuccNode {
    n1: usize,
    n2: usize,
    n3: usize,
    premise1: Box<RuleNode>,
    premise2: Box<RuleNode>,
}
impl TSuccNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n1 = get_peano_num(self.n1);
        let n2 = get_peano_num(self.n2);
        let n3 = get_peano_num(self.n3);
        let _ = write!(
            w,
            "{}{} times {} is {} by T-Succ {{\n",
            get_depth_space(depth),
            n1,
            n2,
            n3
        );
        let _ = self.premise1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = self.premise2.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}
