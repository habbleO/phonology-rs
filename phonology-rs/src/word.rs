use crate::segment::Segment;
use crate::feature::Feature;
use crate::rule::{Rule, RuleIO, Environment};
use std::vec::Vec;

#[allow(unused)]
/// Struct that represents a word, with an underlying form. 
/// The underlying form is a sequence of segments.
pub struct Word {
    underlying_form: Vec<Segment>
}

#[allow(unused)]
impl Word {
    /// Returns a new underlying form given a vector of segments.
    pub fn new(underlying_form: Vec<Segment>) -> Self {
        return Self{underlying_form};
    }

    /// Returns the surface form of the word.
    /// Because phonological rules are currently not implemented, 
    ///  this is just a sequence of each segment's name.
    pub fn get_surface_form(&self) -> String {
        let mut result = String::new();

        for char in &self.underlying_form {
            let to_append = char.get_name();
            result.push_str(to_append);
        }

        return result;
    }

    /// Turns a vector of IPA characters into a word. For this to not error, 
    /// each element in the vector must be parseable as a feature matrix.
    /// If a character is not parseable, returns an error.
    pub fn from_vec(vector: Vec<&str>) -> Result<Self, String> {
        let mut result: Vec<Segment> = Vec::new();

        for item in vector{

            let f_matrix = Feature::to_feature_matrix(item)?;

            let segment = Segment::new(item, f_matrix);
            result.push(segment);
        }
        let new_word = Word::new(result);
        return Ok(new_word);
    }

    /// Tries to parse text as a sequence of IPA characters.
    fn parse_as_vec(text: &str) -> Result<Vec<String>, String> {
        let mut ipa_vec: Vec<String> = Vec::new();

        for symbol in text.chars() {
            let new_symbol = symbol.clone().to_string();
            ipa_vec.push(new_symbol);
        }

        return Ok(ipa_vec);
    }

    /// If possible, converts a &str into a word, where each character is 
    /// parsed as an IPA character. If this fails, returns Err.
    pub fn from_str(text: &str) -> Result<Self, String> {
        let ipa_vec = Self::parse_as_vec(text)?;

        let ipa_vec = ipa_vec.iter().map(|x| x.as_str()).collect();

        return Word::from_vec(ipa_vec);
    }

    /*pub fn left_env_match(&self, pos: i32, rule: &Rule) -> bool {
        let left_env = rule.get_left_env();
        let mut current_pos = pos - 1;
        let mut env_pos = 0;

        match left_env {
            Some(x) => {
                for elem in x.reverse() {
                    match elem {

                    }
                }
            },
            None => {return true;}
        };
    }
    */
}