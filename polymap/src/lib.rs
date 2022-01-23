#![feature(const_fn)]
#![feature(const_type_id)]

use std::any::{Any};
use std::collections::HashMap;

pub use crate::key::Key;
use crate::key::PolyMapKey;

mod key;

#[cfg(test)]
mod test;

// inspired by https://lucumr.pocoo.org/2022/1/6/rust-extension-map/

pub struct PolyMap {
    backing_map: HashMap<PolyMapKey<>, Box<dyn Any>>,
}

impl PolyMap {
    pub fn new() -> Self {
        PolyMap { backing_map: HashMap::new() }
    }

    pub fn get<T: Any, K: Key<T>>(&self, key: &K) -> Option<&T> {
        self.backing_map.get(key.key())
            .map(|v| (**v).downcast_ref::<T>().expect("Correct type"))
    }

    pub fn insert<T: Any, K: Key<T>>(&mut self, key: &K, value: T) -> Option<Box<T>> {
        self.backing_map.insert(key.new_key(), Box::new(value))
            .map(|v| v.downcast::<T>().expect("Correct type"))
    }
}
