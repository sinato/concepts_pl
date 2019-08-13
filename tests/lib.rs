extern crate concepts_pl;

use concepts_pl::{derive, DerivationRules};

use std::{fs, str};

fn run_test(judgement: &str, derivation_rules: DerivationRules, expect_filepath: &str) {
    let expect: String =
        fs::read_to_string(expect_filepath).expect("something went wrong reading the file.");

    let mut buf = Vec::<u8>::new();
    let _ = derive(judgement, derivation_rules, &mut buf);
    let actual = str::from_utf8(&buf).expect("expects result str");
    println!("{:?}", actual);
    assert_eq!(actual, expect);
}

#[test]
fn test_question1() {
    let judgement = "Z plus Z is Z";
    let expect = "tests/expects/question1";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question2() {
    let judgement = "Z plus S(S(Z)) is S(S(Z))";
    let expect = "tests/expects/question2";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question3() {
    let judgement = "S(S(Z)) plus Z is S(S(Z))";
    let expect = "tests/expects/question3";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question4() {
    let judgement = "S(Z) plus S(S(S(Z))) is S(S(S(S(Z))))";
    let expect = "tests/expects/question4";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question5() {
    let judgement = "Z times S(S(Z)) is Z";
    let expect = "tests/expects/question5";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question6() {
    let judgement = "S(S(Z)) times Z is Z";
    let expect = "tests/expects/question6";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question7() {
    let judgement = "S(S(Z)) times S(Z) is S(S(Z))";
    let expect = "tests/expects/question7";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question8() {
    let judgement = "S(S(Z)) times S(S(Z)) is S(S(S(S(Z))))";
    let expect = "tests/expects/question8";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question9() {
    let judgement = "S(S(Z)) is less than S(S(S(Z)))";
    let expect = "tests/expects/question9";
    run_test(judgement, DerivationRules::CompNat1, expect);
}
