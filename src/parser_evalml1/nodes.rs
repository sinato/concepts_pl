use super::lexer::Tokens;
use super::terms::{IfTerms, Term, Terms};
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum RuleNode {
    EInt(EIntNode),
    EIfT(EIfTNode),
    EPlus(EPlusNode),
    ETimes(ETimesNode),
    EMinus(EMinusNode),
    ELt(ELtNode),
}
impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let terms: Terms = Terms::new(tokens);
        let expression = Expression::new(terms.clone(), terms);
        expression.get_rule()
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::EInt(node) => node.show(w, depth, with_newline),
            RuleNode::EIfT(node) => node.show(w, depth, with_newline),
            RuleNode::EPlus(node) => node.show(w, depth, with_newline),
            RuleNode::ETimes(node) => node.show(w, depth, with_newline),
            RuleNode::EMinus(node) => node.show(w, depth, with_newline),
            RuleNode::ELt(node) => node.show(w, depth, with_newline),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Int(i32, Terms),
    Bin(String, Box<Expression>, Box<Expression>, Terms),
    If(Box<Expression>, Box<Expression>, Box<Expression>, Terms),
}
impl Expression {
    fn new(mut terms: Terms, origin_terms: Terms) -> Expression {
        if terms.len() == 1 {
            let term = terms.pop().expect("");
            match term {
                Term::Single(_, num) => Expression::Int(num, origin_terms),
                Term::Paren(_, terms) => Expression::new(terms, origin_terms),
                Term::If(_, if_terms) => Expression::create_if(if_terms, origin_terms),
            }
        } else {
            let (split_position, split_operator) = terms.get_split_position();
            let (former, latter) = terms.get_splitted_terms(split_position);
            println!("================================");
            println!("before: {:?}", former);
            println!("after: {:?}", latter);
            println!("================================");
            let e1 = Expression::new(former.clone(), former);
            let e2 = Expression::new(latter.clone(), latter);
            Expression::Bin(split_operator, Box::new(e1), Box::new(e2), origin_terms)
        }
    }

    fn create_if(if_terms: IfTerms, origin_terms: Terms) -> Expression {
        let condition_exp = Box::new(Expression::new(
            if_terms.condition_terms,
            origin_terms.clone(),
        ));
        let if_exp = Box::new(Expression::new(if_terms.then_terms, origin_terms.clone()));
        let else_exp = Box::new(Expression::new(if_terms.else_terms, origin_terms.clone()));
        Expression::If(condition_exp, if_exp, else_exp, origin_terms)
    }
    fn get_val(&self) -> i32 {
        match self {
            Expression::Int(val, _) => *val,
            Expression::Bin(operator, box_ex1, box_ex2, _) => {
                let val1 = box_ex1.get_val();
                let val2 = box_ex2.get_val();
                match operator.as_ref() {
                    "+" => val1 + val2,
                    "-" => val1 - val2,
                    "*" => val1 * val2,
                    "<" => {
                        if val1 < val2 {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("TODO"),
                }
            }
            Expression::If(box_condition_exp, box_if_exp, box_else_exp, _) => {
                if box_condition_exp.get_val() == 1 {
                    box_if_exp.get_val()
                } else {
                    box_else_exp.get_val()
                }
            }
        }
    }
    fn get_rule(self) -> RuleNode {
        match self {
            Expression::Int(i, _) => RuleNode::EInt(EIntNode { i }),
            Expression::Bin(operator, box_ex1, box_ex2, _) => {
                let e1 = *box_ex1;
                let e2 = *box_ex2;
                match operator.as_ref() {
                    "+" => RuleNode::EPlus(EPlusNode { e1, e2 }),
                    "*" => RuleNode::ETimes(ETimesNode { e1, e2 }),
                    "-" => RuleNode::EMinus(EMinusNode { e1, e2 }),
                    "<" => RuleNode::ELt(ELtNode { e1, e2 }),
                    _ => panic!("todo"),
                }
            }
            Expression::If(box_condition_exp, box_if_exp, box_else_exp, _) => {
                if box_condition_exp.get_val() == 1 {
                    RuleNode::EIfT(EIfTNode {
                        condition_exp: *box_condition_exp,
                        then_exp: *box_if_exp,
                        else_exp: *box_else_exp,
                    })
                } else {
                    panic!("todo")
                }
            }
        }
    }
    fn to_string(self) -> String {
        let origin_terms = match self {
            Expression::Int(_, terms) => terms,
            Expression::Bin(_, _, _, terms) => terms,
            Expression::If(_, _, _, terms) => terms,
        };
        origin_terms.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct EIntNode {
    i: i32,
}
impl EIntNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let nl = if with_newline { "\n" } else { "" };
        write!(
            w,
            "{}{} evalto {} by E-Int {{}}{}",
            get_depth_space(depth),
            self.i,
            self.i,
            nl
        )
    }
}

#[derive(Debug, Clone)]
pub struct EBoolNode {
    b: bool,
}
impl EBoolNode {
    fn _show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let nl = if with_newline { "\n" } else { "" };
        write!(
            w,
            "{}{} evalto {} by E-Bool {{}}{}",
            get_depth_space(depth),
            self.b,
            self.b,
            nl
        )
    }
}

#[derive(Debug, Clone)]
pub struct EIfTNode {
    condition_exp: Expression,
    then_exp: Expression,
    else_exp: Expression,
}
impl EIfTNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} evalto {} by E-IfT {{\n",
            get_depth_space(depth),
            self.condition_exp.clone().to_string(),
            self.then_exp.get_val()
        );
        let condition_premise = self.condition_exp.get_rule();
        let then_premise = self.then_exp.get_rule();
        let _ = condition_premise.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = then_premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EPlusNode {
    e1: Expression,
    e2: Expression,
}
impl EPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val();
        let i2 = self.e2.get_val();
        let _ = write!(
            w,
            "{}{} + {} evalto {} by E-Plus {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            i1 + i2
        );
        let premise1 = self.e1.get_rule();
        let premise2 = self.e2.get_rule();
        let _ = premise1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = premise2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");

