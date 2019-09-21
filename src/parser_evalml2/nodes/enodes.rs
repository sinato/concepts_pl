use super::super::environment::Environment;
use super::super::expression::Expression;
use super::super::nodes::{get_depth_space, RuleNode};
use super::super::terms::{IfTerm, Term};
use super::super::value::Value;
use super::bnodes::BPlusNode;

use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct EVarNode {
    pub environment: Environment,
    pub expression: Expression,
}
impl EVarNode {
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let nl = if with_newline { "\n" } else { "" };
        match self.environment.get_num() {
            2 => {
                let mut terms = self.expression.terms.clone();
                let (_, term) = terms.pop().expect("");
                let identifier = term.get_identifier();
                match identifier.as_ref() {
                    "x" => {
                        let _ = write!(
                            w,
                            "{}{}x evalto {} by E-Var2 {{\n",
                            get_depth_space(depth),
                            self.environment.clone().to_string(),
                            self.environment
                                .clone()
                                .get_val(String::from("x"))
                                .to_string(),
                        );
                        let mut new_env = self.environment.clone();
                        new_env.y = None;
                        let premise = RuleNode::new(new_env, self.expression);
                        let _ = premise.show(w, depth + 2, true);
                        let nl = if with_newline { "\n" } else { "" };
                        write!(w, "{}}}{}", get_depth_space(depth), nl)
                    }
                    "y" => write!(
                        w,
                        "{}{}{} evalto {} by E-Var1 {{}}{}",
                        get_depth_space(depth),
                        self.environment.clone().to_string(),
                        identifier.clone(),
                        self.environment.clone().get_val(identifier).to_string(),
                        nl
                    ),
                    _ => panic!("unexpected"),
                }
            }
            1 => write!(
                w,
                "{}{}{} evalto {} by E-Var1 {{}}{}",
                get_depth_space(depth),
                self.environment.clone().to_string(),
                "x",
                self.environment
                    .clone()
                    .get_val(String::from("x"))
                    .to_string(),
                nl
            ),
            _ => panic!("unexpected"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EIfNode {
    pub environment: Environment,
    pub expression: Expression,
    pub term: IfTerm,
}
impl EIfNode {
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let condition_expression = self.term.condition_expression;
        let then_expression = self.term.then_expression;
        let else_expression = self.term.else_expression;

        let flag: String = match condition_expression
            .clone()
            .get_val(self.environment.clone())
        {
            Value::Num(_) => panic!("unexpected"),
            Value::Bool(b) => b,
        };

        if flag == String::from("true") {
            let _ = write!(
                w,
                "{}{}{} evalto {} by E-IfT {{\n",
                get_depth_space(depth),
                self.environment.clone().to_string(),
                self.expression.clone().to_string(),
                then_expression
                    .clone()
                    .get_val(self.environment.clone())
                    .to_string(),
            );
            let condition_premise = RuleNode::new(self.environment.clone(), condition_expression);
            let then_premise = RuleNode::new(self.environment, then_expression);

            let _ = condition_premise.show(w, depth + 2, false);
            let _ = write!(w, ";\n");
            let _ = then_premise.show(w, depth + 2, true);
            let nl = if with_newline { "\n" } else { "" };
            write!(w, "{}}}{}", get_depth_space(depth), nl)
        } else {
            panic!("todo")
        }
    }
}

#[derive(Debug, Clone)]
pub struct EBNode {
    pub environment: Environment,
    pub expression: Expression,
}
impl EBNode {
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let (split_position, operator) = self.expression.get_split_position();
        let (former, latter) = self.expression.get_splitted_expression(split_position);
        let former_val = former.clone().get_val(self.environment.clone());
        let latter_val = latter.clone().get_val(self.environment.clone());

        match operator.as_ref() {
            "+" => {
                let i1 = former_val.get_num();
                let i2 = latter_val.get_num();
                let _ = write!(
                    w,
                    "{}{}{} + {} evalto {} by E-Plus {{\n",
                    get_depth_space(depth),
                    self.environment.clone().to_string(),
                    former.clone().to_string(),
                    latter.clone().to_string(),
                    i1 + i2
                );
                let premise1 = RuleNode::new(self.environment.clone(), former);
                let premise2 = RuleNode::new(self.environment, latter);
                let _ = premise1.show(w, depth + 2, false);
                let _ = write!(w, ";\n");
                let _ = premise2.show(w, depth + 2, false);
                let _ = write!(w, ";\n");

                let premise = BPlusNode { i1, i2 };
                let _ = premise.show(w, depth + 2, true);

                let nl = if with_newline { "\n" } else { "" };
                write!(w, "{}}}{}", get_depth_space(depth), nl)
            }
            _ => panic!(""),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EValNode {
    pub environment: Environment,
    pub expression: Expression,
}
impl EValNode {
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let nl = if with_newline { "\n" } else { "" };
        let i = self
            .expression
            .clone()
            .get_val(self.environment.clone())
            .get_num();
        let mut terms = self.expression.clone().terms;
        let (operator, term) = terms.pop().expect("");
        match term {
            Term::Val(num) => write!(
                w,
                "{}{}{} evalto {} by E-Int {{}}{}",
                get_depth_space(depth),
                self.environment.clone().to_string(),
                self.expression.to_string(),
                i,
                nl
            ),
            _ => panic!("unexpected"),
        }
    }
}
