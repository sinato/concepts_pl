use crate::lexer::Tokens;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum RuleNode {
    PZero(PZeroNode),
}
impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let rule = PZeroNode::new(tokens);
        RuleNode::PZero(rule)
    }

    pub fn show<W: Write>(self, w: &mut W) -> io::Result<()> {
        match self {
            RuleNode::PZero(node) => node.show(w),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PZeroNode {
    nat_num: usize,
}
impl PZeroNode {
    fn new(tokens: &mut Tokens) -> PZeroNode {
        tokens.pop(); // consume Z
        tokens.pop(); // consume plus
        let nat_num: usize = tokens.consume_peano_num(); // consume num
        tokens.pop(); // consume is
        tokens.pop(); // consume Z
        PZeroNode { nat_num }
    }

    fn show<W: Write>(self, w: &mut W) -> io::Result<()> {
        let n = get_peano_num(self.nat_num);
        write!(w, "Z plus {} is {} by P-Zero {{}}", n, n)
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
