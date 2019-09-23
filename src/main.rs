extern crate concepts_pl;

use concepts_pl::parser_evalml3::derive;

use std::io;

fn main() {
    let judgement = "|- let sq = fun x -> x * x in sq 3 + sq 4 evalto 25";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
