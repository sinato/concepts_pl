extern crate concepts_pl;

use concepts_pl::derive;

use std::str;

fn run_test(judgement: &str, expect: &str) {
    let mut buf = Vec::<u8>::new();
    let _ = derive(judgement, &mut buf);
    let actual = str::from_utf8(&buf).expect("expects result str");
    println!("{:?}", actual);
    assert_eq!(actual, expect);
}

#[test]
fn test_question1() {
    let judgement = "Z plus Z is Z";
    let expect = "Z plus Z is Z by P-Zero {}";
    run_test(judgement, expect);
}

#[test]
fn test_question2() {
    let judgement = "Z plus S(S(Z)) is S(S(Z))";
    let expect = "Z plus S(S(Z)) is S(S(Z)) by P-Zero {}";
    run_test(judgement, expect);
}
