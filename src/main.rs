extern crate concepts_pl;

use concepts_pl::derive;

use std::io;

fn main() {
    let judgement = "S(S(Z)) plus Z is S(S(Z))";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
