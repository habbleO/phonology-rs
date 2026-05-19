use crate::feature::Feature;
use crate::segment::Segment;
use crate::word::Word;
use crate::rule::{Rule, RuleIO, Environment};

#[test]
fn test_rule_creation() {
    let a = Segment::from_symbol("a").unwrap();
    let b = Segment::from_symbol("b").unwrap();
    let syllabic = Feature::new("syllabic", Some(true));

    let env = Some(&vec![Environment::FeatureMatrix(vec![syllabic])]);
    let input = RuleIO::Segment(a);
    let output = RuleIO::Segment(b);

    let new_rule = Rule::new(
        "Hi", 
        &input, 
        &output,
        env.clone(),
        env);
    
    assert!(new_rule.get_name() == "Hi");
    assert!(new_rule.get_input() == &input);
    assert!(new_rule.get_output() == &output);
    assert!(new_rule.get_left_env() == env);
    assert!(new_rule.get_right_env() == env);
}

#[test]
fn left_env_match_boundary() {
    let potato = Word::from_str("potato").unwrap();
    let onset_devoicing: Rule;

    let voice = Feature::new("voice", Some(true));
    let min_voice = Feature::new("voice", Some(false));

    // Onset Devoicing
    {
        let input = RuleIO::FeatureMatrix(vec![voice.clone()]);
        let output = RuleIO::FeatureMatrix(vec![min_voice.clone()]);

        let env = Some(&vec![Environment::Boundary]);

        onset_devoicing = Rule::new("Onset Devoicing",
                                    &input, &output, env, None);
    }

    let fails = potato.left_env_match(1, &onset_devoicing);
    assert!(fails == false);

    let success = potato.left_env_match(0, &onset_devoicing);
    assert!(success == true);
}

#[test]
fn left_env_match_features() {
    let potato = Word::from_str("potato").unwrap();
    let intervocalic_voicing: Rule;

    let voice = Feature::new("voice", Some(true));
    let min_voice = Feature::new("voice", Some(false));
    let syllabic = Feature::new("syllabic", Some(true));

    // Intervocalic Voicing
    {
        let input = RuleIO::FeatureMatrix(vec![min_voice.clone()]);
        let output = RuleIO::FeatureMatrix(vec![voice.clone()]);

        let env = Some(&vec![
            Environment::FeatureMatrix(vec![syllabic.clone()])]);

        intervocalic_voicing = Rule::new("Intervocalic Voicing", 
                                         &input, &output, env, env);
    }

    let fails = potato.left_env_match(0, &intervocalic_voicing);
    assert!(fails == false);

    let success = potato.left_env_match(2, &intervocalic_voicing);
    assert!(success == true);
}

#[test]
fn left_env_match_segment() {
    let post_s_raising: Rule;
    let sell = Word::from_str("set").unwrap();

    {
        let e = Segment::from_symbol("e").unwrap();
        let i = Segment::from_symbol("i").unwrap();
        let s = Segment::from_symbol("s").unwrap();

        let input = RuleIO::Segment(e);
        let output = RuleIO::Segment(i);
        let left_env = Some(&vec![Environment::Segment(s)]);

        post_s_raising = Rule::new("Post-S Rasing",
                                   &input, &output, left_env, None);
    }

    let fails = sell.left_env_match(0, &post_s_raising);
    assert!(fails == false);

    let success = sell.left_env_match(1, &post_s_raising);
    assert!(success == true);
}

#[test]
fn left_env_match_multiple() {
    let palatalization: Rule;
    let pis = Word::from_str("pise").unwrap();

    {
        let p = Segment::from_symbol("p").unwrap();
        let syllabic = Feature::new("syllabic", Some(true));

        let s = RuleIO::Segment(Segment::from_symbol("s").unwrap());
        let esh = RuleIO::Segment(Segment::from_symbol("ʃ").unwrap());
        
        let env_0 = Environment::Boundary;
        let env_1 = Environment::Segment(p);
        let env_2 = Environment::FeatureMatrix(vec![syllabic]);

        let left_env = Some(&vec![env_0, env_1, env_2]);

        palatalization = Rule::new("Palatalization",
                                   &s, &esh, left_env, None);
    }

    let fails = pis.left_env_match(0, &palatalization);
    assert!(fails == false);

    let fails = pis.left_env_match(1, &palatalization);
    assert!(fails == false);

    let success = pis.left_env_match(2, &palatalization);
    assert!(success == true);

    let fails = pis.left_env_match(3, &palatalization);
    assert!(fails == false);
}

