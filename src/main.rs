extern crate concepts_pl;

use concepts_pl::parser_evalml1::derive;

use std::io;

fn main() {
    let judgement = "1 + true + 2 evalto error";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
