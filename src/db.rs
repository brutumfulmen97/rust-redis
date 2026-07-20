use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use bytes::Bytes;

#[derive(Debug)]
struct Entry {
    data: Bytes,
    expires_at: Option<Instant>,
}

#[derive(Debug)]
pub(crate) struct Db {
    entries: Mutex<HashMap<String, Entry>>,
}

impl Db {
    pub(crate) fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        let mut map = self.entries.lock().unwrap();
        if let Some(entry) = map.get(key) {
            if entry.is_expired() {
                map.remove(key);
                return None;
            }
        }
        map.get(key).map(|e| e.data.clone())
    }

    pub(crate) fn set(&self, key: &str, value: Bytes, expire: Option<Instant>) {
        self.entries.lock().unwrap().insert(
            key.to_string(),
            Entry {
                data: value,
                expires_at: expire,
            },
        );
    }

    pub(crate) fn del(&self, key: &str) -> bool {
        self.entries.lock().unwrap().remove(key).is_some()
    }

    pub(crate) fn exists(&self, key: &str) -> bool {
        let mut map = self.entries.lock().unwrap();

        if let Some(entry) = map.get(key) {
            if entry.is_expired() {
                map.remove(key);
                return false;
            }
            true
        } else {
            false
        }
    }

    pub(crate) fn expire(&self, key: &str, expires_at: Instant) -> bool {
        let mut map = self.entries.lock().unwrap();
        if let Some(entry) = map.get_mut(key) {
            if entry.is_expired() {
                map.remove(key);
                return false;
            }
            entry.expires_at = Some(expires_at);
            true
        } else {
            false
        }
    }

    pub(crate) fn ttl(&self, key: &str, now: Instant) -> Option<Duration> {
        let mut map = self.entries.lock().unwrap();
        if let Some(entry) = map.get(key) {
            if entry.is_expired() {
                map.remove(key);
                return None;
            }
            entry
                .expires_at
                .map(|e| e.checked_duration_since(now))
                .flatten()
        } else {
            None
        }
    }
}

impl Entry {
    fn is_expired(&self) -> bool {
        self.expires_at
            .map(|e| Instant::now() >= e)
            .unwrap_or(false)
    }
}
