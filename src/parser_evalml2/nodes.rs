use super::environment::Environment;
use super::expression::Expression;
use super::nodes::enodes::EVarNode;
use std::io::{self, Write};

pub mod enodes;

#[derive(Debug, Clone)]
pub enum RuleNode {
    EVar(EVarNode),
}

impl RuleNode {
    pub fn new(environment: Environment, expression: Expression) -> RuleNode {
        let len: usize = expression.terms.len();
        if len == 1 {
            RuleNode::EVar(EVarNode::new(environment, expression))
        } else {
            panic!("todo")
        }
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::EVar(node) => node.show(w, depth, with_newline),
        }
    }
}

pub fn get_depth_space(depth: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..depth {
        s += " ";
    }
    s
}
