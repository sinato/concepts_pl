use super::lexer::{Token, Tokens};
use super::terms::{IfTerms, Term, Terms};
use std::io::{self, Write};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub enum RuleNode {
    EInt(EIntNode),
    EBool(EBoolNode),
    EVar1(EVar1Node),
    EVar2(EVar2Node),
    EIfInt(EIfIntNode),
    EIfError(EIfErrorNode),
    EIfT(EIfTNode),
    EIfTError(EIfTErrorNode),
    EIfF(EIfFNode),
    EPlus(EPlusNode),
    EPlusErrorL(EPlusErrorLNode),
    EPlusBoolR(EPlusBoolRNode),
    ETimes(ETimesNode),
    EMinus(EMinusNode),
    ELt(ELtNode),
    ELtBoolR(ELtBoolRNode),
}
impl RuleNode {
    pub fn new(tokens: &mut Tokens) -> RuleNode {
        let environment = Environment::new(tokens);

        let terms: Terms = Terms::new(tokens);
        let expression = Expression::new(environment.clone(), terms.clone(), terms);
        expression.get_rule(environment)
    }

    pub fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        match self {
            RuleNode::EInt(node) => node.show(w, depth, with_newline),
            RuleNode::EBool(node) => node.show(w, depth, with_newline),
            RuleNode::EVar1(node) => node.show(w, depth, with_newline),
            RuleNode::EVar2(node) => node.show(w, depth, with_newline),
            RuleNode::EIfInt(node) => node.show(w, depth, with_newline),
            RuleNode::EIfError(node) => node.show(w, depth, with_newline),
            RuleNode::EIfT(node) => node.show(w, depth, with_newline),
            RuleNode::EIfTError(node) => node.show(w, depth, with_newline),
            RuleNode::EIfF(node) => node.show(w, depth, with_newline),
            RuleNode::EPlus(node) => node.show(w, depth, with_newline),
            RuleNode::EPlusErrorL(node) => node.show(w, depth, with_newline),
            RuleNode::EPlusBoolR(node) => node.show(w, depth, with_newline),
            RuleNode::ETimes(node) => node.show(w, depth, with_newline),
            RuleNode::EMinus(node) => node.show(w, depth, with_newline),
            RuleNode::ELt(node) => node.show(w, depth, with_newline),
            RuleNode::ELtBoolR(node) => node.show(w, depth, with_newline),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Bool(String),
    Val(String),
    Error,
}
impl Add for Value {
    type Output = Self;

    fn add(self, other: Value) -> Value {
        match self {
            Value::Int(num1) => match other {
                Value::Int(num2) => Value::Int(num1 + num2),
                _ => Value::Error,
            },
            _ => Value::Error,
        }
    }
}
impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Value) -> Value {
        match self {
            Value::Int(num1) => match other {
                Value::Int(num2) => Value::Int(num1 - num2),
                _ => Value::Error,
            },
            _ => Value::Error,
        }
    }
}
impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Value) -> Value {
        match self {
            Value::Int(num1) => match other {
                Value::Int(num2) => Value::Int(num1 * num2),
                _ => Value::Error,
            },
            _ => Value::Error,
        }
    }
}
impl Value {
    fn comp(&self, other: &Value) -> Value {
        match self {
            Value::Int(num1) => match other {
                Value::Int(num2) => {
                    if num1 < num2 {
                        Value::Bool(String::from("true"))
                    } else {
                        Value::Bool(String::from("false"))
                    }
                }
                _ => Value::Error,
            },
            _ => Value::Error,
        }
    }

