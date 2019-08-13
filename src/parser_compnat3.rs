use crate::lexer::Tokens;
use crate::util::*;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum RuleNode {
    LSucc(LSuccNode),
}
impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let n1 = tokens.consume_peano_num();
        tokens.pop(); // consume "is less than"
        let _n2 = tokens.consume_peano_num();
        RuleNode::LSucc(LSuccNode { n: n1 })
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::LSucc(node) => node.show(w, depth, with_newline),
        }
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
