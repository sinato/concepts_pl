use super::lexer::{Token, Tokens};
use super::util::{get_depth_space, get_peano_num};

use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum RuleNode {
    EConst(EConstNode),
    EPlus(EPlusNode),
    ETimes(ETimesNode),
    PZero(PZeroNode),
    PSucc(PSuccNode),
    TZero(TZeroNode),
    TSucc(TSuccNode),
}

impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let terms = Terms::new(tokens);
        get_rule_eval(terms)
    }
    pub fn get_val(&self) -> usize {
        match self {
            RuleNode::EConst(node) => node.get_val(),
            RuleNode::EPlus(node) => node.get_val(),
            RuleNode::ETimes(node) => node.get_val(),
            RuleNode::PZero(node) => node.get_val(),
            RuleNode::PSucc(node) => node.get_val(),
            RuleNode::TZero(node) => node.get_val(),
            RuleNode::TSucc(node) => node.get_val(),
        }
    }
    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::EConst(node) => node.show(w, depth, with_newline),
            RuleNode::EPlus(node) => node.show(w, depth, with_newline),
            RuleNode::ETimes(node) => node.show(w, depth, with_newline),
            RuleNode::PZero(node) => node.show(w, depth, with_newline),
            RuleNode::PSucc(node) => node.show(w, depth, with_newline),
            RuleNode::TZero(node) => node.show(w, depth, with_newline),
            RuleNode::TSucc(node) => node.show(w, depth, with_newline),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Term {
    Leaf(String, usize),
    Node(String, Terms),
}
impl Term {
    fn new(tokens: &mut Tokens, operator: String) -> Term {
        match tokens.peek().expect("") {
            Token::ParenS(_) => {
                tokens.pop(); // consume (
                let terms = Terms::new(tokens);
                tokens.pop(); // consume )
                Term::Node(operator, terms)
            }
            Token::Ps(_) => {
                let num = tokens.consume_peano_num();
                Term::Leaf(operator, num)
            }
            Token::Zero(_) => {
                tokens.pop(); // consume Z
                Term::Leaf(operator, 0)
            }
            _ => panic!("unexpect"),
        }
    }
    fn get_operator(&self) -> String {
        match self {
            Term::Leaf(op, _) => op.to_string(),
            Term::Node(op, _) => op.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Terms {
    terms: Vec<Term>,
}
impl Terms {
    fn new(tokens: &mut Tokens) -> Terms {
        let terms: Vec<Term> = Vec::new();
        let mut terms = Terms { terms };
        terms.push(Term::new(tokens, "".to_string()));
        loop {
            match tokens.peek().expect("expects a tokens") {
                Token::OpC(operator, _) => {
                    tokens.pop(); // consume operator
                    terms.push(Term::new(tokens, operator));
                }
                Token::Eval(_) => {
                    tokens.pop(); // consume evalto
                    let _res = tokens.consume_peano_num(); // consume ans num. expect a number ( not an expression )
                    break;
                }
                Token::Pe(_) => {
                    break;
                }
                _ => panic!("unexpected token: {:?}"),
            }
        }
        terms
    }
    fn push(&mut self, term: Term) {
        self.terms.push(term)
    }
    fn pop(&mut self) -> Option<Term> {
        self.terms.pop()
    }
    fn len(&mut self) -> usize {
        self.terms.len()
    }
    fn get_split_position(&self) -> (usize, String) {
        let mut priorities: HashMap<String, usize> = HashMap::new();
        priorities.insert("".to_string(), 0);
        priorities.insert("*".to_string(), 10);
        priorities.insert("+".to_string(), 20);

        let mut split_position = 0;
        let mut priority: usize = 0;
        let mut ret_op: String = "".to_string();
        let terms = self.terms.clone();
        for (i, term) in terms.into_iter().enumerate() {
            let operator = term.get_operator();
            if priority <= *priorities.get(&operator).expect("") {
                split_position = i;
                priority = *priorities.get(&operator).expect("");
                ret_op = operator;
            }
        }
        (split_position, ret_op.to_string())
    }
    fn get_splitted_terms(&self, split_position: usize) -> (Terms, Terms) {
        let mut former: Vec<Term> = Vec::new();
        let mut latter: Vec<Term> = Vec::new();

        let terms = self.terms.clone();
        for (i, term) in terms.into_iter().enumerate() {
            if i < split_position {
                former.push(term);
            } else {
                latter.push(term);
            }
        }
        let former = Terms { terms: former };
        let mut latter = Terms { terms: latter };
        latter.rm_first_operator();
        (former, latter)
    }
    fn to_string(self) -> String {
        let mut s = "".to_string();
        let terms = self.terms.into_iter();
        for term in terms {
            match term {
                Term::Leaf(operator, num) => {
                    let snum = &get_peano_num(num);
                    match operator.as_ref() {
                        "" => s += snum,
                        "+" => s = s + " + " + snum,
                        "*" => s = s + " * " + snum,
                        _ => panic!("TODO"),
                    }
                }
                Term::Node(operator, terms) => {
                    match operator.as_ref() {
                        "" => (),
                        "+" => s = s + " + ",
                        "*" => s = s + " * ",
                        _ => panic!("TODO"),
                    }
                    s += "(";
                    s += &terms.to_string();
                    s += ")";
                }
            }
        }
        s
    }
    fn rm_first_operator(&mut self) {
        let mut new_terms: Vec<Term> = Vec::new();
        let terms = self.terms.clone().into_iter();
        for (i, term) in terms.enumerate() {
            if i == 0 {
                match term {
                    Term::Leaf(_, num) => new_terms.push(Term::Leaf(String::from(""), num)),
                    Term::Node(_, v) => new_terms.push(Term::Node(String::from(""), v)),
                }
            } else {
                new_terms.push(term);
            }
        }
        self.terms = new_terms;
    }
}

fn get_rule_eval(mut terms: Terms) -> RuleNode {
    let terms_clone = terms.clone();
    if terms.len() == 1 {
        let term = terms.pop().expect("");
        match term {
            Term::Leaf(_, num) => RuleNode::EConst(EConstNode { n: num }),
            Term::Node(_, terms) => get_rule_eval(terms),
        }
    } else {
        let (split_position, split_operator) = terms.get_split_position();
        let (former, latter) = terms.get_splitted_terms(split_position);

        let premise_term1 = get_rule_eval(former);
        let premise_term2 = get_rule_eval(latter);

        match split_operator.as_ref() {
            "+" => RuleNode::EPlus(EPlusNode {
                terms: terms_clone,
                premise_term1: Box::new(premise_term1),
                premise_term2: Box::new(premise_term2),
            }),
            "*" => RuleNode::ETimes(ETimesNode {
                terms: terms_clone,
                premise_term1: Box::new(premise_term1),
                premise_term2: Box::new(premise_term2),
            }),
            _ => panic!("unexpected operator"),
        }
    }
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

#[derive(Debug, PartialEq, Clone)]
pub struct EConstNode {
    n: usize,
}
impl EConstNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n = get_peano_num(self.n);
        let nl = if with_newline { "\n" } else { "" };
        write!(
            w,
            "{}{} evalto {} by E-Const {{}}{}",
            get_depth_space(depth),
            n,
            n,
            nl,
        )
    }
    fn get_val(&self) -> usize {
        self.n
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EPlusNode {
    terms: Terms,
    premise_term1: Box<RuleNode>,
    premise_term2: Box<RuleNode>,
}
impl EPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n1 = self.premise_term1.get_val();
        let n2 = self.premise_term2.get_val();
        let _ = write!(
            w,
            "{}{} evalto {} by E-Plus {{\n",
            get_depth_space(depth),
            self.terms.to_string(),
            get_peano_num(n1 + n2)
        );

        let _ = self.premise_term1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = self.premise_term2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");

        let premise = get_rule_plus(n1, n2);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
    fn get_val(&self) -> usize {
        self.premise_term1.get_val() + self.premise_term2.get_val()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ETimesNode {
    terms: Terms,
    premise_term1: Box<RuleNode>,
    premise_term2: Box<RuleNode>,
}
impl ETimesNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let n1 = self.premise_term1.get_val();
        let n2 = self.premise_term2.get_val();
        let _ = write!(
            w,
            "{}{} evalto {} by E-Times {{\n",
            get_depth_space(depth),
            self.terms.to_string(),
            get_peano_num(n1 * n2)
        );

        let _ = self.premise_term1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = self.premise_term2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");

        let premise = get_rule_times(n1, n2);
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
    fn get_val(&self) -> usize {
        self.premise_term1.get_val() * self.premise_term2.get_val()
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
    fn get_val(&self) -> usize {
        self.n
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
    fn get_val(&self) -> usize {
        self.n1 + self.n2
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
    fn get_val(&self) -> usize {
        self.n
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
    fn get_val(&self) -> usize {
        self.n1 * self.n2
    }
}
