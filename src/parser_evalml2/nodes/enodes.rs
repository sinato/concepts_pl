use super::super::environment::Environment;
use super::super::expression::Expression;
use super::super::nodes::{get_depth_space, RuleNode};

use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct EVarNode {
    environment: Environment,
    expression: Expression,
}
impl EVarNode {
    pub fn new(environment: Environment, expression: Expression) -> EVarNode {
        EVarNode {
            environment,
            expression,
        }
    }
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self.environment.get_num() {
            2 => {
                let _ = write!(
                    w,
                    "{}{}x evalto {} by E-Var2 {{\n",
                    get_depth_space(depth),
                    self.environment.to_string(),
                    self.environment.get_val(String::from("x")),
                );
                let mut new_env = self.environment.clone();
                new_env.y = None;
                let premise = RuleNode::new(new_env, self.expression);
                let _ = premise.show(w, depth + 2, true);
                let nl = if with_newline { "\n" } else { "" };
                write!(w, "{}}}{}", get_depth_space(depth), nl)
            }
            1 => {
                // TODO y pattern
                let nl = if with_newline { "\n" } else { "" };
                write!(
                    w,
                    "{}{}x evalto {} by E-Var1 {{}}{}",
                    get_depth_space(depth),
                    self.environment.to_string(),
                    self.environment.get_val(String::from("x")),
                    nl
                )
            }
            _ => panic!("unexpected"),
        }
    }
}
