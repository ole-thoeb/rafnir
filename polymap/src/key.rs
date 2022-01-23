use std::any::{Any, TypeId};
use std::hash::{Hash};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct NormalKey<T> where T: 'static + ?Sized {
    type_id: TypeId,
    tag: u64,
    type_param: PhantomData<T>,
    key: PolyMapKey
}

impl <T> NormalKey<T> {
    pub const fn new(tag: u64) -> Self {
        let type_id = TypeId::of::<T>();
        Self {
            type_id,
            tag,
            type_param: PhantomData {},
            key: PolyMapKey {
                element_type: type_id,
                key_type: TypeId::of::<T>(),
                additional_data: tag
            }
        }
    }
}

impl<T> Key<T> for NormalKey<T> where T: 'static + ?Sized + Any {
    fn key(&self) -> &PolyMapKey {
        &self.key
    }

    fn new_key(&self) -> PolyMapKey {
        self.key.clone()
    }

    fn element_type(&self) -> TypeId {
        self.type_id
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PolyMapKey  {
    pub element_type: TypeId,
    pub key_type: TypeId,
    pub additional_data: u64
}

impl PolyMapKey {
    pub fn new<T: Any + ?Sized, K: Key<T> + Any>(key: &K, additional_data: u64) -> Self {
        Self {
            element_type: key.element_type(),
            key_type: TypeId::of::<K>(),
            additional_data
        }
    }
}

pub trait Key<T> where T : Any + ?Sized {
    fn key(&self) -> &PolyMapKey;
    fn new_key(&self) -> PolyMapKey;
    fn element_type(&self) -> TypeId;
}
