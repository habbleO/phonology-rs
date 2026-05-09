use crate::feature::{Feature, IPA_SYMBOLS};
use crate::segment::Segment;
use crate::word::Word;
use crate::rule::{Rule, RuleIO, Environment};

#[test]
fn test_word_get_word_surface_form() {
    // Tests Word::get_surface_form().
    let p = Segment::from_symbol("p").unwrap();
    let o = Segment::from_symbol("o").unwrap();
    let t = Segment::from_symbol("t").unwrap();

    let pot_ur = vec![p, o, t];
    let pot = Word::new(pot_ur);

    let pot_sr = pot.get_surface_form();

    assert!(pot_sr == String::from("pot"));
}

#[test]
fn test_word_from_vec() {
    // Tests Word::from_vec().
    // Tests parsing a vector of IPA characters as a Word.
    let word_vec = vec!["p", "o", "p"];
    let word_struct = Word::from_vec(word_vec);
    let word_sr = word_struct.unwrap().get_surface_form();
    assert_eq!(&word_sr, "pop");
}

#[test]
fn test_word_from_vec_fail() {
    // Tests a failure state for Word::from_vec().
    let failure = Word::from_vec(vec!["0", "1", "2"]);
    assert!(failure.is_err());
}

#[test]
fn test_word_from_str() {
    // Tests Word::from_str().
    let new_word = Word::from_str("pot").unwrap();
    let new_word_sr = new_word.get_surface_form();

    assert_eq!(new_word_sr, "pot");
}

#[test]
fn test_word_from_str_fail() {
    // Tests a failure state for Word::from_str().
    let failure = Word::from_str("000");
    assert!(failure.is_err());
}

#[test]
fn test_all_symbols_to_features() {
    // Tests that every symbol in feature::IPA_FEATURES can be
    // parsed as a feature matrix
    for symbol in IPA_SYMBOLS {
        let success = Feature::to_feature_matrix(symbol);
        assert!(success.is_ok());
    }
}