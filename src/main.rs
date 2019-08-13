extern crate concepts_pl;

use concepts_pl::{derive, DerivationRules};

use std::io;

fn main() {
    let judgement = "S(S(Z)) is less than S(S(S(S(S(Z)))))";
    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, DerivationRules::CompNat2, &mut stdout);
    println!("==================================");
}
