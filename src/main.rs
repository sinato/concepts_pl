extern crate concepts_pl;

use concepts_pl::parser_evalml1::derive;

use std::io;

fn main() {
    let judgement = "3 + (if -23 < -2 * 8 then 8 else 2) + 4 evalto 15";

    println!("judgement ========================");
    println!("{}", judgement);

    println!("derivation tree ==================");
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = derive(judgement, &mut stdout);
    println!("==================================");
}
