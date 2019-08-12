use crate::lexer::{Token, Tokens};
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
        let nat_num: usize = if let Some(Token::Zero(_)) = tokens.pop() {
            0
        } else {
            panic!("")
        }; // consume num
        tokens.pop(); // consume is
        tokens.pop(); // consume Z
        PZeroNode { nat_num }
    }

    fn show<W: Write>(self, w: &mut W) -> io::Result<()> {
        write!(w, "Z plus {} is Z by P-Zero {{}}", "Z")
    }
}
