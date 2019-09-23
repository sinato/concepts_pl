extern crate concepts_pl;

use concepts_pl::parser_evalml3::derive;

use std::io;

fn main() {
    let judgement = "|- let sm = fun f -> f 3 + f 4 in sm (fun x -> x * x) evalto 25";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
