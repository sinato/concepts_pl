use super::super::environment::Environment;
use super::super::expression::Expression;
use super::super::nodes::RuleNode;
use super::super::terms::{AppTerm, FunTerm, IfTerm, LetTerm, Term};
use super::super::value::Value;
use super::bnodes::BOpNode;
use super::writer::RuleWriter;

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

        let (premise, rule_str) = if self.environment.get_match_loc(&identifier) == 0 {
            (None, "E-Var1".to_string())
        } else {
            let mut new_env = self.environment.clone();
            new_env.pop_val();
            (
                Some(RuleNode::new(new_env, self.expression)),
                "E-Var2".to_string(),
            )
        };
        writer.show_rule(
            Some(self.environment.clone()),
            identifier.clone(),
            self.environment.get_val(&identifier).to_string(),
            rule_str,
            false,
            premise,
            None,
            None,
        )
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
            Value::Bool(b) => b,
            _ => panic!("unexpected"),
        };

        if flag == String::from("true") {
            let condition_premise = RuleNode::new(self.environment.clone(), condition_expression);
            let then_premise = RuleNode::new(self.environment.clone(), then_expression.clone());
            writer.show_rule(
                Some(self.environment.clone()),
                self.expression.clone().to_string(&self.environment),
                then_expression.get_val(self.environment).to_string(),
                "E-IfT".to_string(),
                false,
                Some(condition_premise),
                Some(then_premise),
                None,
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

        let premise = RuleNode::BOp(BOpNode {
            i1,
            i2,
            op: operator.clone(),
        });
        let (val_str, rule_str) = match operator.as_ref() {
            "+" => ((i1 + i2).to_string(), "E-Plus".to_string()),
            "*" => ((i1 * i2).to_string(), "E-Times".to_string()),
            "-" => ((i1 - i2).to_string(), "E-Minus".to_string()),
            _ => panic!("todo"),
        };
        writer.show_rule(
            Some(self.environment.clone()),
            self.expression.to_string(&self.environment),
            val_str,
            rule_str,
            false,
            Some(premise1),
            Some(premise2),
            Some(premise),
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
        match term {
            Term::Val(_) => writer.show_rule(
                Some(self.environment.clone()),
                self.expression.clone().to_string(&self.environment),
                self.expression
                    .get_val(self.environment)
                    .get_num()
                    .to_string(),
                "E-Int".to_string(),
                false,
                None,
                None,
                None,
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
        let val = let_expression
            .expression
            .clone()
            .get_val(self.environment.clone());
        new_env.set_val(let_expression.identifier, val);
        let let_premise = RuleNode::new(self.environment.clone(), let_expression.expression);
        let in_premise = RuleNode::new(new_env.clone(), in_expression.clone());
        let val = self.term.get_val(self.environment.clone()).to_string();

        writer.show_rule(
            Some(self.environment.clone()),
            self.expression.clone().to_string(&self.environment),
            val,
            "E-Let".to_string(),
            false,
            Some(let_premise),
            Some(in_premise),
            None,
        )
    }
}

#[derive(Debug, Clone)]
pub struct EFunNode {
    pub environment: Environment,
    pub expression: Expression,
    pub term: FunTerm,
}
impl EFunNode {
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        writer.show_rule(
            Some(self.environment.clone()),
            self.term.clone().to_string(&self.environment),
            self.term.get_val(self.environment).to_string(),
            "E-Fun".to_string(),
            false,
            None,
            None,
            None,
        )
    }
}

#[derive(Debug, Clone)]
pub struct EAppNode {
    pub environment: Environment,
    pub expression: Expression,
    pub term: AppTerm,
}
impl EAppNode {
    pub fn show<W: Write>(self, writer: &mut RuleWriter<W>) -> io::Result<()> {
        let (fun_term, mut clojure_env) = self.term.get_fun_info(self.environment.clone());

        let terms: Vec<(String, Term)> = vec![("".to_string(), *self.term.clone().function)];
        let premise1 = RuleNode::new(self.environment.clone(), Expression { terms });

        let premise2 = RuleNode::new(self.environment.clone(), self.term.clone().argument);
        let parameter: String = fun_term.parameter;
        clojure_env.set_val(
            parameter,
            self.term.clone().argument.get_val(self.environment.clone()),
        );
        let premise3 = RuleNode::new(clojure_env, fun_term.function_body);
        writer.show_rule(
            Some(self.environment.clone()),
            self.term.clone().to_string(&self.environment),
            self.term.get_val(self.environment).to_string(),
            "E-App".to_string(),
            false,
            Some(premise1),
            Some(premise2),
            Some(premise3),
        )
    }
}
