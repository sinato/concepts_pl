extern crate concepts_pl;

use concepts_pl::parser_evalml3::derive;

use std::io;

fn main() {
    let judgement = "|- let y = 2 in func x -> x + y evalto (y=2)[fun x -> x + y]";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
