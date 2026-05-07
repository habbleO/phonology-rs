use std::vec::Vec;
use crate::feature::Feature;

#[allow(unused)]
#[derive(PartialEq, Clone, Debug)]
pub struct Segment {
    /// A segment is an individual phoneme in a word. It has a name (used to display the underlying form of a word) and a feature matrix.
    name: String,
    features: Vec<Feature>
}

#[allow(unused)]
impl Segment {
    pub fn new<T: Into<String>>(
        name: T, 
        features: Vec<Feature>) -> Self {
        /// Creates a new Segment from a given name and feature matrix.
        return Self{name: name.into(), features}
    }

    pub fn get_name(&self) -> &str {
        /// Returns a reference to the Segment's name.
        return &self.name;
    }

    pub fn get_features(&self) -> &Vec<Feature> {
        /// Returns a reference to the Segment's feature matrix.
        return &self.features;
    }

    pub fn from_symbol(symbol: &str) -> Result<Self, String> {
        let features = Feature::to_feature_matrix(symbol)?;

        let new_segment = Segment::new(symbol, features);
        return Ok(new_segment);
    }
}
