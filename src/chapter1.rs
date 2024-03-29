pub mod lexer;
pub mod parser_compnat1;
pub mod parser_compnat2;
pub mod parser_compnat3;
pub mod parser_evalnatexp;
pub mod parser_nat;
pub mod parser_reducenatexp;
pub mod util;

use crate::chapter1::lexer::Lexer;
use parser_compnat1::RuleNode as CompNat1;
use parser_compnat2::RuleNode as CompNat2;
use parser_compnat3::RuleNode as CompNat3;
use parser_evalnatexp::RuleNode as EvalNatExp;
use parser_nat::RuleNode as Nat;
use parser_reducenatexp::RuleNode as ReduceNatExp;

use std::io::{self, Write};

pub enum DerivationRules {
    Nat,
    CompNat1,
    CompNat2,
    CompNat3,
    EvalNatExp,
    ReduceNatExp,
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
        DerivationRules::CompNat2 => {
            let node = CompNat2::new(&mut tokens);
            node.show(w, 0, true)
        }
        DerivationRules::CompNat3 => {
            let node = CompNat3::new(&mut tokens);
            node.show(w, 0, true)
        }
        DerivationRules::EvalNatExp => {
            let node = EvalNatExp::new(&mut tokens);
            node.show(w, 0, true)
        }
        DerivationRules::ReduceNatExp => {
            let node = ReduceNatExp::new(&mut tokens);
            node.show(w, 0, true)
        }
    }
}
