use crate::feature::{Feature, IPA_SYMBOLS};


#[allow(unused)]
use crate::default_feature::*;

#[test]
fn get_feature_name() {
    // Tests Feature::get_name().
    let voiceless = Feature::new("voice", Some(false));
    assert!(voiceless.get_name() == "voice");
}

#[test]
fn get_feature_assignment() {
    // Tests Feature::get_assignment() when assignment is not 0.
    let syllabic = Feature::new("syllabic", Some(true));
    assert_eq!(syllabic.get_assignment(), &Some(true));

    let voiceless = Feature::new("voice", Some(false));
    assert!(voiceless.get_assignment() == &Some(false));   
}

#[test]
fn get_zero_assignment() {
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
fn to_feature_matrix_fail() {
    // Tests a failure state for Feature::to_feature_matrix().
    let failure = Feature::to_feature_matrix("0");
    assert!(failure.is_err());
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
