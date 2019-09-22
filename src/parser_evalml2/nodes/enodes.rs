use super::super::environment::Environment;
use super::super::expression::Expression;
use super::super::nodes::{get_depth_space, RuleNode};
use super::super::terms::{IfTerm, LetTerm, Term};
use super::super::value::Value;
use super::bnodes::BOpNode;

use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct EVarNode {
    pub environment: Environment,
    pub expression: Expression,
}
impl EVarNode {
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        let mut terms = self.expression.terms.clone();
        let (_, term) = terms.pop().expect("");
        let identifier = term.get_identifier();
        let rule_str = "E-Var1".to_string();

        match self.environment.get_num() {
            2 => match identifier.as_ref() {
                "x" => {
                    let mut new_env = self.environment.clone();
                    new_env.y = None;
                    let premise = RuleNode::new(new_env, self.expression);

                    writer.show_rule_with_premise(
                        self.environment.clone(),
                        identifier.clone(),
                        self.environment
                            .get_val(String::from(identifier))
                            .to_string(),
                        "E-Var2".to_string(),
                        premise,
                    )
                }
                "y" => writer.show_rule_without_premise(
                    Some(self.environment.clone()),
                    identifier.clone(),
                    self.environment.get_val(identifier).to_string(),
                    rule_str,
                    false,
                ),
                _ => panic!("unexpected"),
            },
            1 => writer.show_rule_without_premise(
                Some(self.environment.clone()),
                identifier.clone(),
                self.environment.get_val(identifier).to_string(),
                rule_str,
                false,
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
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        let condition_expression = self.term.condition_expression;
        let then_expression = self.term.then_expression;
        let _else_expression = self.term.else_expression;

        let flag: String = match condition_expression
            .clone()
            .get_val(self.environment.clone())
        {
            Value::Num(_) => panic!("unexpected"),
            Value::Bool(b) => b,
        };

        if flag == String::from("true") {
            let condition_premise = RuleNode::new(self.environment.clone(), condition_expression);
            let then_premise = RuleNode::new(self.environment.clone(), then_expression.clone());
            writer.show_rule_with_premise2(
                self.environment.clone(),
                self.expression.clone().to_string(),
                then_expression.get_val(self.environment).to_string(),
                "E-IfT".to_string(),
                condition_premise,
                then_premise,
            )
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
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        let (split_position, operator) = self.expression.get_split_position();
        let (former, latter) = self.expression.get_splitted_expression(split_position);

        let i1 = former.clone().get_val(self.environment.clone()).get_num();
        let i2 = latter.clone().get_val(self.environment.clone()).get_num();
        let premise1 = RuleNode::new(self.environment.clone(), former);
        let premise2 = RuleNode::new(self.environment.clone(), latter);

        let premise = BOpNode {
            i1,
            i2,
            op: operator.clone(),
        };
        let (val_str, rule_str) = match operator.as_ref() {
            "+" => ((i1 + i2).to_string(), "E-Plus".to_string()),
            "*" => ((i1 * i2).to_string(), "E-Times".to_string()),
            _ => panic!("todo"),
        };
        writer.show_rule_with_premise3(
            self.environment,
            self.expression.to_string(),
            val_str,
            rule_str,
            premise1,
            premise2,
            premise,
        )
    }
}

#[derive(Debug, Clone)]
pub struct EValNode {
    pub environment: Environment,
    pub expression: Expression,
}
impl EValNode {
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        let mut terms = self.expression.clone().terms;
        let (_, term) = terms.pop().expect("");
        let rule_str = "E-Int".to_string();
        match term {
            Term::Val(_) => writer.show_rule_without_premise(
                Some(self.environment.clone()),
                self.expression.clone().to_string(),
                self.expression
                    .get_val(self.environment)
                    .get_num()
                    .to_string(),
                rule_str,
                false,
            ),
            _ => panic!("unexpected"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ELetNode {
    pub environment: Environment,
    pub expression: Expression,
    pub term: LetTerm,
}
impl ELetNode {
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        let in_expression = self.term.clone().in_expression;
        let let_expression = self.term.clone().let_expression;

        let mut new_env = self.environment.clone();
        match let_expression.identifier.as_ref() {
            "x" => {
                new_env.x = Some(
                    let_expression
                        .clone()
                        .expression
                        .get_val(self.clone().environment),
                )
            }
            "y" => {
                new_env.y = Some(
                    let_expression
                        .clone()
                        .expression
                        .get_val(self.clone().environment),
                )
            }
            _ => panic!("unexpected identifier"),
        }
        let let_premise =
            RuleNode::new(self.environment.clone(), let_expression.clone().expression);
        let in_premise = RuleNode::new(new_env.clone(), in_expression.clone());
        writer.show_rule_with_premise2(
            self.environment.clone(),
            self.expression.clone().to_string(),
            in_expression.get_val(new_env).to_string(),
            "E-Let".to_string(),
            let_premise,
            in_premise,
        )
    }
}

pub struct RuleWriter<W> {
    w: W,
    depth: usize,
}
impl<W: Write> RuleWriter<W> {
    pub fn new(w: W, depth: usize) -> RuleWriter<W> {
        RuleWriter { w, depth }
    }

    pub fn write_nl(&mut self) {
        let _ = write!(self.w, "\n");
    }

    fn inc_depth(&mut self) {
        self.depth += 2;
    }
    fn dec_depth(&mut self) {
        self.depth -= 2;
    }

    pub fn show_rule_without_premise(
        &mut self,
        environment: Option<Environment>,
        expression_str: String,
        evalto_str: String,
        rule_str: String,
        is_bnode: bool,
    ) -> io::Result<()> {
        let environment_str = match environment.clone() {
            Some(env) => env.to_string(),
            None => "".to_string(),
        };
        let eq_str = if is_bnode { "is" } else { "evalto" };
        write!(
            self.w,
            "{}{}{} {} {} by {} {{}}",
            get_depth_space(self.depth),
            environment_str,
            expression_str,
            eq_str.to_string(),
            evalto_str,
            rule_str,
        )
    }

    pub fn show_rule_with_premise(
        &mut self,
        environment: Environment,
        expression_str: String,
        evalto_str: String,
        rule_str: String,
        premise: RuleNode,
    ) -> io::Result<()> {
        let _ = write!(
            self.w,
            "{}{}{} evalto {} by {} {{\n",
            get_depth_space(self.depth),
            environment.clone().to_string(),
            expression_str,
            evalto_str,
            rule_str,
        );
        self.inc_depth();
        let _ = premise.show(self);
        let _ = write!(self.w, "\n");
        self.dec_depth();
        write!(self.w, "{}}}", get_depth_space(self.depth))
    }

    pub fn show_rule_with_premise2(
        &mut self,
        environment: Environment,
        expression_str: String,
        evalto_str: String,
        rule_str: String,
        premise1: RuleNode,
        premise2: RuleNode,
    ) -> io::Result<()> {
        let _ = write!(
            self.w,
            "{}{}{} evalto {} by {} {{\n",
            get_depth_space(self.depth),
            environment.clone().to_string(),
            expression_str,
            evalto_str,
            rule_str,
        );
        self.inc_depth();
        let _ = premise1.show(self);
        let _ = write!(self.w, ";\n");
        let _ = premise2.show(self);
        let _ = write!(self.w, "\n");
        self.dec_depth();
        write!(self.w, "{}}}", get_depth_space(self.depth))
    }

    pub fn show_rule_with_premise3(
        &mut self,
        environment: Environment,
        expression_str: String,
        evalto_str: String,
        rule_str: String,
        premise1: RuleNode,
        premise2: RuleNode,
        premise: BOpNode,
    ) -> io::Result<()> {
        let _ = write!(
            self.w,
            "{}{}{} evalto {} by {} {{\n",
            get_depth_space(self.depth),
            environment.clone().to_string(),
            expression_str,
            evalto_str,
            rule_str,
        );
        self.inc_depth();
        let _ = premise1.show(self);
        let _ = write!(self.w, ";\n");
        let _ = premise2.show(self);
        let _ = write!(self.w, ";\n");
        let _ = premise.show(self);
        let _ = write!(self.w, "\n");
        self.dec_depth();
        write!(self.w, "{}}}", get_depth_space(self.depth))
    }
}
