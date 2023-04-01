use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use std::sync::Weak;

#[allow(non_snake_case)]
pub fn S(s: &str) -> String {
    s.to_string()
}


pub trait Loader: Send + Sync {
    type Resource;

    fn load(&self, path: &str) -> Result<Self::Resource, String>;
}

pub struct Mgr<L: Loader> {
    loader: L,
    cache: Mutex<HashMap<String, Weak<L::Resource>>>,
}

impl<L: Loader> Mgr<L> {
    pub fn new(loader: L) -> Self {
        Self {
            loader,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_generic<K>(&self, path: &str) -> Result<Handle<K>, String> {
        Err(S("Test"))        
    }

    pub fn get(&self, path: &str) -> Result<Handle<L::Resource>, String> {
        let mut cache = self.cache.lock().unwrap();

        // Try to upgrade the Weak reference to an Arc
        if let Some(resource) = cache.get(path).and_then(|weak| weak.upgrade()) {
            return Ok(Handle::new(resource));
        }

        // Load the resource if not found or no longer in use
        let resource = self.loader.load(path)?;
        let resource = Arc::new(resource);
        cache.insert(path.to_string(), Arc::downgrade(&resource));

        Ok(Handle::new(resource))
    }

    pub fn clean_unused(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.retain(|_key, weak| weak.strong_count() > 0);
    }
}

pub struct Handle<T> {
    inner: Arc<T>,
}

impl<T> Handle<T> {
    fn new(inner: Arc<T>) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> &T {
        &*self.inner
    }
}
