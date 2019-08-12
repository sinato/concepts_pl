pub mod lexer;
pub mod parser;

use crate::lexer::Lexer;
use parser::RuleNode;

use std::io::{self, Write};

pub fn derive<W: Write>(judgement: &str, w: &mut W) -> io::Result<()> {
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(judgement.to_string());
    // dbg!(&tokens);
    let node = RuleNode::new(&mut tokens);
    // dbg!(&node);
    node.show(w, 0)
}
