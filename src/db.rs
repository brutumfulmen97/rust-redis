use std::{collections::HashMap, sync::Mutex};

use bytes::Bytes;

#[derive(Debug)]
pub(crate) struct Db {
    entries: Mutex<HashMap<String, Bytes>>,
}

impl Db {
    pub(crate) fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        self.entries.lock().unwrap().get(key).cloned()
    }

    pub(crate) fn set(&self, key: &str, value: Bytes) -> Option<Bytes> {
        self.entries.lock().unwrap().insert(key.to_string(), value)
    }

    pub(crate) fn del(&self, key: &str) -> bool {
        self.entries.lock().unwrap().remove(key).is_some()
    }

    pub(crate) fn exists(&self, key: &str) -> bool {
        self.entries.lock().unwrap().contains_key(key)
    }
}
