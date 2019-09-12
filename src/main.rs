extern crate concepts_pl;

use concepts_pl::parser_evalml2::derive;

use std::io;

fn main() {
    let judgement = "x = true, y = 4 |- if x then y + 1 else y - 1 evalto 5";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
