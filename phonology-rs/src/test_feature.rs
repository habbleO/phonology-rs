use crate::feature::{Feature, IPA_SYMBOLS};
use crate::segment::Segment;
use crate::word::Word;
use crate::rule::{Rule, RuleIO, Environment};
use crate::default_feature::*;

#[allow(unused)]
use crate::default_feature::*;

#[test]
fn test_feature_get_feature_name() {
    // Tests Feature::get_name().
    let voiceless = Feature::new("voice", Some(false));
    assert!(voiceless.get_name() == "voice");
}

#[test]
fn test_feature_get_feature_assignment() {
    // Tests Feature::get_assignment() when assignment is not 0.
    let syllabic = Feature::new("syllabic", Some(true));
    assert_eq!(syllabic.get_assignment(), &Some(true));

    let voiceless = Feature::new("voice", Some(false));
    assert!(voiceless.get_assignment() == &Some(false));   
}

#[test]
fn test_feature_get_zero_assignment() {
    // Tests Feature::get_assignment when assignment is 0.
    let zero_voice = Feature::new("voice", None);
    assert!(zero_voice.get_assignment() == &None);
}

#[test]
fn test_feature_to_feature_matrix() {
    // Tests Feature::to_feature_matrix().
    let a = Feature::to_feature_matrix("a").unwrap();
    let nonfront = Feature::new("front", Some(false));
    let nonback = Feature::new("back", Some(false));
    let nonhigh = Feature::new("high", Some(false));
    let low = Feature::new("low", Some(true));

    assert!(a.contains(&nonfront));
    assert!(a.contains(&nonback));
    assert!(a.contains(&nonhigh));
    assert!(a.contains(&low));
}

#[test]
fn test_feature_to_feature_matrix_fail() {
    // Tests a failure state for Feature::to_feature_matrix().
    let failure = Feature::to_feature_matrix("0");
    assert!(failure.is_err());
}