use crate::segment::Segment;
use crate::feature::Feature;
use std::vec::Vec;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    name: String,
    input: RuleIO,
    output: RuleIO,
    left_env: Option<Vec<Environment>>,
    right_env: Option<Vec<Environment>>
}

#[allow(unused)]
impl Rule {
    pub fn new<T: Into<String>>(
        name: T, 
        input: &RuleIO, 
        output: &RuleIO,
        left_env: &Option<Vec<Environment>>,
        right_env: &Option<Vec<Environment>>) -> Self {
        
        let name: String = name.into();
        let input = input.clone();
        let output = output.clone();
        let left_env = left_env.clone();
        let right_env = right_env.clone();
        return Self {name, input, output, left_env, right_env};
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_input(&self) -> &RuleIO {
        return &self.input;
    }

    pub fn get_output(&self) -> &RuleIO {
        return &self.output;
    }

    pub fn get_left_env(&self) -> &Option<Vec<Environment>> {
        return &self.left_env;
    }

    pub fn get_right_env(&self) -> &Option<Vec<Environment>> {
        return &self.right_env;
    }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum RuleIO {
    Segment(Segment),
    FeatureMatrix(Vec<Feature>),
    Nothing
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Segment(Segment),
    FeatureMatrix(Vec<Feature>),
    Boundary
}