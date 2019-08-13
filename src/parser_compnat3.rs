use crate::lexer::Tokens;
use crate::util::*;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum RuleNode {
    LSucc(LSuccNode),
    LSuccR(LSuccRNode),
}
impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let n1 = tokens.consume_peano_num();
        tokens.pop(); // consume "is less than"
        let n2 = tokens.consume_peano_num();
        get_rule_lt(n1, n2)
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::LSucc(node) => node.show(w, depth, with_newline),
            RuleNode::LSuccR(node) => node.show(w, depth, with_newline),
        }
    }
}

fn get_rule_lt(n1: usize, n2: usize) -> RuleNode {
    if n1 + 1 == n2 {
        RuleNode::LSucc(LSuccNode { n: n1 })
    } else if n1 + 1 < n2 {
        let premise = Box::new(get_rule_lt(n1, n2 - 1));
        RuleNode::LSuccR(LSuccRNode { n1, n2, premise })
    } else {
        panic!("unexpected number")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LSuccNode {
    n: usize,
}
impl LSuccNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n1 = get_peano_num(self.n);
        let n2 = get_peano_num(self.n + 1);
        let nl = if with_newline { "\n" } else { "" };
        write!(
            w,
            "{}{} is less than {} by L-Succ {{}}{}",
            get_depth_space(depth),
            n1,
            n2,
            nl
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LSuccRNode {
    n1: usize,
    n2: usize,
    premise: Box<RuleNode>,
}
impl LSuccRNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n1 = get_peano_num(self.n1);
        let n2 = get_peano_num(self.n2);
        let _ = write!(
            w,
            "{}{} is less than {} by L-SuccR {{\n",
            get_depth_space(depth),
            n1,
            n2,
        );
        let _ = self.premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}
