use rhai::ImmutableString;
use smartstring::{LazyCompact, SmartString};

/// A String meant to be keyed (translate)
#[derive(Clone, Debug, Default, PartialOrd, PartialEq)]
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

    pub fn as_ref(&self) -> KeyedRef {
        KeyedRef {
            key: &self.key,
            alt: self.alt.as_ref().into(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct KeyedRef<'a> {
    key: &'a str,
    alt: Option<&'a str>,
}

impl<'a> KeyedRef<'a> {
    pub fn key(&self) -> &str {
        self.key
    }

    pub fn alt(&self) -> Option<&str> {
        self.alt
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub(crate) enum KeyedOrRaw {
    Keyed(Keyed),
    Raw(ImmutableString),
}
