use super::environment::Environment;
use super::expression::Expression;
use super::nodes::enodes::{EBNode, EIfNode, ELetNode, EValNode, EVarNode, RuleWriter};
use super::terms::Term;
use std::io::{self, Write};

pub mod bnodes;
pub mod enodes;

#[derive(Debug, Clone)]
pub enum RuleNode {
    EVar(EVarNode),
    EVal(EValNode),
    EIf(EIfNode),
    ELet(ELetNode),
    EBNode(EBNode),
}

impl RuleNode {
    pub fn new(environment: Environment, mut expression: Expression) -> RuleNode {
        let len: usize = expression.terms.len();
        let original_expression = expression.clone();
        if len == 1 {
            let term = expression.get_first_term();
            match term {
                Term::If(if_node) => RuleNode::EIf(EIfNode {
                    environment,
                    expression: original_expression,
                    term: if_node,
                }),
                Term::Var(_) => RuleNode::EVar(EVarNode {
                    environment,
                    expression: original_expression,
                }),
                Term::Val(_) => RuleNode::EVal(EValNode {
                    environment,
                    expression: original_expression,
                }),
                Term::Let(let_node) => RuleNode::ELet(ELetNode {
                    environment,
                    expression: original_expression,
                    term: let_node,
                }),
            }
        } else {
            RuleNode::EBNode(EBNode {
                environment,
                expression: original_expression,
            })
        }
    }
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        match self {
            RuleNode::EVar(node) => node.show(writer),
            RuleNode::EVal(node) => node.show(writer),
            RuleNode::EIf(node) => node.show(writer),
            RuleNode::ELet(node) => node.show(writer),
            RuleNode::EBNode(node) => node.show(writer),
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
