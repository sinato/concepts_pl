extern crate concepts_pl;

use concepts_pl::parser_evalml2::derive;

use std::io;

fn main() {
    let judgement = "x = 3, y = 2 |- x evalto 3";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