#[test]
fn right_env_match_boundary() {
    let final_devoicing: Rule;
    let pad = Word::from_str("pad").unwrap();

    {
        let voice = Feature::new("voice", Some(true));
        let min_voice = Feature::new("voice", Some(false));

        let input = RuleIO::FeatureMatrix(vec![voice]);
        let output = RuleIO::FeatureMatrix(vec![min_voice]);

        let env = Some(&vec![Environment::Boundary]);

        final_devoicing = Rule::new("Final Devoicing", 
                                    &input, &output, None, env);                               
    }

    let fails = pad.right_env_match(0, &final_devoicing);
    assert!(fails == false);

    let fails = pad.right_env_match(1, &final_devoicing);
    assert!(fails == false);

    let success = pad.right_env_match(2, &final_devoicing);
    assert!(success == true);
}

#[test]
fn right_env_match_features() {
    let cluster_deletion: Rule;
    let akta = Word::from_str("akta").unwrap();

    {
        let syllabic = Feature::new("syllabic", Some(true));
        let min_syllabic = Feature::new("syllabic", Some(false));
        let input = RuleIO::FeatureMatrix(vec![min_syllabic.clone()]);
        let output = RuleIO::Nothing;

        let left_env = None;
        let right_env = Some(&vec![Environment::FeatureMatrix(vec![syllabic])]);

        cluster_deletion = Rule::new("Cluster Deletion",
                                         &input, &output, left_env, right_env);
    }

    let fails = akta.right_env_match(0, &cluster_deletion);
    assert!(fails == false);

    let fails = akta.right_env_match(1, &cluster_deletion);
    assert!(fails == false);

    let success = akta.right_env_match(2, &cluster_deletion);
    assert!(success == true);

    let fails = akta.right_env_match(3, &cluster_deletion);
    assert!(fails == false);

}

#[test]
fn right_env_match_segment() {
    let palatalization: Rule;
    let sosi = Word::from_str("sosi").unwrap();

    {
        let s = Segment::from_symbol("s").unwrap();
        let esh = Segment::from_symbol("ʃ").unwrap();

        let i = Segment::from_symbol("i").unwrap();

        let input = RuleIO::Segment(s);
        let output = RuleIO::Segment(esh);

        let right_env = Environment::Segment(i);

        palatalization = Rule::new("Palatalization",
                                   &input, &output, None, Some(&vec![right_env]));
    }

    let fails = sosi.right_env_match(0, &palatalization);
    assert!(fails == false);

    let fails = sosi.right_env_match(1, &palatalization);
    assert!(fails == false);

    let success = sosi.right_env_match(2, &palatalization);
    assert!(success == true);

    let fails = sosi.right_env_match(3, &palatalization);
    assert!(fails == false);
}

#[test]
fn right_env_match_multiple() {
    let akta = Word::from_str("akta").unwrap();

    let new_rule = {
        let k = Segment::from_symbol("k").unwrap();
        let t = Segment::from_symbol("t").unwrap();
        let syllabic = Feature::new("syllabic", Some(true));

        let input = RuleIO::Segment(k);
        let output = RuleIO::Nothing;

        let left_env = None;

        let r_env_1 = Environment::Segment(t);
        let r_env_2 = Environment::FeatureMatrix(vec![syllabic]);
        let r_env_3 = Environment::Boundary;

        let r_env = Some(&vec![r_env_1, r_env_2, r_env_3]);

        Rule::new("test", &input, &output, left_env, r_env)
    };

    let fails = akta.right_env_match(0, &new_rule);
    assert!(fails == false);

    let success = akta.right_env_match(1, &new_rule);
    assert!(success == true);

    let fails = akta.right_env_match(2, &new_rule);
    assert!(fails == false);

    let fails = akta.right_env_match(3, &new_rule);
    assert!(fails == false);

    let aktaa = Word::from_str("aktaa").unwrap();

    let fails = aktaa.right_env_match(1, &new_rule);
    assert!(fails == false);
}