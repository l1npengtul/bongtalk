/// A String meant to be keyed (translate)
pub struct Keyed {
    key: String,
    alt: Option<String>,
}

impl Keyed {
    pub fn new(key: String, alt: Option<String>) -> Keyed {
        Keyed { key, alt }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn alt(&self) -> Option<&str> {
        self.alt.as_ref().into()
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key
    }

    pub fn set_alt(&mut self, alt: Option<String>) {
        self.alt = alt
    }
}
