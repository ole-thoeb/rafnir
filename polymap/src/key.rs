use std::any::{Any, TypeId};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

// derive(Hash, PartialEq, Eq) does not work because the generated impls require that T also
// implements that trait. But that is nonsense.
#[derive(Debug, Clone)]
pub struct Key<T> where T: 'static + ?Sized {
    type_id: TypeId,
    tag: &'static str,
    type_param: PhantomData<T>,
}

impl<T: ?Sized> Hash for Key<T> {
    fn hash<H: Hasher>(&self, mut state: &mut H) {
        self.type_id.hash(&mut state);
        self.tag.hash(&mut state);
    }
}

impl<T: ?Sized> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id && self.tag == other.tag
    }
}

impl<T: ?Sized> Eq for Key<T> {}

impl<T> Key<T> where T: 'static + ?Sized + Any {
    pub const fn new(tag: &'static str) -> Self {
        Key {
            type_id: TypeId::of::<T>(),
            tag,
            type_param: PhantomData {},
        }
    }

    pub(crate) fn as_dyn_as_any(&self) -> &Key<dyn Any> {
        let pointer = self as *const Self;
        unsafe {
            &*(pointer as *const Key<dyn Any>)
        }
    }

    pub(crate) fn to_dyn_as_any(self) -> Key<dyn Any> {
        Key {
            type_id: self.type_id,
            tag: self.tag,
            type_param: PhantomData {},
        }
    }
}
