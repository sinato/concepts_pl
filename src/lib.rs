pub mod lexer;
pub mod parser_compnat1;
pub mod parser_nat;
pub mod util;

use crate::lexer::Lexer;
use parser_compnat1::RuleNode as CompNat1;
use parser_nat::RuleNode as Nat;

use std::io::{self, Write};

pub enum DerivationRules {
    Nat,
    CompNat1,
}

pub fn derive<W: Write>(
    judgement: &str,
    derivation_rules: DerivationRules,
    w: &mut W,
) -> io::Result<()> {
    let lexer = Lexer::new();
    let mut tokens = lexer.lex(judgement.to_string());
    // dbg!(&tokens);
    match derivation_rules {
        DerivationRules::Nat => {
            let node = Nat::new(&mut tokens);
            node.show(w, 0, true)
        }
        DerivationRules::CompNat1 => {
            let node = CompNat1::new(&mut tokens);
            node.show(w, 0, true)
        }
    }
}