        let premise = BPlusNode { i1, i2 };
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct ETimesNode {
    e1: Expression,
    e2: Expression,
}
impl ETimesNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val();
        let i2 = self.e2.get_val();
        let _ = write!(
            w,
            "{}{} * {} evalto {} by E-Times {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            i1 * i2
        );
        let premise1 = self.e1.get_rule();
        let premise2 = self.e2.get_rule();
        let _ = premise1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = premise2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");

        let premise = BTimesNode { i1, i2 };
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct ELtNode {
    e1: Expression,
    e2: Expression,
}
impl ELtNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val();
        let i2 = self.e2.get_val();
        let b = if i1 < i2 { "true" } else { "false" };
        let _ = write!(
            w,
            "{}{} < {} evalto {} by E-Lt {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            b
        );
        let premise1 = self.e1.get_rule();
        let premise2 = self.e2.get_rule();
        let _ = premise1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = premise2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");

        let premise = BLtNode { i1, i2 };
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EMinusNode {
    e1: Expression,
    e2: Expression,
}
impl EMinusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val();
        let i2 = self.e2.get_val();
        let _ = write!(
            w,
            "{}{} - {} evalto {} by E-Minus {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            i1 - i2
        );
        let premise1 = self.e1.get_rule();
        let premise2 = self.e2.get_rule();
        let _ = premise1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = premise2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");

        let premise = BMinusNode { i1, i2 };
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct BPlusNode {
    i1: i32,
    i2: i32,
}
impl BPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        write!(
            w,
            "{}{} plus {} is {} by B-Plus {{}}{}",
            get_depth_space(depth),
            self.i1,
            self.i2,
            self.i1 + self.i2,
            if with_newline { "\n" } else { "" }
        )
    }
}

#[derive(Debug, Clone)]
pub struct BTimesNode {
    i1: i32,
    i2: i32,
}
impl BTimesNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        write!(
            w,
            "{}{} times {} is {} by B-Times {{}}{}",
            get_depth_space(depth),
            self.i1,
            self.i2,
            self.i1 * self.i2,
            if with_newline { "\n" } else { "" }
        )
    }
}

#[derive(Debug, Clone)]
pub struct BMinusNode {
    i1: i32,
    i2: i32,
}
impl BMinusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        write!(
            w,
            "{}{} minus {} is {} by B-Minus {{}}{}",
            get_depth_space(depth),
            self.i1,
            self.i2,
            self.i1 - self.i2,
            if with_newline { "\n" } else { "" }
        )
    }
}

#[derive(Debug, Clone)]
pub struct BLtNode {
    i1: i32,
    i2: i32,
}
impl BLtNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let val = if self.i1 < self.i2 { "true" } else { "false" };
        write!(
            w,
            "{}{} less than {} is {} by B-Lt {{}}{}",
            get_depth_space(depth),
            self.i1,
            self.i2,
            val,
            if with_newline { "\n" } else { "" }
        )
    }
}

pub fn get_depth_space(depth: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..depth {
        s += " ";
    }
    s
}
