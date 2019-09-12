extern crate concepts_pl;

use concepts_pl::parser_evalml2::derive;

use std::io;

fn main() {
    let judgement = "if 3 < 4 then 1 < true else 3 - false evalto error";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