    fn to_string(self) -> String {
        match self {
            Value::Int(i) => i.to_string(),
            Value::Bool(val) => val,
            Value::Val(val) => val,
            _ => panic!("todo"),
        }
    }
    fn get_num(self) -> i32 {
        match self {
            Value::Int(i) => i,
            _ => panic!("unexpcted"),
        }
    }
    fn get_val(self, env: Environment) -> Value {
        match self {
            Value::Val(val) => match val.as_ref() {
                "x" => match env.x {
                    Some(val) => val,
                    None => panic!("use of undeclared identifier \'x\'"),
                },
                "y" => match env.y {
                    Some(val) => val,
                    None => panic!("use of undeclared identifier \'y\'"),
                },
                _ => panic!("expects x or y"),
            },
            _ => panic!("expects a variable"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub x: Option<Value>,
    pub y: Option<Value>,
}
impl Environment {
    fn new(tokens: &mut Tokens) -> Environment {
        let x = match tokens.peek() {
            Some(token) => match token {
                Token::XEQ => {
                    tokens.pop(); // consume x =
                    let num = tokens.consume_num();
                    Some(Value::Int(num))
                }
                Token::ENV => None,
                _ => panic!("expects x = / |-"),
            },
            None => panic!("expects some tokens"),
        };
        let y = match tokens.peek() {
            Some(token) => match token {
                Token::COMMA => {
                    tokens.pop(); // consume ,
                    tokens.pop(); // consume y =
                    let num = tokens.consume_num();
                    Some(Value::Int(num))
                }
                Token::ENV => None,
                _ => panic!("expects y = / |-"),
            },
            None => panic!("expects some tokens"),
        };
        tokens.pop(); // consume |-
        Environment { x, y }
    }

    fn to_string(self) -> String {
        let s = "".to_string();
        let s = match self.x {
            Some(val) => s + &format!("x = {}", val.to_string()),
            None => s,
        };
        let s = match self.y {
            Some(val) => s + &format!(", y = {}", val.to_string()),
            None => s,
        };
        s.to_string()
    }

    fn get_some_num(&self) -> usize {
        let mut num = 0;
        if let Some(_) = self.x {
            num += 1;
        }
        if let Some(_) = self.y {
            num += 1;
        }
        num
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Value, Terms),
    Bin(String, Box<Expression>, Box<Expression>, Terms),
    If(Box<Expression>, Box<Expression>, Box<Expression>, Terms),
}
impl Expression {
    fn new(env: Environment, mut terms: Terms, origin_terms: Terms) -> Expression {
        if terms.len() == 1 {
            let term = terms.pop().expect("");
            match term {
                Term::Single(_, num) => Expression::Value(Value::Int(num), origin_terms),
                Term::SingleB(_, val) => Expression::Value(Value::Bool(val), origin_terms),
                Term::Val(_, val) => Expression::Value(Value::Val(val), origin_terms),
                Term::Paren(_, terms) => Expression::new(env, terms, origin_terms),
                Term::If(_, if_terms) => Expression::create_if(env, if_terms, origin_terms),
                _ => panic!("todo"),
            }
        } else {
            let (split_position, split_operator) = terms.get_split_position();
            let (former, latter) = terms.get_splitted_terms(split_position);
            println!("================================");
            println!("before: {:?}", former);
            println!("after: {:?}", latter);
            println!("================================");
            let e1 = Expression::new(env.clone(), former.clone(), former);
            let e2 = Expression::new(env, latter.clone(), latter);
            Expression::Bin(split_operator, Box::new(e1), Box::new(e2), origin_terms)
        }
    }

    fn create_if(environment: Environment, if_terms: IfTerms, origin_terms: Terms) -> Expression {
        let condition_exp = Box::new(Expression::new(
            environment.clone(),
            if_terms.condition_terms,
            origin_terms.clone(),
        ));
        let if_exp = Box::new(Expression::new(
            environment.clone(),
            if_terms.then_terms,
            origin_terms.clone(),
        ));
        let else_exp = Box::new(Expression::new(
            environment,
            if_terms.else_terms,
            origin_terms.clone(),
        ));
        Expression::If(condition_exp, if_exp, else_exp, origin_terms)
    }
    fn get_val(&self) -> Value {
        match self.clone() {
            Expression::Value(val, _) => val,
            Expression::Bin(operator, box_ex1, box_ex2, _) => {
                let val1 = box_ex1.get_val();
                let val2 = box_ex2.get_val();
                match operator.as_ref() {
                    "+" => val1 + val2,
                    "-" => val1 - val2,
                    "*" => val1 * val2,
                    "<" => val1.comp(&val2),
                    _ => panic!("TODO"),
                }
            }
            Expression::If(box_condition_exp, box_if_exp, box_else_exp, _) => {
                match box_condition_exp.get_val() {
                    Value::Bool(b) => match b.as_ref() {
                        "true" => box_if_exp.get_val(),
                        "false" => box_else_exp.get_val(),
                        _ => panic!("expects true or false"),
                    },
                    _ => panic!("expects boolean value"),
                }
            }
        }
    }
    fn get_rule(self, env: Environment) -> RuleNode {
        match self {
            Expression::Value(value, _) => match value {
                Value::Int(i) => RuleNode::EInt(EIntNode { i }),
                Value::Bool(val) => RuleNode::EBool(EBoolNode { val }),
                Value::Val(val) => match env.get_some_num() {
                    1 => RuleNode::EVar1(EVar1Node {
                        env,
                        val: Value::Val(val),
                    }),
                    2 => RuleNode::EVar2(EVar2Node {
                        env,
                        val: Value::Val(val),
                    }),
                    _ => panic!("unexpected"),
                },
                _ => panic!("todo"),
            },
            Expression::Bin(operator, box_ex1, box_ex2, _) => {
                let e1 = *box_ex1;
                let e2 = *box_ex2;
                /*
                println!("================================");
                println!("e1: {:?}\t{:?}", e1.get_val(), e1);
                println!("e2: {:?}\t{:?}", e2.get_val(), e2);
                println!("================================");
                */
                match e1.get_val() {
                    Value::Int(_) => match e2.get_val() {
                        Value::Int(_) => match operator.as_ref() {
                            "+" => RuleNode::EPlus(EPlusNode { env, e1, e2 }),
                            "*" => RuleNode::ETimes(ETimesNode { env, e1, e2 }),
                            "-" => RuleNode::EMinus(EMinusNode { env, e1, e2 }),
                            "<" => RuleNode::ELt(ELtNode { env, e1, e2 }),
                            _ => panic!("todo"),
                        },
                        Value::Bool(_) => match operator.as_ref() {
                            "+" => RuleNode::EPlusBoolR(EPlusBoolRNode { env, e1, e2 }),
                            "<" => RuleNode::ELtBoolR(ELtBoolRNode { env, e1, e2 }),
                            _ => panic!("todo"),
                        },
                        _ => panic!("todo"),
                    },
                    Value::Error => match e2.get_val() {
                        Value::Int(_) => match operator.as_ref() {
                            "+" => RuleNode::EPlusErrorL(EPlusErrorLNode { env, e1, e2 }),
                            _ => panic!("todo"),
                        },
                        _ => panic!("todo"),
                    },
                    _ => panic!("todo"),
                }
            }
            Expression::If(box_condition_exp, box_then_exp, box_else_exp, _) => {
                let cond_val = box_condition_exp.get_val();
                let then_val = box_then_exp.get_val();
                let else_val = box_else_exp.get_val();
                /*
                println!("cond_val:  {:?}", cond_val);
                println!("then_val:  {:?}", then_val);
                println!("else_val:  {:?}", else_val);
                */
                match cond_val {
                    Value::Bool(b) => match b.as_ref() {
                        "true" => match then_val {
                            Value::Int(_) => RuleNode::EIfT(EIfTNode {
                                env,
                                condition_exp: *box_condition_exp,
                                then_exp: *box_then_exp,
                                else_exp: *box_else_exp,
                            }),
                            _ => RuleNode::EIfTError(EIfTErrorNode {
                                env,
                                condition_exp: *box_condition_exp,
                                then_exp: *box_then_exp,
                                else_exp: *box_else_exp,
                            }),
                        },
                        "false" => match else_val {
                            Value::Int(_) => RuleNode::EIfF(EIfFNode {
                                env,
                                condition_exp: *box_condition_exp,
                                then_exp: *box_then_exp,
                                else_exp: *box_else_exp,
                            }),
                            _ => panic!("todo"),
                        },
                        _ => panic!("expects true or false"),
                    },
                    _ => RuleNode::EIfInt(EIfIntNode {
                        env,
                        condition_exp: *box_condition_exp,
                        then_exp: *box_then_exp,
                        else_exp: *box_else_exp,
                    }),
                }
            }
        }
    }
    fn to_string(self) -> String {
        let origin_terms = match self {
            Expression::Value(_, terms) => terms,
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
    val: String,
}
impl EBoolNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let nl = if with_newline { "\n" } else { "" };
        write!(
            w,
            "{}{} evalto {} by E-Bool {{}}{}",
            get_depth_space(depth),
            self.val,
            self.val,
            nl
        )
    }
}

#[derive(Debug, Clone)]
pub struct EVar1Node {
    env: Environment,
    val: Value,
}
impl EVar1Node {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let nl = if with_newline { "\n" } else { "" };
        write!(
            w,
            "{}{} |- evalto {} by E-Var2 {{}}{}",
            get_depth_space(depth),
            self.env.clone().to_string(),
            self.val.get_val(self.env).to_string(),
            nl
        )
    }
}

#[derive(Debug, Clone)]
pub struct EVar2Node {
    env: Environment,
    val: Value,
}
impl EVar2Node {
    fn show<W: Write>(mut self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} |- {} evalto {} by E-Var2 {{\n",
            get_depth_space(depth),
            self.env.clone().to_string(),
            self.val.clone().to_string(),
            self.val.clone().get_val(self.env.clone()).to_string(),
        );
        self.env.y = None;
        let _ = write!(
            w,
            "{}{} |- {} evalto {} by E-Var1 {{}}\n",
            get_depth_space(depth + 2),
            self.env.clone().to_string(),
            self.val.clone().to_string(),
            self.val.get_val(self.env).to_string(),
        );
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EIfIntNode {
    env: Environment,
    condition_exp: Expression,
    then_exp: Expression,
    else_exp: Expression,
}
impl EIfIntNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} evalto error by E-IfInt {{\n",
            get_depth_space(depth),
            self.condition_exp.clone().to_string(),
        );
        let condition_premise = self.condition_exp.get_rule(self.env);
        let _ = condition_premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EIfErrorNode {
    env: Environment,
    condition_exp: Expression,
    then_exp: Expression,
    else_exp: Expression,
}
impl EIfErrorNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} evalto error by E-IfError {{\n",
            get_depth_space(depth),
            self.condition_exp.clone().to_string(),
        );
        let condition_premise = self.condition_exp.get_rule(self.env);
        let _ = condition_premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EIfTNode {
    env: Environment,
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
            self.then_exp.get_val().to_string(),
        );
        let condition_premise = self.condition_exp.get_rule(self.env.clone());
        let then_premise = self.then_exp.get_rule(self.env);
        let _ = condition_premise.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = then_premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EIfTErrorNode {
    env: Environment,
    condition_exp: Expression,
    then_exp: Expression,
    else_exp: Expression,
}
impl EIfTErrorNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} evalto error by E-IfTError {{\n",
            get_depth_space(depth),
            self.condition_exp.clone().to_string(),
        );
        let condition_premise = self.condition_exp.get_rule(self.env.clone());
        let then_premise = self.then_exp.get_rule(self.env);
        let _ = condition_premise.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = then_premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EIfFNode {
    env: Environment,
    condition_exp: Expression,
    then_exp: Expression,
    else_exp: Expression,
}
impl EIfFNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} evalto {} by E-IfF {{\n",
            get_depth_space(depth),
            self.condition_exp.clone().to_string(),
            self.else_exp.get_val().to_string(),
        );
        let condition_premise = self.condition_exp.get_rule(self.env.clone());
        let else_premise = self.else_exp.get_rule(self.env);
        let _ = condition_premise.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = else_premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EPlusNode {
    env: Environment,
    e1: Expression,
    e2: Expression,
}
impl EPlusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val().get_num();
        let i2 = self.e2.get_val().get_num();
        let _ = write!(
            w,
            "{}{} + {} evalto {} by E-Plus {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            i1 + i2
        );
        let premise1 = self.e1.get_rule(self.env.clone());
        let premise2 = self.e2.get_rule(self.env);
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
pub struct EPlusErrorLNode {
    env: Environment,
    e1: Expression,
    e2: Expression,
}
impl EPlusErrorLNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} + {} evalto error by E-PlusErrorL {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
        );
        let premise = self.e1.get_rule(self.env);
        let _ = premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EPlusBoolRNode {
    env: Environment,
    e1: Expression,
    e2: Expression,
}
impl EPlusBoolRNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} + {} evalto error by E-PlusBoolR {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
        );
        let premise = self.e2.get_rule(self.env);
        let _ = premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct ETimesNode {
    env: Environment,
    e1: Expression,
    e2: Expression,
}
impl ETimesNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val().get_num();
        let i2 = self.e2.get_val().get_num();
        let _ = write!(
            w,
            "{}{} * {} evalto {} by E-Times {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            i1 * i2
        );
        let premise1 = self.e1.get_rule(self.env.clone());
        let premise2 = self.e2.get_rule(self.env);
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
    env: Environment,
    e1: Expression,
    e2: Expression,
}
impl ELtNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val();
        let i2 = self.e2.get_val();

