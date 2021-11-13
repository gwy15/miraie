//! copied from actix_web

use crate::msg_framework::FromRequest;
use crate::prelude::Bot;
use std::sync::Arc;

/// Application data.
#[derive(Debug)]
pub struct Data<T: ?Sized>(Arc<T>);

impl<T> Data<T> {
    /// Create new `Data` instance.
    pub fn new(state: T) -> Data<T> {
        Data(Arc::new(state))
    }
}

impl<T: ?Sized> Data<T> {
    /// Get reference to inner app data.
    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    /// Convert to the internal Arc<T>
    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}

impl<T: ?Sized> core::ops::Deref for Data<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

impl<T: ?Sized> Clone for Data<T> {
    fn clone(&self) -> Data<T> {
        Data(self.0.clone())
    }
}

impl<T: ?Sized> From<Arc<T>> for Data<T> {
    fn from(arc: Arc<T>) -> Self {
        Data(arc)
    }
}

impl<T: ?Sized + 'static> FromRequest<Bot> for Data<T> {
    fn from_request(request: &crate::msg_framework::Request<Bot>) -> Option<Self> {
        let app = &request.app;
        let ext = app.extensions.read();
        match ext.get::<Data<T>>() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}
