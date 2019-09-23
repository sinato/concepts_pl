mod lexer;
mod nodes;
mod terms;

use lexer::Lexer;
use nodes::RuleNode;

use std::io::{self, Write};

pub fn derive<W: Write>(judgement: &str, w: &mut W) {
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(judgement.to_string());
    dbg!(&tokens);
    let derivation_tree = RuleNode::new(&mut tokens);
    let _ = derivation_tree.show(w, 0, true);
}
