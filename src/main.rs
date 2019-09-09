extern crate concepts_pl;

use concepts_pl::parser_evalml1::derive;

use std::io;

fn main() {
    let judgement = "if 4 < 5 then 2 + 3 else 8 * 8 evalto 5";
    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
