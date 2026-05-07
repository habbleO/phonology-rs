/// A default set of features.
/// Includes the name of the feature, all sounds that are 
/// [+X], and all sounds that are [-X].
/// Any other character is parsed as undefinded/underspecified.
pub struct DefaultFeature {
    name: String,
    plus: Vec<&'static str>,
    minus: Vec<&'static str>
}

impl DefaultFeature{

    /// Creates a new DefaultFeature with a given name, set of characters
    /// that are [+X], and set of characters that are [-X].
    ///
    /// Todo: Enforce invariant: the intersecion of plus and minus 
    ///       should be 0.
    pub fn new<T: Into<String>>(
        name: T, plus: Vec<&'static str>, minus: Vec<&'static str>) -> Self {
        

        let name: String = name.into();
        return Self{name, plus, minus};
    }

    /// Returns a reference to self.name.
    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    /// Returns a reference to self.plus.
    pub fn get_plus(&self) -> &Vec<&'static str> {
        return &self.plus;
    }
    
    /// Returns a reference to self.minus.
    pub fn get_minus(&self) -> &Vec<&'static str> {
        return &self.minus;
    }
}

