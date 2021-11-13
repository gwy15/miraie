//! heavily copied from [actix http](https://docs.rs/actix-http/3.0.0-beta.11/src/actix_http/extensions.rs.html#12-16)

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type BoxedAny = Box<dyn Any + Send + Sync>;

/// A type map for request extensions.
///
/// All entries into this map must be owned types (or static references).
#[derive(Default)]
pub struct Extensions {
    map: HashMap<TypeId, BoxedAny>,
}

impl Extensions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(downcast_owned)
    }

    pub fn contains<T: Send + 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut())
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map.remove(&TypeId::of::<T>()).and_then(downcast_owned)
    }
}

fn downcast_owned<T: 'static>(boxed: BoxedAny) -> Option<T> {
    boxed.downcast().ok().map(|boxed| *boxed)
}
