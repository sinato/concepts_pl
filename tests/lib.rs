extern crate concepts_pl;

use concepts_pl::chapter1::{derive, DerivationRules};
use concepts_pl::parser_evalml1::derive as evalml1_derive;

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

fn run_test2(judgement: &str, expect_filepath: &str) {
    let expect: String =
        fs::read_to_string(expect_filepath).expect("something went wrong reading the file.");

    let mut buf = Vec::<u8>::new();
    let _ = evalml1_derive(judgement, &mut buf);
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

#[test]
fn test_question012() {
    let judgement = "S(S(Z)) is less than S(S(S(S(S(Z)))))";
    let expect = "tests/expects/question012";
    run_test(judgement, DerivationRules::CompNat1, expect);
}

#[test]
fn test_question013() {
    let judgement = "S(S(Z)) is less than S(S(S(S(S(Z)))))";
    let expect = "tests/expects/question013";
    run_test(judgement, DerivationRules::CompNat2, expect);
}

#[test]
fn test_question014() {
    let judgement = "S(S(Z)) is less than S(S(S(S(S(Z)))))";
    let expect = "tests/expects/question014";
    run_test(judgement, DerivationRules::CompNat3, expect);
}

#[test]
fn test_question015() {
    let judgement = "Z + S(S(Z)) evalto S(S(Z))";
    let expect = "tests/expects/question015";
    run_test(judgement, DerivationRules::EvalNatExp, expect);
}

#[test]
fn test_question016() {
    let judgement = "S(S(Z)) + Z evalto S(S(Z))";
    let expect = "tests/expects/question016";
    run_test(judgement, DerivationRules::EvalNatExp, expect);
}

#[test]
fn test_question017() {
    let judgement = "S(Z) + S(Z) + S(Z) evalto S(S(S(Z)))";
    let expect = "tests/expects/question017";
    run_test(judgement, DerivationRules::EvalNatExp, expect);
}

#[test]
fn test_question018() {
    let judgement = "S(S(S(Z))) + S(S(Z)) * S(Z) evalto S(S(S(S(S(Z)))))";
    let expect = "tests/expects/question018";
    run_test(judgement, DerivationRules::EvalNatExp, expect);
}

#[test]
fn test_question019() {
    let judgement = "(S(S(Z)) + S(S(Z))) * Z evalto Z";
    let expect = "tests/expects/question019";
    run_test(judgement, DerivationRules::EvalNatExp, expect);
}

#[test]
fn test_question020() {
    let judgement = "Z * (S(S(Z)) + S(S(Z))) evalto Z";
    let expect = "tests/expects/question020";
    run_test(judgement, DerivationRules::EvalNatExp, expect);
}

#[test]
fn test_question021() {
    let judgement = "Z + S(S(Z)) -*-> S(S(Z))";
    let expect = "tests/expects/question021";
    run_test(judgement, DerivationRules::ReduceNatExp, expect);
}

#[test]
fn test_question022() {
    let judgement = "S(Z) * S(Z) + S(Z) * S(Z) -d-> S(Z) + S(Z) * S(Z)";
    let expect = "tests/expects/question022";
    run_test(judgement, DerivationRules::ReduceNatExp, expect);
}

#[test]
fn test_question023() {
    let judgement = "S(Z) * S(Z) + S(Z) * S(Z) ---> S(Z) * S(Z) + S(Z)";
    let expect = "tests/expects/question023";
    run_test(judgement, DerivationRules::ReduceNatExp, expect);
}

#[ignore]
#[test]
fn test_question024() {
    let judgement = "S(Z) * S(Z) + S(Z) * S(Z) -*-> S(S(Z))";
    let expect = "tests/expects/question024";
    run_test(judgement, DerivationRules::ReduceNatExp, expect);
}

#[test]
fn test_question025() {
    let judgement = "3 + 5 evalto 8";
    let expect = "tests/expects/question025";
    run_test2(judgement, expect);
}

#[test]
fn test_question026() {
    let judgement = "8 - 2 - 3 evalto 3";
    let expect = "tests/expects/question026";
    run_test2(judgement, expect);
}
