use crate::feature::{Feature, IPA_SYMBOLS};
use crate::segment::Segment;
use crate::word::Word;
use crate::rule::{Rule, RuleIO, Environment};




#[test]
fn test_rule_creation() {
    let a = Segment::from_symbol("a").unwrap();
    let b = Segment::from_symbol("b").unwrap();
    let syllabic = Feature::new("syllabic", Some(true));

    let env = Some(vec![Environment::FeatureMatrix(vec![syllabic])]);
    let input = RuleIO::Segment(a);
    let output = RuleIO::Segment(b);

    let new_rule = Rule::new(
        "Hi", 
        &input, 
        &output,
        &env,
        &env);
    
    assert!(new_rule.get_name() == "Hi");
    assert!(new_rule.get_input() == &input);
    assert!(new_rule.get_output() == &output);
    assert!(new_rule.get_left_env() == &env);
    assert!(new_rule.get_right_env() == &env);
    
}