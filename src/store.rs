use ahash::RandomState;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct TraversedStore {
    internal: HashMap<i64, i64, RandomState>,
}

impl TraversedStore {
    pub fn new() -> TraversedStore {
        TraversedStore {
            internal: HashMap::with_hasher(RandomState::new()),
        }
    }

    pub fn get(&self, id: i64) -> i64 {
        self.internal.get(&id).map(|x| *x).unwrap_or(0)
    }

    pub fn add(&mut self, id: i64) {
        match self.internal.get_mut(&id) {
            Some(v) => {
                *v += 1;
            }
            None => {
                self.internal.insert(id, 1);
            }
        }
    }
}
