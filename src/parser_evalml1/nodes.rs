use super::lexer::Tokens;
use super::terms::{Term, Terms};
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum RuleNode {
    EInt(EIntNode),
    EPlus(EPlusNode),
    ETimes(ETimesNode),
    EMinus(EMinusNode),
}
impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let terms: Terms = Terms::new(tokens);
        let expression = Expression::new(terms.clone(), terms);
        match expression {
            Expression::Int(i, _) => RuleNode::EInt(EIntNode { i }),
            Expression::Bin(operator, box_ex1, box_ex2, _) => {
                let e1 = *box_ex1;
                let e2 = *box_ex2;
                match operator.as_ref() {
                    "+" => RuleNode::EPlus(EPlusNode { e1, e2 }),
                    "*" => RuleNode::ETimes(ETimesNode { e1, e2 }),
                    "-" => RuleNode::EMinus(EMinusNode { e1, e2 }),
                    _ => panic!("todo"),
                }
            }
        }
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::EInt(node) => node.show(w, depth, with_newline),
            RuleNode::EPlus(node) => node.show(w, depth, with_newline),
            RuleNode::ETimes(node) => node.show(w, depth, with_newline),
            RuleNode::EMinus(node) => node.show(w, depth, with_newline),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Int(i32, Terms),
    Bin(String, Box<Expression>, Box<Expression>, Terms),
}
impl Expression {
    fn new(mut terms: Terms, origin_terms: Terms) -> Expression {
        if terms.len() == 1 {
            let term = terms.pop().expect("");
            match term {
                Term::Leaf(_, num) => Expression::Int(num, origin_terms),
                Term::Node(_, terms) => Expression::new(terms, origin_terms),
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
    fn get_val(&self) -> i32 {
        match self {
            Expression::Int(val, _) => *val,
            Expression::Bin(operator, box_ex1, box_ex2, _) => {
                let val1 = box_ex1.get_val();
                let val2 = box_ex2.get_val();
                match operator.as_ref() {
                    "+" => val1 + val2,
                    "-" => val1 - val2,
                    _ => panic!("TODO"),
                }
            }
        }
    }
    fn get_rule(self) -> RuleNode {
        match self {
            Expression::Int(i, _) => RuleNode::EInt(EIntNode { i }),
            Expression::Bin(operator, box_ex1, box_ex2, _) => match operator.as_ref() {
                "+" => RuleNode::EPlus(EPlusNode {
                    e1: *box_ex1,
                    e2: *box_ex2,
                }),
                "-" => RuleNode::EMinus(EMinusNode {
                    e1: *box_ex1,
                    e2: *box_ex2,
                }),
                _ => panic!(""),
            },
        }
    }
    fn to_string(self) -> String {
        let origin_terms = match self {
            Expression::Int(_, terms) => terms,
            Expression::Bin(_, _, _, terms) => terms,
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

pub fn get_depth_space(depth: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..depth {
        s += " ";
    }
    s
}
