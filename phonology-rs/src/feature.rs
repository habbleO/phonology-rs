use crate::default_feature::DefaultFeature;

#[allow(unused)]
pub const IPA_SYMBOLS: [&'static str; 22] = [
    "p", "b", "t", "d", "k", "g",
    "m", "n", "ŋ",
    "a", "e", "i", "o", "u",
    "f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ"
];

#[allow(unused)]
#[derive(PartialEq, Clone, Debug)]
/// A phonological Feature object. It has a name and an assignment, which can be +, -, or 0 / underspecified / undefined.
/// A [+X] feature is represented by Some(true).
/// A [-X] feature is represented by Some(false).
/// A [0X] feature is represented by None.
pub struct Feature {
    name: String,
    assignment: Option<bool>
}

#[allow(unused)]
impl Feature {

    /// Creates a new feature with the given name and assignment.
    pub fn new<T: Into<String>>(name: T, assignment: Option<bool>) -> Self {
        return Self {name: name.into(), assignment}
    }

    /// Returns a reference to the Feature's name.
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    /// Returns a reference to the Feature's assignment.
    pub fn get_assignment(&self) -> &Option<bool> {
        return &self.assignment;
    }

    pub fn get_default_features() -> Vec<DefaultFeature> {
        // Features are from Hayes, 2011, Introductory Phonology
        // Currently used symbols and features
        // p, b, t, d, k, g, a, e, i, o, u
        // m, n, ŋ
        // f, v, θ, ð, s, z, ʃ, ʒ
        // labial, coronal, dorsal, voice, syllabic,
        // high, low, front, back
        // consonantal, sonorant, nasal
        // continuant, anterior, distributed

        let labial = DefaultFeature::new(
            "labial", 
            vec!["p", "b", "m", "f", "v"], 
            vec!["t", "d", "k", "g", 
                 "a", "e", "i", "o", "u", "n", "ŋ",
                 "θ", "ð", "s", "z", "ʃ", "ʒ"]);
        
        let coronal = DefaultFeature::new(
            "coronal",
            vec!["t", "d", "n",
                 "θ", "ð", "s", "z", "ʃ", "ʒ"],
            vec!["p", "b", "k", "g", 
                 "a", "e", "i", "o", "u", "m", "ŋ",
                 "f", "v"]);

        let anterior = DefaultFeature::new(
            "anterior",
            vec!["t", "d", "n", "θ", "ð", "s", "z"],
            vec!["ʃ", "ʒ"]
        );

        let distributed = DefaultFeature::new(
            "distributed",
            vec!["θ", "ð", "ʃ", "ʒ"],
            vec!["t", "d", "n", "s", "z"]
        );
        
        let dorsal = DefaultFeature::new(
            "dorsal",
            vec!["k", "g", "a", "e", "i", "o", "u", "ŋ"],
            vec!["p", "b", "t", "d", "m", "n",
                 "f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ"]);

        let voice = DefaultFeature::new(
            "voice",
            vec!["b", "d", "g", 
                 "a", "e", "i", "o", "u", "m", "n", "ŋ",
                 "v", "ð", "z", "ʒ"], 
            vec!["p", "t", "k", "f", "θ", "s", "ʃ"]);

        let consonantal = DefaultFeature::new(
            "consonantal",
            vec!["p", "b", "t", "d", "k", "g", "m", "n", "ŋ",
                 "f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ"],
            vec!["a", "e", "i", "o", "u"]
        );

        let sonorant = DefaultFeature::new(
            "sonorant",
            vec!["m", "n", "ŋ", "a", "e", "i", "o", "u"],
            vec!["p", "b", "t", "d", "k", "g",
                 "f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ"]
        );

        let continuant = DefaultFeature::new(
            "continuant",
            vec!["f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ",
                 "a", "e", "i", "o", "u"],
            vec!["p", "b", "t", "d", "k", "g", "m", "n", "ŋ"]
        );

        let nasal = DefaultFeature::new(
            "nasal",
            vec!["m", "n", "ŋ"],
            vec!["p", "b", "t", "d", "k", "g",
                 "a", "e", "i", "o", "u",
                 "f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ"]
        );

        let syllabic = DefaultFeature::new(
            "syllabic",
            vec!["a", "e", "i", "o", "u"],
            vec!["p", "b", "t", "d", "k", "g", "m", "n", "ŋ",
                 "f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ"]);

        let high = DefaultFeature::new(
            "high",
            vec!["i", "u", "k", "g", "ŋ"],
            vec!["a", "e", "o"]
        );

        let low = DefaultFeature::new(
            "low",
            vec!["a"],
            vec!["i", "e", "u", "o", "k", "g", "ŋ"]
        );

        let front = DefaultFeature::new(
            "front",
            vec!["i", "e"],
            vec!["a", "u", "o"]
        );

        let back = DefaultFeature::new(
            "back",
            vec!["u", "o"],
            vec!["a", "i", "e"]
        );

        let all_defaults = vec![
            labial, coronal,
            dorsal, voice,
            consonantal, sonorant,
            continuant, anterior,
            distributed,
            nasal, syllabic,
            high, low, 
            front, back
        ];

        return all_defaults;
    }

    /// Tries to convert a symbol into a feature matrix, using the default
    /// features.
    pub fn to_feature_matrix(symbol: &str) -> Result<Vec<Feature>, String>{

        let all_defaults = Feature::get_default_features();

        let mut result: Vec<Feature> = Vec::new();
        let mut feat_assignment: Option<bool>;

        for feat in all_defaults {

            if feat.get_plus().contains(&symbol) {
                feat_assignment = Some(true);
            } else if feat.get_minus().contains(&symbol) {
                feat_assignment = Some(false);
            } else {
                feat_assignment = None;
            }

            
            let feat_name = feat.get_name();
            
            let new_feature = Feature::new(feat_name, feat_assignment);
            result.push(new_feature);
        }

        if result.iter().all(|x| *x.get_assignment() == None) {
            let err_msg = format!("Could not parse {} as a feature matrix.", symbol);
            return Err(err_msg);
        }

        return Ok(result);
    }
    

}