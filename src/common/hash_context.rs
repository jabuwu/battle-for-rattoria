use std::{
    any::type_name,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashContext(u64);

impl HashContext {
    pub fn new<T: Hash>(value: T) -> Self {
        let mut hasher = DefaultHasher::new();
        type_name::<T>().hash(&mut hasher);
        value.hash(&mut hasher);
        Self(hasher.finish())
    }

    pub fn is(&self, context: impl Hash) -> bool {
        *self == HashContext::new(context)
    }
}

impl<T: Hash> From<T> for HashContext {
    fn from(value: T) -> Self {
        HashContext::new(value)
    }
}
