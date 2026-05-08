use std::vec::Vec;
use crate::feature::Feature;

#[allow(unused)]
#[derive(PartialEq, Clone, Debug)]
/// A segment is an individual phoneme in a word. 
/// It has a name (used to display the underlying form of a word) 
/// and a feature matrix.
pub struct Segment {
    name: String,
    features: Vec<Feature>
}

#[allow(unused)]
impl Segment {
    /// Creates a new Segment from a given name and feature matrix.
    pub fn new<T: Into<String>>(
        name: T, 
        features: Vec<Feature>) -> Self {
        return Self{name: name.into(), features}
    }

    /// Returns a reference to the Segment's name.
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    /// Returns a reference to the Segment's name.
    pub fn get_features(&self) -> &Vec<Feature> {
        return &self.features;
    }

    /// Tries to convert a symbol (&str) into a segment, using
    /// The default features in Feature::to_feature_matrix().
    /// If this succeeds, returns Ok(Segment)
    /// Else, returns Err.
    pub fn from_symbol(symbol: &str) -> Result<Self, String> {
        let features = Feature::to_feature_matrix(symbol)?;

        let new_segment = Segment::new(symbol, features);
        return Ok(new_segment);
    }

    /// Tries to convert a feature matrix into a segment.
    /// If successful, returns Ok(segment).
    pub fn from_features(matrix: &Vec<Feature>) -> Result<Self, String> {
        let symbols = crate::feature::IPA_SYMBOLS.clone();
        let mut symbols: Vec<_> = symbols
            .iter()
            .map(|x| Segment::from_symbol(x).unwrap())
            .collect();

        for feat in matrix {
            symbols = symbols
                .iter()
                .filter(|x| x.is_feature(feat.get_name()) == *feat.get_assignment())
                .map(|x| x.clone())
                .collect();
        }

        if symbols.len() == 0 {
            return Err("There is no matching symbol :(".to_string());
        } else {
            return Ok(symbols[0].clone());
        }
    }

    /// If a Segment is specified for a feature, returns the assigment.
    /// If the Segment is underspecified for that feature, or the
    /// feature is not in the Segment's feature matrix, returns None.
    pub fn is_feature(&self, feat_name: &str) -> Option<bool> {
        let feats = self.get_features();
        let selected: Vec<_> = feats
            .iter()
            .filter(|x| x.get_name() == feat_name)
            .collect();

        if selected.len() == 0 {
            return None;
        } else {
            return *selected[0].get_assignment();
        }
    }
}
