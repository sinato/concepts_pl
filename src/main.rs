extern crate concepts_pl;

use concepts_pl::parser_evalml2::derive;

use std::io;

fn main() {
    let judgement = "|- let x = 3 * 3 in let y = 4 * x in x + y evalto 45";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