        let b = match i1.comp(&i2) {
            Value::Bool(val) => val,
            _ => panic!("expects a boolean value"),
        };
        let _ = write!(
            w,
            "{}{} < {} evalto {} by E-Lt {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            b
        );
        let premise1 = self.e1.get_rule(self.env.clone());
        let premise2 = self.e2.get_rule(self.env);
        let _ = premise1.show(w, depth + 2, false);
        let _ = write!(w, ";\n");
        let _ = premise2.show(w, depth + 2, false);
        let _ = write!(w, ";\n");

        let i1 = i1.get_num();
        let i2 = i2.get_num();
        let premise = BLtNode { i1, i2 };
        let _ = premise.show(w, depth + 2, true);

        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct ELtBoolRNode {
    env: Environment,
    e1: Expression,
    e2: Expression,
}
impl ELtBoolRNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let _ = write!(
            w,
            "{}{} < {} evalto error by E-LtBoolR {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
        );
        let premise = self.e2.get_rule(self.env);
        let _ = premise.show(w, depth + 2, true);
        let nl = if with_newline { "\n" } else { "" };
        write!(w, "{}}}{}", get_depth_space(depth), nl)
    }
}

#[derive(Debug, Clone)]
pub struct EMinusNode {
    env: Environment,
    e1: Expression,
    e2: Expression,
}
impl EMinusNode {
    fn show<W: Write>(self, w: &mut W, depth: usize, with_newline: bool) -> io::Result<()> {
        let i1 = self.e1.get_val().get_num();
        let i2 = self.e2.get_val().get_num();
        let _ = write!(
            w,
            "{}{} - {} evalto {} by E-Minus {{\n",
            get_depth_space(depth),
            self.e1.clone().to_string(),
            self.e2.clone().to_string(),
            i1 - i2
        );
        let premise1 = self.e1.get_rule(self.env.clone());
        let premise2 = self.e2.get_rule(self.env);
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
