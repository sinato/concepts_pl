mod environment;
mod expression;
mod lexer;
mod nodes;
mod terms;
mod value;

use environment::Environment;
use expression::Expression;
use lexer::Lexer;
use nodes::writer::RuleWriter;
use nodes::RuleNode;

use std::io::{self, Write};

pub fn derive<W: Write>(judgement: &str, w: &mut W) {
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(judgement.to_string());
    dbg!(&tokens);

    let environment = Environment::new(&mut tokens);
    tokens.pop(); // consume |-
    let expression = Expression::new(&mut tokens);
    println!("++++++++++++++++++++++");
    dbg!(expression.clone());
    println!("++++++++++++++++++++++");
    println!("++++++++++++++++++++++");
    let derivation_tree = RuleNode::new(environment, expression);
    println!("++++++++++++++++++++++");
    dbg!(derivation_tree.clone());
    println!("++++++++++++++++++++++");
    let mut writer = RuleWriter::new(w, 0);
    let _ = derivation_tree.show(&mut writer);
    writer.write_nl();
}
