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
fn test_question001() {
    let judgement = "Z plus Z is Z";
    let expect = "tests/expects/question001";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question002() {
    let judgement = "Z plus S(S(Z)) is S(S(Z))";
    let expect = "tests/expects/question002";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question003() {
    let judgement = "S(S(Z)) plus Z is S(S(Z))";
    let expect = "tests/expects/question003";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question004() {
    let judgement = "S(Z) plus S(S(S(Z))) is S(S(S(S(Z))))";
    let expect = "tests/expects/question004";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question005() {
    let judgement = "Z times S(S(Z)) is Z";
    let expect = "tests/expects/question005";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question006() {
    let judgement = "S(S(Z)) times Z is Z";
    let expect = "tests/expects/question006";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question007() {
    let judgement = "S(S(Z)) times S(Z) is S(S(Z))";
    let expect = "tests/expects/question007";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question008() {
    let judgement = "S(S(Z)) times S(S(Z)) is S(S(S(S(Z))))";
    let expect = "tests/expects/question008";
    run_test(judgement, DerivationRules::Nat, expect);
}

#[test]
fn test_question009() {
    let judgement = "S(S(Z)) is less than S(S(S(Z)))";
    let expect = "tests/expects/question009";
    run_test(judgement, DerivationRules::CompNat1, expect);
}

#[test]
fn test_question010() {
    let judgement = "S(S(Z)) is less than S(S(S(Z)))";
    let expect = "tests/expects/question010";
    run_test(judgement, DerivationRules::CompNat2, expect);
}

#[test]
fn test_question011() {
    let judgement = "S(S(Z)) is less than S(S(S(Z)))";
    let expect = "tests/expects/question011";
    run_test(judgement, DerivationRules::CompNat3, expect);
}
