use super::lexer::{Token, Tokens};
use super::util::{get_depth_space, get_peano_num};

use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum RuleNode {
    MROne(MROneNode),
    DRTimes(DRTimesNode),
    DRPlusL(DRPlusLNode),
    RPlus(RPlusNode),
    RPlusR(RPlusRNode),
    RTimes(RTimesNode),
    PZero(PZeroNode),
    PSucc(PSuccNode),
    TZero(TZeroNode),
    TSucc(TSuccNode),
}

impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let before_terms = Terms::new(tokens);
        let eval_op = match tokens.pop().expect("eval operator") {
            Token::EvalMR(_) => "-*->".to_string(),
            Token::EvalDR(_) => "-d->".to_string(),
            Token::EvalONE(_) => "--->".to_string(),
            _ => panic!("TODO"),
        };
        let after_terms = Terms::new(tokens);

        match eval_op.as_ref() {
            "-*->" => RuleNode::MROne(MROneNode {
                before_terms,
                after_terms,
            }),
            "-d->" => RuleNode::DRPlusL(DRPlusLNode {
                before_terms,
                after_terms,
            }),
            "--->" => RuleNode::RPlusR(RPlusRNode {
                before_terms,
                after_terms,
            }),
            _ => panic!("TODO"),
        }
    }
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::MROne(node) => node.show(w, depth, with_newline),
            RuleNode::DRTimes(node) => node.show(w, depth, with_newline),
            RuleNode::DRPlusL(node) => node.show(w, depth, with_newline),
            RuleNode::RPlus(node) => node.show(w, depth, with_newline),
            RuleNode::RPlusR(node) => node.show(w, depth, with_newline),
            RuleNode::RTimes(node) => node.show(w, depth, with_newline),
            RuleNode::PZero(node) => node.show(w, depth, with_newline),
            RuleNode::PSucc(node) => node.show(w, depth, with_newline),
            RuleNode::TZero(node) => node.show(w, depth, with_newline),
            RuleNode::TSucc(node) => node.show(w, depth, with_newline),
        }
    }
}

fn get_rplus(terms: &mut Terms) -> RuleNode {
    let (_, n1) = terms.pop().expect("");
    let (_, n2) = terms.pop().expect("");
    RuleNode::RPlus(RPlusNode { n1, n2 })
}

fn get_drtimes(terms: &mut Terms) -> RuleNode {
    let (_, n1) = terms.pop().expect("");
    let (_, n2) = terms.pop().expect("");
    RuleNode::DRTimes(DRTimesNode { n1, n2 })
}

fn get_rtimes_r(terms: &mut Terms) -> RuleNode {
    terms.reverse();
    let (_, n1) = terms.pop().expect("");
    let (_, n2) = terms.pop().expect("");
    terms.reverse();
    RuleNode::RTimes(RTimesNode { n1, n2 })
}

fn get_rule_plus(n1: usize, n2: usize) -> RuleNode {
    if n1 == 0 {
        RuleNode::PZero(PZeroNode { n: n2 })
    } else {
        RuleNode::PSucc(PSuccNode { n1, n2 })
    }
}

fn get_rule_times(n1: usize, n2: usize) -> RuleNode {
    if n1 == 0 {
        RuleNode::TZero(TZeroNode { n: n2 })
    } else {
        RuleNode::TSucc(TSuccNode { n1, n2 })
    }
}

#[derive(Debug, Clone)]
pub struct DRTimesNode {
    n1: usize,
    n2: usize,
}
impl DRTimesNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} * {} -d-> {} by DR-Times {{\n",
            get_depth_space(depth),
            get_peano_num(self.n1),
            get_peano_num(self.n2),
            get_peano_num(self.n1 * self.n2),
        );

