use smartstring::{LazyCompact, SmartString};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct TraversedStore {
    internal: BTreeMap<SmartString<LazyCompact>, i64>,
}

impl TraversedStore {
    pub fn new() -> TraversedStore {
        TraversedStore {
            internal: BTreeMap::new(),
        }
    }

    pub fn get(&self, id: &SmartString<LazyCompact>) -> i64 {
        self.internal.get(id).map(|x| *x).unwrap_or(0)
    }

    pub fn reset(&mut self, id: &SmartString<LazyCompact>) {
        if let Some(cnt) = self.internal.get_mut(id) {
            *cnt = 0;
        }
    }

    pub fn add(&mut self, id: &SmartString<LazyCompact>) {
        match self.internal.get_mut(&id) {
            Some(v) => {
                *v += 1;
            }
            None => {
                self.internal.insert(id.clone(), 1);
            }
        }
    }

    pub fn remove(&mut self, id: &SmartString<LazyCompact>) {
        self.remove(id)
    }

    pub fn clear(&mut self) {
        self.clear()
    }
}
