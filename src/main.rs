extern crate concepts_pl;

use concepts_pl::{derive, DerivationRules};

use std::io;

fn main() {
    let judgement = "S(Z) + S(Z) + S(Z) evalto S(S(S(Z)))";
    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, DerivationRules::EvalNatExp, &mut stdout);
    println!("==================================");
}
