use crate::lexer::Tokens;
use crate::util::{get_depth_space, get_peano_num};
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum RuleNode {
    EConst(EConstNode),
    EPlus(EPlusNode),
    PZero(PZeroNode),
    PSucc(PSuccNode),
}

impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let n1 = tokens.consume_peano_num();
        tokens.pop(); // consume +
        let n2 = tokens.consume_peano_num();
        tokens.pop(); // consume evalto
        let n3 = tokens.consume_peano_num();
        get_rule_eval(n1, n2, n3)
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::EConst(node) => node.show(w, depth, with_newline),
            RuleNode::EPlus(node) => node.show(w, depth, with_newline),
            RuleNode::PZero(node) => node.show(w, depth, with_newline),
            RuleNode::PSucc(node) => node.show(w, depth, with_newline),
        }
    }
}

fn get_rule_eval(n1: usize, n2: usize, n3: usize) -> RuleNode {
    let premise_term1 = Box::new(RuleNode::EConst(EConstNode { n: n1 }));
    let premise_term2 = Box::new(RuleNode::EConst(EConstNode { n: n2 }));
    let premise = Box::new(get_rule_plus(n1, n2, n3));
    RuleNode::EPlus(EPlusNode {
        n1,
        n2,
        premise_term1,
        premise_term2,
        premise,
    })
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
pub struct EConstNode {
    n: usize,
}
impl EConstNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n = get_peano_num(self.n);
        let nl = if with_newline { "\n" } else { "" };
        write!(
            w,
            "{}{} evalto {} by E-Const {{}}{}",
            get_depth_space(depth),
            n,
            n,
            nl,
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EPlusNode {
    n1: usize,
    n2: usize,
    premise_term1: Box<RuleNode>,
    premise_term2: Box<RuleNode>,
    premise: Box<RuleNode>,
}
impl EPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n1 = get_peano_num(self.n1);
        let n2 = get_peano_num(self.n2);
        let n3 = get_peano_num(self.n1 + self.n2);
        let _ = write!(
            w,
            "{}{} + {} evalto {} by E-Plus {{\n",
            get_depth_space(depth),
            n1,
            n2,
            n3,
        );
        let _ = self.premise_term1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = self.premise_term2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = self.premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
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
