extern crate concepts_pl;

use concepts_pl::{derive, DerivationRules};

use std::io;

fn main() {
    let judgement = "(S(S(Z)) + S(S(Z))) * Z evalto Z";
    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, DerivationRules::EvalNatExp, &mut stdout);
    println!("==================================");
}
