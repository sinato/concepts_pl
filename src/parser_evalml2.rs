mod environment;
mod expression;
mod lexer;
mod nodes;
mod terms;
mod value;

use environment::Environment;
use expression::Expression;
use lexer::Lexer;
use nodes::RuleNode;

use std::io::{self, Write};

pub fn derive<W: Write>(judgement: &str, w: &mut W) -> io::Result<()> {
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(judgement.to_string());
    dbg!(&tokens);

    let environment = Environment::new(&mut tokens);
    let expression = Expression::new(&mut tokens);
    let derivation_tree = RuleNode::new(environment, expression);

    derivation_tree.show(w, 0, true)
}
