pub struct DefaultFeature {
    name: String,
    plus: Vec<&'static str>,
    minus: Vec<&'static str>
}

impl DefaultFeature{
    pub fn new<T: Into<String>>(
        name: T, plus: Vec<&'static str>, minus: Vec<&'static str>) -> Self {

        let name: String = name.into();
        return Self{name, plus, minus};
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_plus(&self) -> &Vec<&'static str> {
        return &self.plus;
    }

    pub fn get_minus(&self) -> &Vec<&'static str> {
        return &self.minus;
    }
}

