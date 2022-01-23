#![feature(const_fn)]
#![feature(const_type_id)]

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub use crate::key::Key;

mod key;

#[cfg(test)]
mod test;

// inspired by https://lucumr.pocoo.org/2022/1/6/rust-extension-map/

pub struct PolyMap {
    backing_map: HashMap<Key<dyn Any>, Box<dyn Any>>,
}

impl PolyMap {
    pub fn new() -> Self {
        PolyMap { backing_map: HashMap::new() }
    }

    pub fn get<T: Any>(&self, key: &Key<T>) -> Option<&T> {
        self.backing_map.get(key.as_dyn_as_any())
            .map(|v| (**v).downcast_ref::<T>().expect("Correct type"))
    }

    pub fn insert<T>(&mut self, key: Key<T>, value: T) -> Option<Box<T>> {
        self.backing_map.insert(key.to_dyn_as_any(), Box::new(value))
            .map(|v| v.downcast::<T>().expect("Correct type"))
    }
}


// pub trait AsAny: Any {
//     fn as_any(&self) -> &dyn Any;
//     fn as_any_mut(&mut self) -> &mut dyn Any;
//
//     fn to_any(self: Box<Self>) -> Box<dyn Any>;
// }
//
// impl<T> AsAny for T where T: 'static + Any {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//
//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
//
//     fn to_any(self: Box<Self>) -> Box<dyn Any> {
//         self
//     }
// }