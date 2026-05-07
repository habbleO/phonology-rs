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
}
