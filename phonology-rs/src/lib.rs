#[cfg(test)]
mod tests;

#[cfg(test)]
mod test {
    pub mod test_feature;
    pub mod test_segment;
    pub mod test_word;
}

mod segment;
mod feature;
mod word;
mod default_feature;
mod rule;