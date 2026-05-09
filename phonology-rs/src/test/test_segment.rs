use crate::feature::Feature;
use crate::segment::Segment;

#[test]
fn get_name() {
    // Tests Segment::get_name().
    let t = Segment::from_symbol("t").unwrap();

    assert_eq!(t.get_name(), "t")
}

#[test]
fn get_features() {
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
fn from_symbol() {
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
fn from_symbol_fail() {
    // Tests a failure state for Segment::from_symbol.
    let failure = Segment::from_symbol("1");
    
    assert!(failure.is_err());
}

#[test]
fn is_feature() {
    let t = Segment::from_symbol("t").unwrap();
    let some_true = t.is_feature("coronal");
    let some_false = t.is_feature("voice");
    let none = t.is_feature("low");

    assert!(some_true == Some(true));
    assert!(some_false == Some(false));
    assert!(none == None);
}

#[test]
fn from_features() {
    let t = Segment::from_symbol("t").unwrap();
    let t_features = t.get_features().clone();
    let t_from_feat = Segment::from_features(&t_features);

    assert!(t_from_feat.is_ok());
    assert!(t_from_feat.unwrap().get_name() == "t");
}

#[test]
pub fn update_features() {
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