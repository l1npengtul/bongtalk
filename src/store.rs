use std::{collections::BTreeMap, hash::{Hash, Hasher}};
use ahash::AHasher;
use rhai::{FnCallExpr, Namespace};

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct FnTraversedStore {
    internal: BTreeMap<i64, i32>
}

impl FnTraversedStore {
    pub fn new() -> Self {
        Self
    }

    pub(crate) fn add_traversal(&mut self, fn_call: &FnCallExpr) {
        let mut namespace = (&fn_call.namespace).iter().map(ToString::to_string).collect::<String>();
        namespace = namespace + (&fn_call.name).as_str();

        let mut hasher = AHasher::default();
        namespace.hash(&mut hasher);
        let hash = hasher.finish();

        match self.internal.get_mut(&hash) {
            Some(cnt) => {
                *cnt += 1;
            }
            None => {
                let _ = self.internal.insert(hash, 1);
            }
        }
    }

    pub(crate) fn set_traversal(&mut self, fn_call: &FnCallExpr, new: i32) {
        match self.internal.get_mut(&hash) {
            Some(cnt) => {
                *cnt = new;
            }
            None => {
                let _ = self.internal.insert(hash, new);
            }
        }
    }

    pub fn traversed(&self, fn_call: &FnCallExpr)
}


pub trait RhaiFnCall
