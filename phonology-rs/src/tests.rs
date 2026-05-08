use crate::feature::{Feature, IPA_SYMBOLS};
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

#[test]
fn test_all_symbols_to_features() {
    // Tests that every symbol in feature::IPA_FEATURES can be
    // parsed as a feature matrix
    for symbol in IPA_SYMBOLS {
        let success = Feature::to_feature_matrix(symbol);
        assert!(success.is_ok());
    }
}

#[test]
fn test_no_feature_overlap() {
    let mut feature_matrices: Vec<Vec<Feature>> = Vec::new();

    for symbol in IPA_SYMBOLS {
        let matrix = Feature::to_feature_matrix(symbol).unwrap();
        feature_matrices.push(matrix)
    }

    for i in 0..feature_matrices.len() {
        for j in 0..feature_matrices.len() {
            if i != j {
                assert_ne!(feature_matrices[i], feature_matrices[j])
            }
        }
    }
}

#[test]
fn test_failed_feature_overlap() {
    let mut feature_matrices: Vec<Vec<Feature>> = Vec::new();

    for symbol in IPA_SYMBOLS {
        let matrix = Feature::to_feature_matrix(symbol).unwrap();
        feature_matrices.push(matrix)
    }

    let duplicate = Feature::to_feature_matrix("p").unwrap();
    feature_matrices.push(duplicate);

    let mut is_overlap = false;

    for i in 0..feature_matrices.len() {
        for j in 0..feature_matrices.len() {
            if i != j {
                if feature_matrices[i] == feature_matrices[j] {
                    is_overlap = true;
                }
            }
        }
    }

    assert!(is_overlap == true);
}

#[test]
fn test_no_default_feature_overlap() {
    // Tests that none of the DefaultFeatures in 
    // Feature::get_default_features() have overlaps in their
    // definitions. Essentially, that no sound is specified for
    // both [+X] and [-X]

    let def_features = Feature::get_default_features();

    for feat in def_features {
        let plus = feat.get_plus();
        let min = feat.get_minus();

        let overlap: Vec<_> = plus
            .iter()
            .filter(|x| min.contains(x))
            .collect();

        assert!(overlap.len() == 0);


    }
}

#[test]
fn test_no_default_feature_overlap_failure() {
    // Tests the failure state of test_no_default_feature_overlap()
    // Essentially just here to test the logic of the other function
    // So that if there was overlap, I know that it would be found

    let def_features = Feature::get_default_features();

    for feat in def_features {
        let plus = feat.get_plus();
        let plus_2 = feat.get_plus();

        let overlap: Vec<_> = plus
            .iter()
            .filter(|x| plus_2.contains(x))
            .collect();

        assert!(overlap.len() != 0);
    }
}

#[test]
fn test_is_feature() {
    let t = Segment::from_symbol("t").unwrap();
    let some_true = t.is_feature("coronal");
    let some_false = t.is_feature("voice");
    let none = t.is_feature("low");

    assert!(some_true == Some(true));
    assert!(some_false == Some(false));
    assert!(none == None);
}

#[test]
fn test_from_features() {
    let t = Segment::from_symbol("t").unwrap();
    let t_features = t.get_features().clone();
    let t_from_feat = Segment::from_features(&t_features);

    assert!(t_from_feat.is_ok());
    assert!(t_from_feat.unwrap().get_name() == "t");
}

#[test]
pub fn test_update_features() {
    let mut t_d = Segment::from_symbol("t").unwrap();
    let voice = Feature::new("voice", Some(true));
    let _ = t_d.update_features(&vec![voice]);

    assert!(t_d.get_name() == "d");

    let mut d_theta = Segment::from_symbol("d").unwrap();
    let min_voice = Feature::new("voice", Some(false));
    let continu = Feature::new("continuant", Some(true));
    let dist = Feature::new("distributed", Some(true));

    let _ = d_theta.update_features(&vec![min_voice, continu, dist]);

    assert!(d_theta.get_name() == "θ");
}