        let premise = get_rule_times(self.n1, self.n2);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct DRPlusLNode {
    before_terms: Terms,
    after_terms: Terms,
}
impl DRPlusLNode {
    fn show<W: Write>(mut self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} -d-> {} by DR-PlusL {{\n",
            get_depth_space(depth),
            self.before_terms.clone().to_string(),
            self.after_terms.to_string(),
        );

        let premise = get_drtimes(&mut self.before_terms);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct MROneNode {
    before_terms: Terms,
    after_terms: Terms,
}
impl MROneNode {
    fn show<W: Write>(mut self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} -*-> {} by MR-One {{\n",
            get_depth_space(depth),
            self.before_terms.clone().to_string(),
            self.after_terms.to_string(),
        );

        let premise = get_rplus(&mut self.before_terms);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct RPlusNode {
    n1: usize,
    n2: usize,
}
impl RPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} + {} ---> {} by R-Plus {{\n",
            get_depth_space(depth),
            get_peano_num(self.n1),
            get_peano_num(self.n2),
            get_peano_num(self.n1 + self.n2),
        );

        let premise = get_rule_plus(self.n1, self.n2);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct RPlusRNode {
    before_terms: Terms,
    after_terms: Terms,
}
impl RPlusRNode {
    fn show<W: Write>(mut self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} ---> {} by R-PlusR {{\n",
            get_depth_space(depth),
            self.before_terms.clone().to_string(),
            self.after_terms.to_string(),
        );

        let premise = get_rtimes_r(&mut self.before_terms);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct RTimesNode {
    n1: usize,
    n2: usize,
}
impl RTimesNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} * {} ---> {} by R-Times {{\n",
            get_depth_space(depth),
            get_peano_num(self.n1),
            get_peano_num(self.n2),
            get_peano_num(self.n1 * self.n2),
        );

        let premise = get_rule_times(self.n1, self.n2);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PZeroNode {
    n: usize,
}
impl PZeroNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n = get_peano_num(self.n);
        let _ = write!(w, "{}", get_depth_space(depth));
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "Z plus {} is {} by P-Zero {{}}{}", n, n, nl)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PSuccNode {
    n1: usize,
    n2: usize,
}
impl PSuccNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} plus {} is {} by P-Succ {{\n",
            get_depth_space(depth),
            get_peano_num(self.n1),
            get_peano_num(self.n2),
            get_peano_num(self.n1 + self.n2),
        );
        let premise = get_rule_plus(self.n1 - 1, self.n2);
        let _ = premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct TZeroNode {
    n: usize,
}
impl TZeroNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n = get_peano_num(self.n);
        let _ = write!(w, "{}", get_depth_space(depth));
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "Z times {} is Z by T-Zero {{}}{}", n, nl)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TSuccNode {
    n1: usize,
    n2: usize,
}
impl TSuccNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} times {} is {} by T-Succ {{\n",
            get_depth_space(depth),
            get_peano_num(self.n1),
            get_peano_num(self.n2),
            get_peano_num(self.n1 * self.n2),
        );
        let premise1 = get_rule_times(self.n1 - 1, self.n2);
        let _ = premise1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let premise2 = get_rule_plus(self.n2, (self.n1 - 1) * self.n2);
        let _ = premise2.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
struct Terms {
    terms: Vec<(String, usize)>,
}
impl Terms {
    fn new(tokens: &mut Tokens) -> Terms {
        let mut terms: Vec<(String, usize)> = Vec::new();
        let num = tokens.consume_peano_num();
        terms.push(("".to_string(), num));
        loop {
            match tokens.peek() {
                Some(token) => match token {
                    Token::OpC(operator, _) => {
                        tokens.pop(); // consume operator
                        let num = tokens.consume_peano_num();
                        terms.push((operator, num));
                    }
                    Token::EvalMR(_) => break,
                    Token::EvalDR(_) => break,
                    Token::EvalONE(_) => break,
                    _ => panic!("unexpected token: {:?}"),
                },
                None => break,
            }
        }
        Terms { terms }
    }
    fn pop(&mut self) -> Option<(String, usize)> {
        self.terms.reverse();
        let term = self.terms.pop();
        self.terms.reverse();
        term
    }
    fn reverse(&mut self) {
        self.terms.reverse();
    }
    fn to_string(self) -> String {
        let mut s = "".to_string();
        let terms = self.terms.into_iter();
        for (operator, num) in terms {
            let snum = &get_peano_num(num);
            match operator.as_ref() {
                "" => s += snum,
                "+" => s = s + " + " + snum,
                "*" => s = s + " * " + snum,
                _ => panic!("TODO"),
            }
        }
        s
    }
}
