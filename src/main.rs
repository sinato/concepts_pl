extern crate concepts_pl;

use concepts_pl::parser_evalml1::derive;

use std::io;

fn main() {
    let judgement = "8 - 2 - 3 evalto 3";
    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
