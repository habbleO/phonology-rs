use crate::feature::Feature;
use crate::segment::Segment;
use crate::word::Word;

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

#[test]
fn test_segment_get_name() {
    // Tests Segment::get_name().
    let t = Segment::from_symbol("t").unwrap();

    assert_eq!(t.get_name(), "t")
}

#[test]
fn test_segment_get_features() {
    // Tests Segment::get_features().
    let a = Segment::from_symbol("a").unwrap();

    let nonfront = Feature::new("front", Some(false));
    let nonback = Feature::new("back", Some(false));
    let nonhigh = Feature::new("high", Some(false));
    let low = Feature::new("low", Some(true));

    assert!(a.get_features().contains(&nonfront));
    assert!(a.get_features().contains(&nonback));
    assert!(a.get_features().contains(&nonhigh));
    assert!(a.get_features().contains(&low));
}

#[test]
fn test_segment_from_symbol() {
    // Tests Segment::from_symbol().
    let p = Segment::from_symbol("p").unwrap();
    let p_name = p.get_name();
    let p_features = p.get_features();

    let labial = Feature::new("labial", Some(true));
    let noncoronal = Feature::new("coronal", Some(false));

    assert_eq!(p_name, "p");
    assert!(p_features.contains(&labial));
    assert!(p_features.contains(&noncoronal));
}

#[test]
fn test_segment_from_symbol_fail() {
    // Tests a failure state for Segment::from_symbol.
    let failure = Segment::from_symbol("1");
    
    assert!(failure.is_err());
}

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